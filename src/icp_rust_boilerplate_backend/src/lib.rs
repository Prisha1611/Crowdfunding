#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Campaign {
    id: u64,
    title: String,
    description: String,
    goal_amount: u64,
    raised_amount: u64,
    created_at: u64,
    updated_at: Option<u64>,
    creator: String,
}

impl Storable for Campaign {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Campaign {
    const MAX_SIZE: u32 = 1024;  // Reduced to fit within 1024 bytes
    const IS_FIXED_SIZE: bool = false;
}

// Helper function to validate the size before insertion
fn validate_size(campaign: &Campaign) -> Result<(), Error> {
    let encoded_campaign = Encode!(campaign).unwrap();
    if encoded_campaign.len() as u32 > Campaign::MAX_SIZE {
        return Err(Error::SizeExceeded {
            msg: format!("Campaign data exceeds the max size of {}", Campaign::MAX_SIZE),
        });
    }
    Ok(())
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static STORAGE: RefCell<StableBTreeMap<u64, Campaign, Memory>> = 
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        ));
}

// Define Payloads
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct CampaignPayload {
    title: String,
    description: String,
    goal_amount: u64,
    creator: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct DonationPayload {
    campaign_id: u64,
    amount: u64,
}

// Campaign Functions
#[ic_cdk::query]
fn get_campaign(id: u64) -> Result<Campaign, Error> {
    match _get_campaign(&id) {
        Some(campaign) => Ok(campaign),
        None => Err(Error::NotFound {
            msg: format!("Campaign with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn create_campaign(payload: CampaignPayload) -> Result<Campaign, Error> {
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("cannot increment id counter");

    let campaign = Campaign {
        id,
        title: payload.title,
        description: payload.description,
        goal_amount: payload.goal_amount,
        raised_amount: 0,
        created_at: time(),
        updated_at: None,
        creator: payload.creator,
    };

    // Validate the size of the campaign before inserting
    validate_size(&campaign)?;

    _insert_campaign(&campaign);
    Ok(campaign)
}

#[ic_cdk::update]
fn update_campaign(id: u64, payload: CampaignPayload) -> Result<Campaign, Error> {
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut campaign) => {
            campaign.title = payload.title;
            campaign.description = payload.description;
            campaign.goal_amount = payload.goal_amount;
            campaign.updated_at = Some(time());

            // Validate the size of the updated campaign
            validate_size(&campaign)?;

            _insert_campaign(&campaign);
            Ok(campaign)
        }
        None => Err(Error::NotFound {
            msg: format!("Campaign with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn delete_campaign(id: u64) -> Result<Campaign, Error> {
    match STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(campaign) => Ok(campaign),
        None => Err(Error::NotFound {
            msg: format!("Campaign with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn donate_to_campaign(payload: DonationPayload) -> Result<String, Error> {
    STORAGE.with(|service| {
        let mut storage = service.borrow_mut();

        // Try to retrieve the campaign by its ID
        if let Some(mut campaign) = storage.get(&payload.campaign_id) {
            // Log the current state of the campaign and donation
            ic_cdk::println!(
                "Campaign ID: {}, Goal: {}, Raised: {}, Donation: {}",
                payload.campaign_id,
                campaign.goal_amount,
                campaign.raised_amount,
                payload.amount
            );

            // Check if the donation does not exceed the campaign goal
            if campaign.raised_amount + payload.amount <= campaign.goal_amount {
                campaign.raised_amount += payload.amount;

                // Validate the size before reinsertion
                validate_size(&campaign)?;

                // Reinsert the updated campaign
                storage.insert(payload.campaign_id, campaign);

                Ok(format!("Donation of {} accepted to campaign {}", payload.amount, payload.campaign_id))
            } else {
                // Log the failure scenario
                ic_cdk::println!(
                    "Donation exceeds goal: Campaign ID: {}, Raised: {}, Donation: {}, Goal: {}",
                    payload.campaign_id,
                    campaign.raised_amount,
                    payload.amount,
                    campaign.goal_amount
                );

                Err(Error::NotEnoughFunds {
                    msg: "Donation exceeds the campaign goal".to_string(),
                })
            }
        } else {
            Err(Error::NotFound {
                msg: format!("Campaign with id={} not found", payload.campaign_id),
            })
        }
    })
}


// Helper Methods
fn _insert_campaign(campaign: &Campaign) {
    STORAGE.with(|service| service.borrow_mut().insert(campaign.id, campaign.clone()));
}

fn _get_campaign(id: &u64) -> Option<Campaign> {
    STORAGE.with(|service| service.borrow().get(id))
}

// Error Handling
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    NotEnoughFunds { msg: String },
    SizeExceeded { msg: String },  // New error for size validation
}

// Candid Export
ic_cdk::export_candid!();
