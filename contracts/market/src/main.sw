// SPDX-License-Identifier: BUSL-1.1
contract;
/**
 *
 * @title Swaylend's Market Contract
 * @notice An efficient monolithic money market protocol
 * @author Composability labs
 */
mod helpers;
mod structs;

use helpers::*;
use structs::*;
// use signed_integers::i64::I64;
use i64::*;
use oracle_abi::*;
use std::auth::{AuthError,msg_sender};
use std::call_frames::{contract_id,msg_asset_id};
use std::constants::ZERO_B256;
use std::context::{msg_amount,this_balance};
use std::hash::sha256;
use std::logging::log;
use std::revert::require;
use std::token::transfer_to_address;
use std::u128::U128;
use std::storage::storage_vec::*;
use std::vec::Vec;
use std::token::mint_to_address;

const SCALE_18: u64 = 1_000_000_000_000_000_000; // 1e18

configurable {
    MARKET_CONFIGURATION: Option<MarketConfiguration> = Option::None,
    DEBUG_STEP: Option<u64> = Option::None,
}

storage {
    asset_configurations: StorageMap<AssetId, AssetConfig> = StorageMap {},
    asset_configurations_keys: StorageVec<AssetId> = StorageVec {},
    pause_config: PauseConfiguration = PauseConfiguration::default(),
    totals_collateral: StorageMap<AssetId, u64> = StorageMap {},
    user_collateral: StorageMap<(Address, AssetId), u64> = StorageMap {},
    user_basic: StorageMap<Address, UserBasic> = StorageMap {},
    market_basic: MarketBasics = MarketBasics::default(),
    
    debug_timestamp: u64 = 0,
}

abi Market {

    // ## 1. Debug Functionality
    // This functionality is exclusively utilized in local tests to evaluate interest accrual. It works by advancing the timestamp within the contract at specific intervals defined as `DEBUG_STEP`.
    
    #[storage(read, write)]
    fn debug_increment_timestamp();


    // ## 2. User Account Initialization
    // This is an administrative function that allows the system's governor to set up new collateral assets. Each collateral assets may have different characteristics.
    
    #[storage(write, read)]
    fn add_asset_collateral(asset_config: AssetConfig); 

    #[storage(read, write)]
    fn pause_asset_collateral(asset_id: AssetId); 
    
    #[storage(read, write)]
    fn resume_asset_collateral(asset_id: AssetId); 

    #[storage(read)]
    fn get_asset_configurations() -> Vec<AssetConfig>;


    // ## 3. Supply and Withdraw collateral
    // Users can deposit and withdraw collateral  (e.g., BTC, ETH, UNI...). This collateral is necessary to borrow.
    
    #[payable, storage(read, write)]
    fn supply_collateral(); //Payment is required: any collateral asset

    #[storage(read, write)]
    fn withdraw_collateral(asset: AssetId, amount: u64);
    
    #[storage(read)]
    fn get_user_collateral(address: Address, asset_id: AssetId) -> u64;

    #[storage(read)]
    fn totals_collateral(asset_id: AssetId) -> u64;


    // ## 4. Supply and Withdraw base
    // If the user has enough collateral, `withdraw_base` performs the borrow function
    
    #[payable, storage(read, write)]
    fn supply_base(); //Payment is required: base asset (USDC)

    #[storage(read, write)]
    fn withdraw_base(amount: u64);

    #[storage(read)]
    fn get_user_supply_borrow(account: Address) -> (u64, u64);

    #[storage(read)]
    fn available_to_borrow(account: Address) -> u64;


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
    fn buy_collateral(asset_id: AssetId, min_amount: u64, recipient: Address); //Payment is required: base asset (USDC)
    
    #[storage(read)]
    fn collateral_value_to_sell(asset_id: AssetId, collateral_amount: u64) -> u64;
    
    // A function that counts the collateral purchase rate of the user from our protocol (how much collateral the user gets for redeeming one dollar of debt to be liquidated)
    #[storage(read)]
    fn quote_collateral(asset_id: AssetId, base_amount: u64) -> u64;


    // ## 6. Reward stuff
    // todo add description and test reward stuff

    fn get_reward_token_asset_id() -> AssetId;

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
    fn get_reserves() -> I64;

    // Withdraws base token reserves if called by the governor
    #[storage(read)]
    fn withdraw_reserves(to: Address, amount: u64);

    #[storage(read)]
    fn get_collateral_reserves(asset_id: AssetId) -> I64;
    
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

    fn balance_of(asset: AssetId) -> u64;

    
    // ## 10. Supply and Borrow rates
    // Formulas to help calculate supply and borrow rates

    fn get_supply_rate(utilization: u64) -> u64;

    fn get_borrow_rate(utilization: u64) -> u64;

}

impl Market for Contract {

    // ## 1. Debug Functionality

    #[storage(read, write)]
    fn debug_increment_timestamp() {
        require(DEBUG_STEP.is_some(), Error::DebuggingDisabled);
        storage.debug_timestamp.write(storage.debug_timestamp.try_read().unwrap_or(0) + DEBUG_STEP.unwrap());
    }


