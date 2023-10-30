library;
pub mod structs;

use structs::*;
use i64::I64;

abi Market {

    // ## 1. Debug Functionality
    // This functionality is exclusively utilized in local tests to evaluate interest accrual. It works by advancing the timestamp within the contract at specific intervals defined as `DEBUG_STEP`.
    
    #[storage(read, write)]
    fn debug_increment_timestamp();


    // ## 2. User Account Initialization
    // This is an administrative function that allows the system's governor to set up new collateral assets. Each collateral assets may have different characteristics.
    
    #[storage(write, read)]
    fn add_collateral_asset(configuration: CollateralConfiguration); 

    #[storage(read, write)]
    fn pause_collateral_asset(asset_id: b256); 
    
    #[storage(read, write)]
    fn resume_collateral_asset(asset_id: b256); 

    #[storage(read)]
    fn get_collateral_configurations() -> Vec<CollateralConfiguration>; //todo add this data to indexer


    // ## 3. Supply and Withdraw collateral
    // Users can deposit and withdraw collateral  (e.g., BTC, ETH, UNI...). This collateral is necessary to borrow.
    
    #[payable, storage(read, write)]
    fn supply_collateral(); //Payment is required: any collateral asset

    #[storage(read, write)]
    fn withdraw_collateral(asset: b256, amount: u64);
    
    #[storage(read)]
    fn get_user_collateral(address: Address, asset_id: b256) -> u64;  //todo add this data to indexer

    #[storage(read)]
    fn totals_collateral(asset_id: b256) -> u64;  //todo add this data to indexer


    // ## 4. Supply and Withdraw base
    // If the user has enough collateral, `withdraw_base` performs the borrow function
    
    #[payable, storage(read, write)]
    fn supply_base(); //Payment is required: base asset (USDC)

    #[storage(read, write)]
    fn withdraw_base(amount: u64);

    #[storage(read)]
    fn get_user_supply_borrow(account: Address) -> (u64, u64);  //todo add this data to indexer

    #[storage(read)]
    fn available_to_borrow(account: Address) -> u64; //todo add this data to indexer


    // ## 4. Absorbation
    // Liquidates the user if there is insufficient collateral for the borrowing. 

    #[storage(read, write)]
    fn absorb(accounts: Vec<Address>);

    #[storage(read)]
    fn is_liquidatable(account: Address) -> bool;


    // ## 5. Buying collateral
    // Purchase of collateral for liquidated positions

    //Function for buying the collateral of a liquidated user
    #[payable, storage(read)]
    fn buy_collateral(asset_id: b256, min_amount: u64, recipient: Address); //Payment is required: base asset (USDC)
    
    #[storage(read)]
    fn collateral_value_to_sell(asset_id: b256, collateral_amount: u64) -> u64;
    
    // A function that counts the collateral purchase rate of the user from our protocol (how much collateral the user gets for redeeming one dollar of debt to be liquidated)
    #[storage(read)]
    fn quote_collateral(asset_id: b256, base_amount: u64) -> u64;


    // ## 6. Reward stuff
    // todo add description and test reward stuff

    fn get_reward_token_asset_id() -> b256;

    //function for claiming reward tokens by admin. `reward_token` is set in merket config
    fn withdraw_reward_token(recipient: Address, amount: u64);

    #[storage(read, write)]
    fn get_reward_owed(account: Address) -> u64;

    #[storage(read, write)]
    fn claim();


    // ## 7. Reserves stuff
    // todo add description and test reserves stuff

    // Gets the total amount of protocol reserves of the base asset
    #[storage(read)]
    fn get_reserves() -> I64; //todo add this data to indexer

    // Withdraws base token reserves if called by the governor
    #[storage(read)]
    fn withdraw_reserves(to: Address, amount: u64);

    #[storage(read)]
    fn get_collateral_reserves(asset_id: b256) -> I64;
    
    // ## 8. Pause settings
    // This is an administrative function that allows the system's governor to pause supply, withdraw, absorb, buy and claim
    
    #[storage(write, read)]
    fn pause(config: PauseConfiguration);

    // ## 9. Getters
    // Data that will be useful in the calculations

    fn get_configuration() -> MarketConfiguration;

    #[storage(read)]
    fn get_market_basics() -> MarketBasics;

    #[storage(read)]
    fn get_user_basic(account: Address) -> UserBasic;

    #[storage(read)]
    fn get_utilization() -> u64;

    fn balance_of(asset: b256) -> u64;

    
    // ## 10. Supply and Borrow rates
    // Formulas to help calculate supply and borrow rates

    fn get_supply_rate(utilization: u64) -> u64;  //todo add this data to indexer

    fn get_borrow_rate(utilization: u64) -> u64;  //todo add this data to indexer

}
