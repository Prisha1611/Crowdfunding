# Crowdfunding platform
This is a decentralized crowdfunding management platform. The platform allows users to create campaigns, make donations, and update or delete campaigns. It leverages IC canisters to store and manage campaign data.

Features
Create Campaigns: Users can create new crowdfunding campaigns with a title, description, fundraising goal, and creator information.
Update Campaigns: Existing campaigns can be updated with new details such as title, description, and goal amount.
Delete Campaigns: Campaigns can be deleted.
Donate to Campaigns: Users can make donations to campaigns. The system checks if the donation amount exceeds the campaign goal.


Methods:
create_campaign: Creates a new campaign.
update_campaign: Updates an existing campaign.
delete_campaign: Deletes a campaign.
donate_to_campaign: Donates to a specified campaign.

Prerequisites
Rust and the dfx (for deploying to the Internet Computer).
Access to the Internet Computer's Canister Development environment.

Installation
Clone the repository:
git clone (https://github.com/Prisha1611/crowdfunding.git)
cd crowdfunding-management-platform
Start the Internet Computer:
dfx start --background
Deploy the Canisters:
dfx deploy