    // ## 2. User Account Initialization
    
    #[storage(write, read)]
     fn add_asset_collateral(asset_config: AssetConfig){ 
        require(msg_sender_address() == MARKET_CONFIGURATION.unwrap().governor, Error::Unauthorized);
        require(storage.asset_configurations.get(asset_config.asset_id).try_read().is_none(), Error::UnknownAsset);
        storage.asset_configurations.insert(asset_config.asset_id, asset_config);
        storage.asset_configurations_keys.push(asset_config.asset_id);
    }

    #[storage(read, write)]
    fn pause_asset_collateral(asset_id: AssetId){
        require(msg_sender_address() == MARKET_CONFIGURATION.unwrap().governor, Error::Unauthorized);
        let mut config = storage.asset_configurations.get(asset_id).try_read().unwrap();
        config.paused = true;
        storage.asset_configurations.insert(asset_id, config);
    }

    #[storage(read, write)]
    fn resume_asset_collateral(asset_id: AssetId){
         require(msg_sender_address() == MARKET_CONFIGURATION.unwrap().governor, Error::Unauthorized);
        let mut config = storage.asset_configurations.get(asset_id).try_read().unwrap();
        config.paused = false;
        storage.asset_configurations.insert(asset_id, config);
    }

    #[storage(read)]
    fn get_asset_configurations() -> Vec<AssetConfig> {
        let mut result = Vec::new();
        let mut index = 0;
        while index < storage.asset_configurations_keys.len() {
            let asset_config = storage.asset_configurations.get(storage.asset_configurations_keys.get(index).unwrap().read()).read();
            result.push(asset_config);
            index += 1;
        }
        result
    }


    // ## 3. Supply and Withdraw collateral
    
    #[payable, storage(read, write)]
    fn supply_collateral() { 
        let caller = msg_sender_address();
        require(!storage.pause_config.supply_paused.try_read().unwrap_or(false), Error::Paused);
    
        let amount = msg_amount();
        require(amount > 0, Error::InvalidPayment);
    
        let asset_id = msg_asset_id();
        let asset_config = storage.asset_configurations.get(asset_id).read();
        require(!asset_config.paused, Error::Paused);

        let mut total_supply_asset = storage.totals_collateral.get(asset_id).try_read().unwrap_or(0);
        total_supply_asset += amount;
        require(total_supply_asset <= asset_config.supply_cap, Error::SupplyCapExceeded);
    
        let user_collateral = storage.user_collateral.get((caller, asset_id)).try_read().unwrap_or(0);
        let user_collateral_new = user_collateral + amount;
    
        storage.totals_collateral.insert(asset_id, total_supply_asset);
        storage.user_collateral.insert((caller, asset_id), user_collateral_new);
    }

    #[storage(read, write)]
    fn withdraw_collateral(asset: AssetId, amount: u64) {
        let caller = msg_sender_address();
        let src_collateral = storage.user_collateral.get((caller, asset)).try_read().unwrap_or(0);
        let src_collateral_new = src_collateral - amount;

        let new_total_supply_asset = storage.totals_collateral.get(asset).try_read().unwrap_or(0) - amount;
        storage.totals_collateral.insert(asset, new_total_supply_asset);
        storage.user_collateral.insert((caller, asset), src_collateral_new);

        // Note: no accrue interest, BorrowCF < LiquidationCF covers small changes
        require(is_borrow_collateralized(caller), Error::NotCollateralized);

        transfer_to_address(caller, asset, amount);
    }
    
    #[storage(read)]
    fn get_user_collateral(address: Address, asset_id: AssetId) -> u64 {
        storage.user_collateral.get((address, asset_id)).try_read().unwrap_or(0)
    }

    #[storage(read)]
    fn totals_collateral(asset_id: AssetId) -> u64 {
        storage.totals_collateral.get(asset_id).try_read().unwrap_or(0)
    }


    // ## 4. Supply and Withdraw base
    
    #[payable, storage(read, write)]
    fn supply_base() { // @Payment base_token
        require(!storage.pause_config.supply_paused.try_read().unwrap_or(false), Error::Paused);
        let caller = msg_sender_address();
        let config = MARKET_CONFIGURATION.unwrap();
        let amount = msg_amount();
    
        require(amount > 0, Error::InvalidPayment);
        require(msg_asset_id() == config.base_token, Error::InvalidPayment);
    
        accrue_internal();
    
        let dst_user = storage.user_basic.get(caller).try_read().unwrap_or(UserBasic::default());
        let dst_principal = dst_user.principal;
        let dst_present_value = present_value(dst_principal);
    
        let dst_balance = dst_present_value + I64::from(amount);
        let dst_principal_new = principal_value(dst_balance);
    
        let (repay_amount, supply_amount) = repay_and_supply_amount(dst_principal, dst_principal_new);
        let mut market_basic = storage.market_basic.try_read().unwrap_or(MarketBasics::default());
        market_basic.total_supply_base += supply_amount;
        market_basic.total_borrow_base -= repay_amount;
        storage.market_basic.write(market_basic);
        update_base_principal(caller, dst_user, dst_principal_new);
    
        // if supply_amount > 0 {
        //     mint_to_address(supply_amount, caller);
        // }
    }

