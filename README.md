
```markdown
# Crowdfunding Platform

This is a decentralized crowdfunding management platform built on the Internet Computer (IC). The platform allows users to create campaigns, make donations, and manage campaigns. Campaign data is stored and managed using IC canisters, ensuring scalability and reliability.

## Features

- Create Campaigns: Users can create new crowdfunding campaigns with details like title, description, fundraising goal, and creator information.
- Update Campaigns: Existing campaigns can be updated with new details, such as title, description, and goal amount.
- Delete Campaigns: Users can delete campaigns when necessary.
- Donate to Campaigns: Users can make donations to specified campaigns, and the system ensures that the donation amount does not exceed the campaign goal.

## Methods

The following methods are available in the platform:

- create_campaign: Creates a new campaign with the provided details.
- update_campaign: Updates an existing campaign with new information.
- delete_campaign: Deletes a campaign from the platform.
- donate_to_campaign: Allows users to donate to a specified campaign, ensuring the donation amount doesn't exceed the campaign's goal.

## Prerequisites

- **Rust**: The platform is developed using Rust. Ensure that you have Rust installed on your system.
- **dfx**: The Internet Computer's Canister Development tool, required for deploying canisters to the network.
- **Internet Computer Canister Development Environment**: Access to the IC development environment for deployment.

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/Prisha1611/crowdfunding.git
cd crowdfunding-management-platform
```

### 2. Start the Internet Computer

```bash
dfx start --background
```

This will start the IC local environment in the background for local development and testing.

### 3. Deploy the Canisters

```bash
dfx deploy
```

This will deploy your canisters to the local Internet Computer environment.

## Usage

After deployment, you can interact with the canisters using `dfx` commands or by integrating them into a frontend or backend system. The platform supports basic CRUD operations for crowdfunding campaigns, making it suitable for creating decentralized fundraising platforms.

## Contributing

Feel free to fork the repository and submit pull requests. All contributions are welcome!