    #[storage(read, write)]
    fn withdraw_base(amount: u64) {
        require(!storage.pause_config.withdraw_paused.try_read().unwrap_or(false), Error::Paused);
        require(amount > 0, Error::InvalidPayment); //fixme strange error name
        accrue_internal();

        let config = MARKET_CONFIGURATION.unwrap();
        let caller = msg_sender_address();
        let src_user = storage.user_basic.get(caller).try_read().unwrap_or(UserBasic::default());
        let src_principal = src_user.principal;
        let src_balance = present_value(src_principal) - I64::from(amount);
        let src_principal_new = principal_value(src_balance);

        let (withdraw_amount, borrow_amount) = withdraw_and_borrow_amount(src_principal, src_principal_new);
        let mut market_basic = storage.market_basic.try_read().unwrap_or(MarketBasics::default());
        market_basic.total_supply_base -= withdraw_amount; 
        market_basic.total_borrow_base += borrow_amount;
        storage.market_basic.write(market_basic);

        update_base_principal(caller, src_user, src_principal_new);

        if src_balance < I64::from(0) {
            require(src_balance.flip().into() >= config.base_borrow_min, Error::BorrowTooSmall);
            require(is_borrow_collateralized(caller), Error::NotCollateralized);
        }

        transfer_to_address(caller, config.base_token, amount);

        // burn(withdraw_amount); 
    }

    #[storage(read)]
    fn get_user_supply_borrow(account: Address) -> (u64, u64) {
        let principal_value = storage.user_basic.get(account).try_read().unwrap_or(UserBasic::default()).principal;
        let last_accrual_time = storage.market_basic.try_read().unwrap_or(MarketBasics::default()).last_accrual_time;
        let (supply_index_, borrow_index_) = accrued_interest_indices(timestamp(), last_accrual_time);   // decimals (18, 18)
        if principal_value >= I64::new() {
            let supply = present_value_supply(supply_index_, principal_value.into());
            (supply, 0)
        } else {
            let borrow = present_value_borrow(borrow_index_, principal_value.flip().into());
            (0, borrow)
        }
    }

    #[storage(read)]
    fn available_to_borrow(account: Address) -> u64 {
        let mut borrow_limit = U128::new();
        let mut index = 0;
        let config = MARKET_CONFIGURATION.unwrap();
        while index < storage.asset_configurations_keys.len() { 
            let asset_config = storage.asset_configurations.get(storage.asset_configurations_keys.get(index).unwrap().read()).read();

            let balance = storage.user_collateral.get((account, asset_config.asset_id)).try_read().unwrap_or(0);
            let balance = U128::from_u64(balance);
            let price = get_price(asset_config.asset_id, asset_config.price_feed); // decimals 9
            let price = U128::from_u64(price);

            let collateral_factor = U128::from_u64(asset_config.borrow_collateral_factor); // decimals 4
            let scale = U128::from_u64(10.pow(asset_config.decimals + 4 + 9 - config.base_token_decimals));

            borrow_limit += balance * price * collateral_factor / scale; //base_token_decimals
            index = index + 1;
        };
        borrow_limit.as_u64().unwrap()
    }


    // ## 4. Absorbation

    #[storage(read, write)]
    fn absorb(accounts: Vec<Address>) {
        require(!storage.pause_config.absorb_paused.try_read().unwrap_or(false), Error::Paused);
        accrue_internal();
        let mut index = 0;
        while index < accounts.len() {
            absorb_internal(accounts.get(index).unwrap());
            index += 1;
        }
    }

    #[storage(read)]
    fn is_liquidatable(account: Address) -> bool {
        is_liquidatable_internal(account)
    }


    // ## 5. Buying collateral

    #[payable, storage(read)]
    fn buy_collateral(asset_id: AssetId, min_amount: u64, recipient: Address){
        require(!storage.pause_config.buy_paused.try_read().unwrap_or(false), Error::Paused);
        let config = MARKET_CONFIGURATION.unwrap();
        let base_amount = msg_amount();
        require(msg_asset_id() == config.base_token && base_amount > 0, Error::InvalidPayment);

        let reserves = get_reserves_internal();
        require(reserves < I64::from(0) || reserves.into() < config.target_reserves, Error::NotForSale);

        // Note: Re-entrancy can skip the reserves check above on a second buyCollateral call.
        let reserves = get_collateral_reserves_internal(asset_id);
        let collateral_amount = quote_collateral_internal(asset_id, base_amount);
        require(collateral_amount >= min_amount, Error::TooMuchSlippage);
        require(I64::from(collateral_amount) <= reserves, Error::InsufficientReserves);

        // Note: Pre-transfer hook can re-enter buyCollateral with a stale collateral ERC20 balance.
        //  Assets should not be listed which allow re-entry from pre-transfer now, as too much collateral could be bought.
        //  This is also a problem if quoteCollateral derives its discount from the collateral ERC20 balance.
        transfer_to_address(recipient, asset_id, collateral_amount);
    }
    
   #[storage(read)]
    fn collateral_value_to_sell(asset_id: AssetId, collateral_amount: u64) -> u64 { // asset decimals
        let config = MARKET_CONFIGURATION.unwrap();
        let asset_config = storage.asset_configurations.get(asset_id).read();
        let asset_price = get_price(asset_id, asset_config.price_feed); // decimals 9
        let base_price = get_price(config.base_token, config.base_token_price_feed);// decimals 9
        let base_price = U128::from_u64(base_price);
        let store_front_price_factor = config.store_front_price_factor; // decimals 4
        let liquidation_penalty = asset_config.liquidation_penalty; // decimals 4
        // Store front discount is derived from the collateral asset's liquidation_penalty and store_front_price_factor
        let scale4 = 10.pow(4);
        let discount_factor = store_front_price_factor * (scale4 - liquidation_penalty) / scale4; // decimals 4
        let asset_price_discounted = U128::from_u64(asset_price * (scale4 - discount_factor) / scale4); // decimals 9
        // of collateral assets
        let base_scale = U128::from_u64(10.pow(config.base_token_decimals));
        let asset_scale = U128::from_u64(10.pow(asset_config.decimals));
        (U128::from_u64(collateral_amount) * asset_price_discounted * base_scale / asset_scale / base_price).as_u64().unwrap()
    }
    
    #[storage(read)]
    fn quote_collateral(asset_id: AssetId, base_amount: u64) -> u64 {
        quote_collateral_internal(asset_id, base_amount)
    }


    // ## 6. Reward stuff

    fn get_reward_token_asset_id() -> AssetId {
        sha256((contract_id(), ZERO_B256))
    }

    fn withdraw_reward_token(recipient: Address, amount: u64) {
        require(msg_sender_address() == MARKET_CONFIGURATION.unwrap().governor, Error::NotPermitted);
        mint_to_address(recipient, ZERO_B256, amount);
    }

    #[storage(read, write)]
    fn get_reward_owed(account: Address) -> u64 {
        accrue_internal();

        let basic = storage.user_basic.get(account).try_read().unwrap_or(UserBasic::default());
        update_base_principal(account, basic, basic.principal);

        let claimed = storage.user_basic.get(account).try_read().unwrap_or(UserBasic::default()).reward_claimed;
        let accrued = storage.user_basic.get(account).try_read().unwrap_or(UserBasic::default()).base_tracking_accrued;

        if accrued > claimed {
            accrued - claimed
        } else {
            0
        }
    }

    #[storage(read, write)]
    fn claim() {
        require(!storage.pause_config.claim_paused.try_read().unwrap_or(false), Error::Paused);
        let caller = msg_sender_address();

        accrue_internal();

        let basic = storage.user_basic.get(caller).try_read().unwrap_or(UserBasic::default());
        update_base_principal(caller, basic, basic.principal);
        let mut basic = storage.user_basic.get(caller).try_read().unwrap_or(UserBasic::default());
        let claimed = basic.reward_claimed;
        let accrued = basic.base_tracking_accrued;

        if accrued > claimed {
            basic.reward_claimed = accrued;
            storage.user_basic.insert(caller, basic);

            let amount = accrued - claimed;
            mint_to_address(caller, ZERO_B256, amount);
        }
    }


    // ## 7. Reserves stuff

    #[storage(read)]
    fn get_reserves() -> I64{
        get_reserves_internal()
    }

    #[storage(read)]
    fn withdraw_reserves(to: Address, amount: u64){
        let config = MARKET_CONFIGURATION.unwrap();
        let sender = msg_sender_address();

        require(sender == config.governor, Error::Unauthorized);
        let reserves = get_reserves_internal();
        require(reserves >= I64::from(0) && amount <= reserves.into(), Error::InsufficientReserves);

        transfer_to_address(to, config.base_token, amount)
    }

    #[storage(read)]
    fn get_collateral_reserves(asset_id: AssetId) -> I64{
        get_collateral_reserves_internal(asset_id)
    }


    // ## 8. Pause settings
    
    #[storage(write, read)]
    fn pause(pause_config: PauseConfiguration) {
        let sender = msg_sender_address();
        let config = MARKET_CONFIGURATION.unwrap();
        require(sender == config.governor || sender == config.pause_guardian, Error::Unauthorized);
        storage.pause_config.write(pause_config);
    }

    // ## 9. Getters

    fn get_configuration() -> MarketConfiguration {
        MARKET_CONFIGURATION.unwrap()
    }

    #[storage(read)]
    fn get_market_basics() -> MarketBasics {
        storage.market_basic.try_read().unwrap_or(MarketBasics::default())
    }

    #[storage(read)]
    fn get_user_basic(account: Address) -> UserBasic {
        storage.user_basic.get(account).try_read().unwrap_or(UserBasic::default())
    }

    #[storage(read)]
    fn get_utilization() -> u64 {
        get_utilization_internal()
    }

    fn balance_of(asset: AssetId) -> u64 {
        this_balance(asset)
    }

    
    // ## 10. Supply and Borrow rates

    fn get_supply_rate(utilization: u64) -> u64 {
        get_supply_rate_internal(utilization).as_u64().unwrap()
    }

    fn get_borrow_rate(utilization: u64) -> u64 {
        get_borrow_rate_internal(utilization).as_u64().unwrap()
    }

}


#[storage(read)]
fn timestamp() -> u64 {
    if DEBUG_STEP.is_none() {
        storage.debug_timestamp.try_read().unwrap_or(0)
    } else {
        std::block::timestamp()
    }
}

fn msg_sender_address() -> Address {
    match msg_sender().unwrap() {
        Identity::Address(identity) => identity,
        _ => revert(0),
    }
}

fn get_price(asset: AssetId, price_feed: ContractId) -> u64 {
    let res = abi(Oracle, price_feed.value).get_price(asset);
    res.price
}

pub fn present_value_supply(base_supply_index_: u64, principal_value_: u64) -> u64 { // -> base_asset_decimals
    let res = U128::from_u64(principal_value_) * U128::from_u64(base_supply_index_) / U128::from_u64(SCALE_18);
    res.as_u64().unwrap()
}

pub fn present_value_borrow(base_borrow_index_: u64, principal_value_: u64) -> u64 { // -> base_asset_decimals
    let res = U128::from_u64(principal_value_) * U128::from_u64(base_borrow_index_) / U128::from_u64(SCALE_18);
    res.as_u64().unwrap()
}

pub fn principal_value_supply(base_supply_index_: u64, present_value_: u64) -> u64 { // -> base_asset_decimals
    let res = U128::from_u64(present_value_) * U128::from_u64(SCALE_18) / U128::from_u64(base_supply_index_);
    res.as_u64().unwrap()
}

pub fn principal_value_borrow(base_borrow_index_: u64, present_value_: u64) -> u64 { // -> base_asset_decimals
    let res = (U128::from_u64(present_value_) * U128::from_u64(SCALE_18) + U128::from_u64(base_borrow_index_ - 1)) / U128::from_u64(base_borrow_index_);
    res.as_u64().unwrap()
}

#[storage(read)]
fn present_value(principal_value_: I64) -> I64 { // -> base_asset_decimals
    let market_basic = storage.market_basic.try_read().unwrap_or(MarketBasics::default());
    if principal_value_ >= I64::from(0) {
        let present_value_u64 = present_value_supply(market_basic.base_supply_index, principal_value_.into());
        I64::from(present_value_u64)
    } else {
        let present_value_u64 = present_value_borrow(market_basic.base_borrow_index, principal_value_.flip().into());
        I64::from(present_value_u64).flip()
    }
}

#[storage(read)]
fn principal_value(present_value_: I64) -> I64 { // -> base_asset_decimals
    let market_basic = storage.market_basic.try_read().unwrap_or(MarketBasics::default());
    if present_value_ >= I64::from(0) {
        let principal_value_u64 = principal_value_supply(market_basic.base_supply_index, present_value_.into());
        I64::from(principal_value_u64)
    } else {
        let principal_value_u64 = principal_value_borrow(market_basic.base_borrow_index, present_value_.flip().into());
        I64::from(principal_value_u64).flip()
    }
}

// @Callable get_utilization() -> u64
#[storage(read)]
fn get_utilization_internal() -> u64 { // -> decimals 18
    let market_basic = storage.market_basic.try_read().unwrap_or(MarketBasics::default());
    let total_supply_ = present_value_supply(market_basic.base_supply_index, market_basic.total_supply_base); // decimals 6
    let total_borrow_ = present_value_borrow(market_basic.base_borrow_index, market_basic.total_borrow_base); // decimals 6
    if total_supply_ == 0 {
        0
    } else {
        let res = U128::from_u64(total_borrow_) * U128::from_u64(SCALE_18) / U128::from_u64(total_supply_);
        res.as_u64().unwrap()
    }
}

// @Callable get_supply_rate(utilization: u64) -> U128
fn get_supply_rate_internal(utilization: u64) -> U128 { // -> decimals 18
    let utilization = U128::from_u64(utilization);
    let config = MARKET_CONFIGURATION.unwrap();
    let kink_ = U128::from_u64(config.kink); // decimals 18
    let interest_rate_slope_low = U128::from_u64(config.supply_per_second_interest_rate_slope_low);// decimals 18
    let interest_rate_slope_high = U128::from_u64(config.supply_per_second_interest_rate_slope_high);// decimals 18
    let scale = U128::from_u64(SCALE_18);
    if utilization <= kink_ {
        interest_rate_slope_low * utilization / scale
    } else {
        (interest_rate_slope_low * kink_ + interest_rate_slope_high * (utilization - kink_)) / scale
    }
}

// @Callable get_borrow_rate(utilization: u64) -> U128
fn get_borrow_rate_internal(utilization: u64) -> U128 { // -> decimals 18
    let utilization = U128::from_u64(utilization); // decimals 18
    let config = MARKET_CONFIGURATION.unwrap();
    let kink_ = U128::from_u64(config.kink); // decimals 18
    let interest_rate_base = U128::from_u64(config.borrow_per_second_interest_rate_base); // decimals 18
    let interest_rate_slope_low = U128::from_u64(config.borrow_per_second_interest_rate_slope_low); // decimals 18
    let interest_rate_slope_high = U128::from_u64(config.borrow_per_second_interest_rate_slope_high); // decimals 18
    let scale = U128::from_u64(SCALE_18);
    if utilization <= kink_ {
        interest_rate_base + interest_rate_slope_low * utilization / scale
    } else {
        interest_rate_base + (interest_rate_slope_low * kink_ + interest_rate_slope_high * (utilization - kink_)) / scale
    }
}

// calculation of the updated value base_supply/borrow_index
#[storage(read)]
fn accrued_interest_indices(now: u64, last_accrual_time: u64) -> (u64, u64) { // -> decimals (18, 18)
    if last_accrual_time == 0 {
        return (SCALE_18, SCALE_18)
    }
    let time_elapsed = now - last_accrual_time;
    let market_basic = storage.market_basic.try_read().unwrap_or(MarketBasics::default());
    let mut base_supply_index_ = U128::from_u64(market_basic.base_supply_index); // decimals 18
    let mut base_borrow_index_ = U128::from_u64(market_basic.base_borrow_index); // decimals 18
    if time_elapsed > 0 {
        let utilization = get_utilization_internal();  // decimals 18
        let supply_rate = get_supply_rate_internal(utilization); // decimals 18
        let borrow_rate = get_borrow_rate_internal(utilization); // decimals 18
        let scale = U128::from_u64(SCALE_18);
        base_supply_index_ += base_supply_index_ * supply_rate / scale * U128::from_u64(time_elapsed);
        base_borrow_index_ += base_borrow_index_ * borrow_rate / scale * U128::from_u64(time_elapsed);
    }
    let base_supply_index_ = base_supply_index_.as_u64().unwrap();
    let base_borrow_index_ = base_borrow_index_.as_u64().unwrap();
    return (base_supply_index_, base_borrow_index_);
}

// Checks that the dollar value of the user's collateral multiplied by borrow_collateral_factor is greater than the (planned) loan amount.
#[storage(read)]
fn is_borrow_collateralized(account: Address) -> bool {
    let principal_value_ = storage.user_basic.get(account).try_read().unwrap_or(UserBasic::default()).principal; // decimals base_asset_decimal
    if principal_value_ >= I64::new() {
        return true
    };

    let config = MARKET_CONFIGURATION.unwrap();
    let present_value_ = present_value(principal_value_.flip()); // decimals base_asset_decimals
    let mut borrow_limit = U128::new();
    let mut index = 0;
    while index < storage.asset_configurations_keys.len() {
        let asset_config = storage.asset_configurations.get(storage.asset_configurations_keys.get(index).unwrap().read()).read();

        let balance = storage.user_collateral.get((account, asset_config.asset_id)).try_read().unwrap_or(0); // decimals asset_config.decimals
        let balance = U128::from_u64(balance);

        let price = get_price(asset_config.asset_id, asset_config.price_feed); // decimals 9
        let price = U128::from_u64(price);

        let collateral_factor = U128::from_u64(asset_config.borrow_collateral_factor); // decimals 4
        let scale = U128::from_u64(10.pow(asset_config.decimals));

        borrow_limit += balance * price * collateral_factor / U128::from_u64(10000) / scale; //decimals 9
        index += 1;
    }

    let base_token_price = get_price(config.base_token, config.base_token_price_feed); //decimals 9
    let scale = U128::from_u64(10.pow(9));
    let borrow_amount = U128::from_u64(present_value_.into()) * U128::from_u64(base_token_price) / scale; // decimals 9
    borrow_limit >= borrow_amount
}
// @Callable is_liquidatable(account: Address) -> bool
// checks that the dollar value of the user's collateral multiplied
// by liquidate_collateral_factor is less than the loan amount. 
#[storage(read)]
fn is_liquidatable_internal(account: Address) -> bool {
    let principal_value_ = storage.user_basic.get(account).try_read().unwrap_or(UserBasic::default()).principal; // decimals base_asset_decimal
    if principal_value_ >= I64::new() {
        return false
    };

    let config = MARKET_CONFIGURATION.unwrap();
    let present_value_ = U128::from_u64(present_value(principal_value_.flip()).into()); // decimals base_asset_decimals
    let mut liquidation_treshold = U128::new();
    let mut index = 0;
    while index < storage.asset_configurations_keys.len() { 

        let asset_config = storage.asset_configurations.get(storage.asset_configurations_keys.get(index).unwrap().read()).read();

        let balance = storage.user_collateral.get((account, asset_config.asset_id)).try_read().unwrap_or(0); // decimals asset_config.decimals
        let balance = U128::from_u64(balance);
        let price = get_price(asset_config.asset_id, asset_config.price_feed); // decimals 9
        let price = U128::from_u64(price);
        let collateral_factor = U128::from_u64(asset_config.liquidate_collateral_factor); // decimals 4
        let scale = U128::from_u64(10.pow(asset_config.decimals));

        liquidation_treshold += balance * price * collateral_factor / U128::from_u64(10000) / scale; //decimals 9
        index = index + 1;
        index = index + 1;
    }

    let scale = U128::from_u64(10.pow(config.base_token_decimals));
    let base_token_price = U128::from_u64(get_price(config.base_token, config.base_token_price_feed)); //decimals 9
    let borrow_amount = present_value_ * base_token_price / scale; // decimals 9
    liquidation_treshold < borrow_amount
}

// @Callable get_collateral_reserves(asset: AssetId) -> I64
// @Callable get_collateral_reserves(asset: AssetId) -> I64
#[storage(read)]
fn get_collateral_reserves_internal(asset_id: AssetId) -> I64 { // -> asset decimals
    I64::from(this_balance(asset_id)) - I64::from(storage.totals_collateral.get(asset_id).try_read().unwrap_or(0))
}

// @Callable get_reserves_internal() -> I64
#[storage(read)]
fn get_reserves_internal() -> I64 {  // base_token_decimals
    let config = MARKET_CONFIGURATION.unwrap();
    let market_basic = storage.market_basic.try_read().unwrap_or(MarketBasics::default());
    let (base_supply_index_, base_borrow_index_) = accrued_interest_indices(timestamp(), market_basic.last_accrual_time);  // decimals (18, 18)
    let balance = this_balance(config.base_token); // base_token_decimals
    let total_supply = present_value_supply(base_supply_index_, market_basic.total_supply_base); // base_token_decimals
    let total_borrow = present_value_borrow(base_borrow_index_, market_basic.total_borrow_base); // base_token_decimals
    I64::from(balance + total_borrow) - I64::from(total_supply)
}

#[storage(read, write)]
fn accrue_internal() {
    let config = MARKET_CONFIGURATION.unwrap();
    let mut market_basic = storage.market_basic.try_read().unwrap_or(MarketBasics::default());    let now = timestamp();
    let time_elapsed = now - market_basic.last_accrual_time;
    if time_elapsed > 0 {
        if market_basic.last_accrual_time != 0 {
            let (base_supply_index, base_borrow_index) = accrued_interest_indices(now, market_basic.last_accrual_time);
            market_basic.base_supply_index = base_supply_index;
            market_basic.base_borrow_index = base_borrow_index;
        };
        let total_supply_base = U128::from_u64(market_basic.total_supply_base); // base_asset_decimal
        let total_borrow_base = U128::from_u64(market_basic.total_borrow_base); // base_asset_decimal
        let tracking_supply_speed = U128::from_u64(config.base_tracking_supply_speed); // decimals 18
        let tracking_borrow_speed = U128::from_u64(config.base_tracking_borrow_speed); // decimals 18
        let min_for_rewards = U128::from_u64(config.base_min_for_rewards); // decimals 6
        let scale = U128::from_u64(10.pow(config.base_token_decimals));

        if total_supply_base >= min_for_rewards {
            market_basic.tracking_supply_index += (tracking_supply_speed * U128::from_u64(time_elapsed) / total_supply_base / scale).as_u64().unwrap(); // 18 
        }
        if total_borrow_base >= min_for_rewards {
            market_basic.tracking_borrow_index += (tracking_borrow_speed * U128::from_u64(time_elapsed) / total_borrow_base / scale).as_u64().unwrap();  // 18 
        }
        market_basic.last_accrual_time = now;
        storage.market_basic.write(market_basic);
    }
}

// the function through which any balance changes will pass. updates the reward variables on the user
#[storage(write, read)]
fn update_base_principal(account: Address, basic: UserBasic, principal_new: I64) {
    let market_basic = storage.market_basic.try_read().unwrap_or(MarketBasics::default());
    let principal = basic.principal;
    let mut basic = basic;
    basic.principal = principal_new;

    if basic.base_tracking_index == 0 {
        basic.base_tracking_index = SCALE_18
    };

    if principal >= I64::from(0) {
        let index_delta = market_basic.tracking_supply_index - basic.base_tracking_index; // decimals 18
        basic.base_tracking_accrued += principal.into() * index_delta / SCALE_18; // native_asset_decimal
    } else {
        let index_delta = market_basic.tracking_borrow_index - basic.base_tracking_index;
        basic.base_tracking_accrued += principal.flip().into() * index_delta / SCALE_18; // native_asset_decimal
    }
    if principal_new >= I64::from(0) {
        basic.base_tracking_index = market_basic.tracking_supply_index;
    } else {
        basic.base_tracking_index = market_basic.tracking_borrow_index;
    }
    storage.user_basic.insert(account, basic);
}

fn repay_and_supply_amount(old_principal: I64, new_principal: I64) -> (u64, u64) {
    // If the new principal is less than the old principal, then no amount has been repaid or supplied
    if new_principal < old_principal {
        return (0, 0)
    };

    if new_principal <= I64::from(0) {
        return ((new_principal - old_principal).into(), 0);
    } else if old_principal >= I64::from(0) {
        return (0, (new_principal - old_principal).into());
    } else {
        return (old_principal.flip().into(), new_principal.into());
    }
}

fn withdraw_and_borrow_amount(old_principal: I64, new_principal: I64) -> (u64, u64) {
    // If the new principal is greater than the old principal, then no amount has been withdrawn or borrowed
    if new_principal > old_principal {
        return (0, 0)
    };

    if new_principal >= I64::from(0) {
        return ((old_principal - new_principal).into(), 0);
    } else if old_principal <= I64::from(0) {
        return (0, (old_principal - new_principal).into());
    } else {
        return ((old_principal).into(), (new_principal).flip().into());
    }
}

#[storage(read)]
fn quote_collateral_internal(asset_id: AssetId, base_amount: u64) -> u64 { // asset decimals
    let config = MARKET_CONFIGURATION.unwrap();
    let asset_config = storage.asset_configurations.get(asset_id).read();
    let asset_price = get_price(asset_id, asset_config.price_feed); // decimals 9
    let base_price = get_price(config.base_token, config.base_token_price_feed); // decimals 9
    let store_front_price_factor = config.store_front_price_factor; // decimals 4
    let liquidation_penalty = asset_config.liquidation_penalty; // decimals 4
    // Store front discount is derived from the collateral asset's liquidation_penalty and store_front_price_factor
    let scale4 = 10.pow(4);
    let discount_factor = store_front_price_factor * (scale4 - liquidation_penalty) / scale4; // decimals 4
    let asset_price_discounted = asset_price * (scale4 - discount_factor) / scale4; // decimals 9
    // of collateral assets
    let base_scale = 10.pow(config.base_token_decimals);
    let asset_scale = 10.pow(asset_config.decimals);
    (U128::from_u64(base_price) * U128::from_u64(base_amount) * U128::from_u64(asset_scale) / U128::from_u64(asset_price_discounted) / U128::from_u64(base_scale)).as_u64().unwrap()
}

// the function transfers the pledge to the property of the protocol and closes the user's debt
#[storage(read, write)]
fn absorb_internal(account: Address) {
    require(is_liquidatable_internal(account), Error::NotLiquidatable);

    let account_user = storage.user_basic.get(account).try_read().unwrap_or(UserBasic::default());
    let old_principal = account_user.principal;
    let old_balance = present_value(old_principal);// base_asset_decimals
    let config = MARKET_CONFIGURATION.unwrap();

    let mut delta_value = U128::new(); // decimals 9
    let mut index = 0;
    while index < storage.asset_configurations_keys.len() { 
        let asset_config = storage.asset_configurations.get(storage.asset_configurations_keys.get(index).unwrap().read()).read();
        let seize_amount = storage.user_collateral.get((account, asset_config.asset_id)).try_read().unwrap_or(0); // asset decimals
        if seize_amount == 0 {
            index += 1;
            continue;
        }
        storage.user_collateral.insert((account, asset_config.asset_id), 0);

        let total_collateral = storage.totals_collateral.get(asset_config.asset_id).try_read().unwrap_or(0); // asset decimals
        storage.totals_collateral.insert(asset_config.asset_id, total_collateral - seize_amount);

        let price = get_price(asset_config.asset_id, asset_config.price_feed); // decimals 9
        let liquidation_penalty = asset_config.liquidation_penalty; // decimals 4
        let asset_scale = 10.pow(asset_config.decimals);
        let penalty_scale = 10.pow(4);
        delta_value += U128::from_u64(seize_amount) * U128::from_u64(price) * U128::from_u64(liquidation_penalty) / U128::from_u64(asset_scale) / U128::from_u64(penalty_scale); // decimals 9
        index += 1;
    }

    let base_price = get_price(config.base_token, config.base_token_price_feed); //decimals 9
    let base_scale = 10.pow(config.base_token_decimals);

    let delta_balance = delta_value.as_u64().unwrap() * base_scale / base_price; // base_asset_decimals
    let mut new_balance = old_balance + I64::from(delta_balance); // base_asset_decimals
    if new_balance < I64::from(0) {
        new_balance = I64::from(0);
    }

    let new_principal = principal_value(new_balance);
    update_base_principal(account, account_user, new_principal);

    let (repay_amount, supply_amount) = repay_and_supply_amount(old_principal, new_principal);

    // Reserves are decreased by increasing total supply and decreasing borrows
    //  the amount of debt repaid by reserves is `newBalance - oldBalance`
    let mut market_basic = storage.market_basic.try_read().unwrap_or(MarketBasics::default());
    market_basic.total_supply_base += supply_amount;
    market_basic.total_borrow_base -= repay_amount;
    storage.market_basic.write(market_basic);

    // if supply_amount > 0, issue LP token in the amount equal to supply_amount and send it to the user
    // if supply_amount > 0 {
        // mint_to_address(supply_amount, absorber);
    // }
}
