// SPDX-License-Identifier: BUSL-1.1
contract;
/**
 *
 * @title Swaylend's Market Contract
 * @notice An efficient monolithic money market protocol
 * @author SWAY GANG
 */
dep numbers;
dep structs;
dep i64;

use i64::I64;
use structs::*;
use numbers::*;
use oracle_abi::*;
use token_abi::*;
use std::{
    auth::{
        AuthError,
        msg_sender,
    },
    call_frames::{
        contract_id,
        msg_asset_id,
    },
    constants::ZERO_B256,
    context::{
        msg_amount,
        this_balance,
    },
    hash::sha256,
    logging::log,
    revert::require,
    storage::StorageVec,
    token::transfer_to_address,
    u128::U128,
};

abi Market {
    #[storage(read, write)]
    fn debug_increment_timestamp();

    #[storage(read, write)]
    fn initialize(config: MarketConfiguration, asset_configs: Vec<AssetConfig>, debug_step: Option<u64>);

    #[storage(write, read)]
    fn pause(config: PauseConfiguration);

    #[storage(read)]
    fn get_configuration() -> MarketConfiguration;

    #[storage(read)]
    fn get_user_basic(account: Address) -> UserBasic;

    #[storage(read)]
    fn get_user_collateral(address: Address, asset: ContractId) -> u64;

    #[storage(read)]
    fn get_oracle_price(asset: ContractId) -> u64;

    #[storage(read)]
    fn get_asset_config_by_asset_id(asset: ContractId) -> AssetConfig;

    #[storage(read)]
    fn get_user_supply_borrow(account: Address) -> (u64, u64);

    fn balance_of(asset: ContractId) -> u64;

    #[storage(read)]
    fn get_market_basics() -> MarketBasics;

    #[storage(read)]
    fn totals_collateral(asset: ContractId) -> u64;

    #[storage(read)]
    fn available_to_borrow(account: Address) -> u64;
    //-------------------------------------------------
    #[storage(read)]
    fn get_utilization() -> u64;

    #[storage(read)]
    fn get_supply_rate(utilization: u64) -> u64;

    #[storage(read)]
    fn get_borrow_rate(utilization: u64) -> u64;

    #[storage(read)]
    fn is_liquidatable(account: Address) -> bool;

    #[storage(read)]
    fn get_collateral_reserves(asset: ContractId) -> I64;

    #[storage(read)]
    fn get_reserves() -> I64;

    // TODO: test withdraw_reserves
    #[storage(read)]
    fn withdraw_reserves(to: Address, amount: u64);

    // TODO: test quote_collateral
    #[storage(read)]
    fn quote_collateral(asset: ContractId, base_amount: u64) -> u64;

    #[storage(read)]
    fn collateral_value_to_sell(asset: ContractId, collateral_amount: u64) -> u64;

    #[storage(read, write)]
    fn absorb(accounts: Vec<Address>);

    #[storage(read)]
    fn buy_collateral(asset: ContractId, min_amount: u64, recipient: Address);
    #[storage(read, write)]
    fn supply_collateral();
    #[storage(read, write)]
    fn withdraw_collateral(asset: ContractId, amount: u64);

    #[storage(read, write)]
    fn supply_base();
    #[storage(read, write)]
    fn withdraw_base(amount: u64);

    // #[storage(read)]
    // fn withdraw_reward_token(to: Address, amount: u64);

    // #[storage(read, write)]
    // fn get_reward_owed(account: Address) -> u64;

    // #[storage(read, write)]
    // fn claim();
}

const SCALE_18: u64 = 1_000_000_000_000_000_000; // 1e18
storage {
    debug: bool = false,
    debug_timestamp: u64 = 0,
    debug_step: u64 = 0,
    //----------
    config: Option<MarketConfiguration> = Option::None,
    asset_configs: StorageVec<AssetConfig> = StorageVec {},
    pause_config: Option<PauseConfiguration> = Option::None,
    totals_collateral: StorageMap<ContractId, u64> = StorageMap {},
    user_collateral: StorageMap<(Address, ContractId), u64> = StorageMap {},
    user_basic: StorageMap<Address, UserBasic> = StorageMap {},
    // borrowers: StorageVec<Address> = StorageVec {},
    market_basic: MarketBasics = MarketBasics {
        base_supply_index: SCALE_18,
        base_borrow_index: SCALE_18,
        tracking_supply_index: SCALE_18,
        tracking_borrow_index: SCALE_18,
        total_supply_base: 0,
        total_borrow_base: 0,
        last_accrual_time: 0,
    },
}

#[storage(read)]
fn timestamp() -> u64 {
    if storage.debug {
        storage.debug_timestamp
    } else {
        std::block::timestamp()
    }
}

#[storage(read)]
fn mint_reward_token(amount: u64, recipient: Address) {
    let config = get_config();
    abi(Token, config.reward_token.value).mint_and_transfer(amount, recipient);
}

#[storage(read)]
fn is_absorb_paused() -> bool {
    match storage.pause_config {
        Option::Some(config) => config.absorb_paused,
        Option::None(_) => false,
    }
}

#[storage(read)]
fn is_buy_paused() -> bool {
    match storage.pause_config {
        Option::Some(config) => config.buy_pause,
        Option::None(_) => false,
    }
}
#[storage(read)]
fn is_supply_paused() -> bool {
    match storage.pause_config {
        Option::Some(config) => config.supply_paused,
        Option::None(_) => false,
    }
}
#[storage(read)]
fn is_withdraw_paused() -> bool {
    match storage.pause_config {
        Option::Some(config) => config.withdraw_paused,
        Option::None(_) => false,
    }
}

// #[storage(read)]
// fn is_claim_paused() -> bool {
//     match storage.pause_config {
//         Option::Some(config) => config.claim_paused,
//         Option::None(_) => false,
//     }
// }

#[storage(read)]
fn get_config() -> MarketConfiguration {
    match storage.config {
        Option::Some(c) => c,
        _ => revert(0),
    }
}

pub fn get_caller() -> Address {
    let sender: Result<Identity, AuthError> = msg_sender();
    if let Identity::Address(address) = sender.unwrap() {
        address
    } else {
        revert(0);
    }
}

fn get_price(asset: ContractId, price_feed: ContractId) -> u64 {
    let res = abi(Oracle, price_feed.value).get_price(asset);
    res.price
}

// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
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
    let base_supply_index = storage.market_basic.base_supply_index; // -> decimals 18
    let base_borrow_index = storage.market_basic.base_borrow_index; // -> decimals 18
    if principal_value_ >= I64::from(0) {
        let present_value_u64 = present_value_supply(base_supply_index, principal_value_.into());
        I64::from(present_value_u64)
    } else {
        let present_value_u64 = present_value_borrow(base_borrow_index, principal_value_.flip().into());
        I64::from(present_value_u64).flip()
    }
}

#[storage(read)]
fn principal_value(present_value_: I64) -> I64 { // -> base_asset_decimals
    let base_supply_index = storage.market_basic.base_supply_index; // -> decimals 18
    let base_borrow_index = storage.market_basic.base_borrow_index; // -> decimals 18
    if present_value_ >= I64::from(0) {
        let principal_value_u64 = principal_value_supply(base_supply_index, present_value_.into());
        I64::from(principal_value_u64)
    } else {
        let principal_value_u64 = principal_value_borrow(base_borrow_index, present_value_.flip().into());
        I64::from(principal_value_u64).flip()
    }
}

// @Callable get_utilization() -> u64
#[storage(read)]
fn get_utilization_internal() -> u64 { // -> decimals 18
    let market_basic = storage.market_basic;
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
#[storage(read)]
fn get_supply_rate_internal(utilization: u64) -> U128 { // -> decimals 18
    let utilization = U128::from_u64(utilization);
    let config = get_config();
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
#[storage(read)]
fn get_borrow_rate_internal(utilization: u64) -> U128 { // -> decimals 18
    let utilization = U128::from_u64(utilization); // decimals 18
    let config = get_config();
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
    let mut base_supply_index_ = U128::from_u64(storage.market_basic.base_supply_index); // decimals 18
    let mut base_borrow_index_ = U128::from_u64(storage.market_basic.base_borrow_index); // decimals 18
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

// Checks that the dollar value of the user's collateral multiplied 
// by borrow_collateral_factor is greater than the (planned) loan amount.
#[storage(read)]
fn is_borrow_collateralized(account: Address) -> bool {
    let principal_value_ = storage.user_basic.get(account).principal; // decimals base_asset_decimal
    if principal_value_ >= I64::new() {
        return true
    };

    let config = get_config();
    let present_value_ = present_value(principal_value_.flip()); // decimals base_asset_decimals
    let mut borrow_limit = U128::new();
    let mut index = 0;
    while index < storage.asset_configs.len() {
        let asset_config = match storage.asset_configs.get(index) {
            Option::Some(asset_config) => asset_config,
            Option::None => {
                index = index + 1;
                continue
            },
        };

        let balance = storage.user_collateral.get((account, asset_config.asset)); // decimals asset_config.decimals
        let balance = U128::from_u64(balance);

        let price = get_price(asset_config.asset, asset_config.price_feed); // decimals 9
        let price = U128::from_u64(price);

        let collateral_factor = U128::from_u64(asset_config.borrow_collateral_factor); // decimals 4
        let scale = U128::from_u64(10.pow(asset_config.decimals));

        borrow_limit += balance * price * collateral_factor / U128::from_u64(10000) / scale; //decimals 9
        index = index + 1;
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
    let principal_value_ = storage.user_basic.get(account).principal; // decimals base_asset_decimal
    if principal_value_ >= I64::new() {
        return false
    };

    let config = get_config();
    let present_value_ = U128::from_u64(present_value(principal_value_.flip()).into()); // decimals base_asset_decimals
    let mut liquidation_treshold = U128::new();
    let mut index = 0;
    while index < storage.asset_configs.len() {
        let asset_config = match storage.asset_configs.get(index) {
            Option::Some(asset_config) => asset_config,
            Option::None => {
                index = index + 1;
                continue
            },
        };

        let balance = storage.user_collateral.get((account, asset_config.asset)); // decimals asset_config.decimals
        let balance = U128::from_u64(balance);
        let price = get_price(asset_config.asset, asset_config.price_feed); // decimals 9
        let price = U128::from_u64(price);
        let collateral_factor = U128::from_u64(asset_config.liquidate_collateral_factor); // decimals 4
        let scale = U128::from_u64(10.pow(asset_config.decimals));

        liquidation_treshold += balance * price * collateral_factor / U128::from_u64(10000) / scale; //decimals 9
        index = index + 1;
    }

    let scale = U128::from_u64(10.pow(config.base_token_decimals));
    let base_token_price = U128::from_u64(get_price(config.base_token, config.base_token_price_feed)); //decimals 9
    let borrow_amount = present_value_ * base_token_price / scale; // decimals 9
    liquidation_treshold < borrow_amount
}

// @Callable get_collateral_reserves(asset: ContractId) -> I64
#[storage(read)]
fn get_collateral_reserves_internal(asset: ContractId) -> I64 { // -> asset decimals
    I64::from(this_balance(asset)) - I64::from(storage.totals_collateral.get(asset))
}

// @Callable get_reserves_internal() -> I64
#[storage(read)]
fn get_reserves_internal() -> I64 {  // base_token_decimals
    let config = get_config();
    let last_accrual_time = storage.market_basic.last_accrual_time;
    let (base_supply_index_, base_borrow_index_) = accrued_interest_indices(timestamp(), last_accrual_time);  // decimals (18, 18)
    let balance = this_balance(config.base_token); // base_token_decimals
    let total_supply = present_value_supply(base_supply_index_, storage.market_basic.total_supply_base); // base_token_decimals
    let total_borrow = present_value_borrow(base_borrow_index_, storage.market_basic.total_borrow_base); // base_token_decimals
    I64::from(balance + total_borrow) - I64::from(total_supply)
}

#[storage(read, write)]
fn accrue_internal() {
    let config = get_config();
    let mut market_basic = storage.market_basic;
    let now = timestamp();
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
        storage.market_basic = market_basic;
    }
}

// the function through which any balance changes will pass. updates the reward variables on the user
#[storage(write, read)]
fn update_base_principal(account: Address, basic: UserBasic, principal_new: I64) {
    let principal = basic.principal;
    let mut basic = basic;
    basic.principal = principal_new;

    if basic.base_tracking_index == 0 {
        basic.base_tracking_index = SCALE_18
    };

    if principal >= I64::from(0) {
        let index_delta = storage.market_basic.tracking_supply_index - basic.base_tracking_index; // decimals 18
        basic.base_tracking_accrued += principal.into() * index_delta / SCALE_18; // native_asset_decimal
    } else {
        let index_delta = storage.market_basic.tracking_borrow_index - basic.base_tracking_index;
        basic.base_tracking_accrued += principal.flip().into() * index_delta / SCALE_18; // native_asset_decimal
    }
    if principal_new >= I64::from(0) {
        basic.base_tracking_index = storage.market_basic.tracking_supply_index;
    } else {
        basic.base_tracking_index = storage.market_basic.tracking_borrow_index;
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

// @Callable withdraw_reserves(to: Address, amount: u64)
#[storage(read)]
fn withdraw_reserves_internal(to: Address, amount: u64) {
    let config = get_config();
    let sender = get_caller();

    require(sender == config.governor, Error::Unauthorized);
    let reserves = get_reserves_internal();
    require(reserves >= I64::from(0) && amount <= reserves.into(), Error::InsufficientReserves);

    transfer_to_address(amount, config.base_token, to);
}

#[storage(read)]
fn get_asset_config_by_asset_id_internal(asset: ContractId) -> AssetConfig {
    let mut out: Option<AssetConfig> = Option::None;
    let mut i = 0;
    while i < storage.asset_configs.len() {
        let asset_config = storage.asset_configs.get(i).unwrap();
        if asset_config.asset == asset {
            out = Option::Some(asset_config);
            break;
        }
        i += 1;
    }
    match out {
        Option::Some(v) => v,
        Option::None(_) => revert(0),
    }
}

// @Callable quote_collateral(asset: ContractId, base_amount: u64) -> u64 
//  A function that counts the collateral purchase rate of the user from our protocol
//  i.e. how much collateral the user gets for redeeming one dollar of debt to be liquidated
#[storage(read)]
fn quote_collateral_internal(asset: ContractId, base_amount: u64) -> u64 { // asset decimals
    let config = get_config();
    let asset_config = get_asset_config_by_asset_id_internal(asset);
    let asset_price = get_price(asset, asset_config.price_feed); // decimals 9
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

// @Callable collateral_value_to_sell_internal(asset: ContractId, base_amount: u64) -> u64 
#[storage(read)]
fn collateral_value_to_sell_internal(asset: ContractId, collateral_amount: u64) -> u64 { // asset decimals
    let config = get_config();
    let asset_config = get_asset_config_by_asset_id_internal(asset);
    let asset_price = get_price(asset, asset_config.price_feed); // decimals 9
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

   // @Callable absorb(absorber: Address, accounts: Vec<Address>)
// the function transfers the pledge to the property of the protocol and closes the user's debt
#[storage(read, write)]
fn absorb_internal(account: Address) {
    require(is_liquidatable_internal(account), Error::NotLiquidatable);

    let account_user = storage.user_basic.get(account);
    let old_principal = account_user.principal;
    let old_balance = present_value(old_principal);// base_asset_decimals
    let config = get_config();

    let mut delta_value = U128::new(); // decimals 9
    let mut i = 0;
    while i < storage.asset_configs.len() {
        let asset_config = storage.asset_configs.get(i).unwrap();
        let asset = asset_config.asset;
        let seize_amount = storage.user_collateral.get((account, asset)); // asset decimals
        if seize_amount == 0 {
            i += 1;
            continue;
        }
        storage.user_collateral.insert((account, asset), 0);

        let total_collateral = storage.totals_collateral.get(asset); // asset decimals
        storage.totals_collateral.insert(asset, total_collateral - seize_amount);

        let price = get_price(asset, asset_config.price_feed); // decimals 9
        let liquidation_penalty = asset_config.liquidation_penalty; // decimals 4
        let asset_scale = 10.pow(asset_config.decimals);
        let penalty_scale = 10.pow(4);
        delta_value += U128::from_u64(seize_amount) * U128::from_u64(price) * U128::from_u64(liquidation_penalty) / U128::from_u64(asset_scale) / U128::from_u64(penalty_scale); // decimals 9
        i += 1;
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
    let mut market_basic = storage.market_basic;
    market_basic.total_supply_base += supply_amount;
    market_basic.total_borrow_base -= repay_amount;
    storage.market_basic = market_basic;

    // if supply_amount > 0, issue LP token in the amount equal to supply_amount and send it to the user
    // if supply_amount > 0 {
    //     mint_to_address(supply_amount, absorber);
    // }
}

// @Callable buy_collateral(asset: ContractId, min_amount: u64, recipient: Address)
// @Payment base_token
// function for buying the collateral of a liquidated user. 
#[storage(read)]
fn buy_collateral_internal(asset: ContractId, min_amount: u64, recipient: Address) {
    require(!is_buy_paused(), Error::Paused);
    let config = get_config();
    let base_amount = msg_amount();
    require(msg_asset_id() == config.base_token && base_amount > 0, Error::InvalidPayment);

    let reserves = get_reserves_internal();
    require(reserves < I64::from(0) || reserves.into() < config.target_reserves, Error::NotForSale);

    // Note: Re-entrancy can skip the reserves check above on a second buyCollateral call.
    let reserves = get_collateral_reserves_internal(asset);
    let collateral_amount = quote_collateral_internal(asset, base_amount);
    require(collateral_amount >= min_amount, Error::TooMuchSlippage);
    require(I64::from(collateral_amount) <= reserves, Error::InsufficientReserves);

    // Note: Pre-transfer hook can re-enter buyCollateral with a stale collateral ERC20 balance.
    //  Assets should not be listed which allow re-entry from pre-transfer now, as too much collateral could be bought.
    //  This is also a problem if quoteCollateral derives its discount from the collateral ERC20 balance.
    transfer_to_address(collateral_amount, asset, recipient);
}

// @Callable supply_collateral()
// @Payment any collateral asset
#[storage(read, write)]
fn supply_collateral_internal() {
    let dst = get_caller();
    require(!is_supply_paused(), Error::Paused);

    let amount = msg_amount();
    require(amount > 0, Error::InvalidPayment);

    let asset = msg_asset_id();
    let asset_config = get_asset_config_by_asset_id_internal(asset);
    let mut total_supply_asset = storage.totals_collateral.get(asset);
    total_supply_asset += amount;
    require(total_supply_asset <= asset_config.supply_cap, Error::SupplyCapExceeded);

    let dst_collateral = storage.user_collateral.get((dst, asset));
    let dst_collateral_new = dst_collateral + amount;

    storage.totals_collateral.insert(asset, total_supply_asset);
    storage.user_collateral.insert((dst, asset), dst_collateral_new);
}

// @Callable withdraw_collateral(asset: ContractId, amount: u64)
#[storage(read, write)]
fn withdraw_collateral_internal(asset: ContractId, amount: u64) {
    let caller = get_caller();
    let src_collateral = storage.user_collateral.get((caller, asset));
    let src_collateral_new = src_collateral - amount;

    let new_total_supply_asset = storage.totals_collateral.get(asset) - amount;
    storage.totals_collateral.insert(asset, new_total_supply_asset);
    storage.user_collateral.insert((caller, asset), src_collateral_new);

    // Note: no accrue interest, BorrowCF < LiquidationCF covers small changes
    require(is_borrow_collateralized(caller), Error::NotCollateralized);

    transfer_to_address(amount, asset, caller);
}

// @Callable supply_base()
// @Payment base_token
#[storage(read, write)]
fn supply_base_internal() {
    require(!is_supply_paused(), Error::Paused);
    let caller = get_caller();
    let config = get_config();
    let amount = msg_amount();

    require(amount > 0, Error::InvalidPayment);
    require(msg_asset_id() == config.base_token, Error::InvalidPayment);

    accrue_internal();

    let dst_user = storage.user_basic.get(caller);
    let dst_principal = dst_user.principal;
    let dst_present_value = present_value(dst_principal);

    let dst_balance = dst_present_value + I64::from(amount);
    let dst_principal_new = principal_value(dst_balance);

    let (repay_amount, supply_amount) = repay_and_supply_amount(dst_principal, dst_principal_new);
    let mut market_basic = storage.market_basic;
    market_basic.total_supply_base += supply_amount;
    market_basic.total_borrow_base -= repay_amount;
    storage.market_basic = market_basic;
    update_base_principal(caller, dst_user, dst_principal_new);

    // if supply_amount > 0 {
    //     mint_to_address(supply_amount, caller);
    // }
}

// @Callable withdraw_base(amount: u64)
#[storage(read, write)]
fn withdraw_base_internal(amount: u64) {
    require(!is_withdraw_paused(), Error::Paused);
    // let amount = msg_amount();
    // require(msg_asset_id() == contract_id() && amount > 0, Error::InvalidPayment);
    require(amount > 0, Error::InvalidPayment);
    accrue_internal();

    let config = get_config();
    let caller = get_caller();
    let src_user = storage.user_basic.get(caller);
    let src_principal = src_user.principal;
    let src_balance = present_value(src_principal) - I64::from(amount);
    let src_principal_new = principal_value(src_balance);

    let (withdraw_amount, borrow_amount) = withdraw_and_borrow_amount(src_principal, src_principal_new);
    let mut market_basic = storage.market_basic;
    market_basic.total_supply_base -= withdraw_amount;
    market_basic.total_borrow_base += borrow_amount;
    storage.market_basic = market_basic;

    update_base_principal(caller, src_user, src_principal_new);

    if src_balance < I64::from(0) {
        // storage.borrowers.push(caller);
        require(src_balance.flip().into() >= config.base_borrow_min, Error::BorrowTooSmall);
        require(is_borrow_collateralized(caller), Error::NotCollateralized);
    }

    transfer_to_address(amount, config.base_token, caller);

    // burn(withdraw_amount); 
}

// @Callable withdraw_reward_token(to: Address, amount: u64)
// function for removing reward tokens by the admin. The reward_token is set in the config
#[storage(read)]
fn withdraw_reward_token_internal(to: Address, amount: u64) {
    let config = get_config();
    let sender = get_caller();
    require(sender == config.governor, Error::NotPermitted());

    mint_reward_token(amount, to);
}

// @Callable get_reward_owed(account: Address) -> u64
#[storage(read, write)]
fn get_reward_owed_internal(account: Address) -> u64 {
    accrue_internal();

    let basic = storage.user_basic.get(account);
    update_base_principal(account, basic, basic.principal);

    let claimed = storage.user_basic.get(account).reward_claimed;
    let accrued = storage.user_basic.get(account).base_tracking_accrued;

    if accrued > claimed {
        accrued - claimed
    } else {
        0
    }
}

// @Callable claim()
#[storage(read, write)]
fn claim_internal() {
    // require(!is_claim_paused(), Error::Paused);
    let caller = get_caller();

    accrue_internal();

    let basic = storage.user_basic.get(caller);
    update_base_principal(caller, basic, basic.principal);
    let mut basic = storage.user_basic.get(caller);
    let claimed = basic.reward_claimed;
    let accrued = basic.base_tracking_accrued;

    if accrued > claimed {
        basic.reward_claimed = accrued;
        storage.user_basic.insert(caller, basic);

        let owed = accrued - claimed;
        mint_reward_token(owed, caller);
    }
}

impl Market for Contract {
    #[storage(read, write)]
    fn debug_increment_timestamp() {
        require(storage.debug, Error::DebuggingDisabled);
        storage.debug_timestamp = storage.debug_timestamp + storage.debug_step;
    }

    #[storage(read)]
    fn get_oracle_price(asset: ContractId) -> u64 {
        let base_token_price_feed = get_config().base_token_price_feed;
        get_price(asset, base_token_price_feed)
    }

    #[storage(read, write)]
    fn initialize(
        config: MarketConfiguration,
        asset_configs: Vec<AssetConfig>,
        debug_step: Option<u64>,
    ) {
        require(storage.config.is_none(), Error::AlreadyInitialized);
        storage.config = Option::Some(config);
        let mut i = 0;
        while i < asset_configs.len() {
            storage.asset_configs.push(asset_configs.get(i).unwrap());
            i += 1;
        }
        if debug_step.is_some() {
            storage.debug = true;
            storage.debug_timestamp = std::block::timestamp();
            storage.debug_step = debug_step.unwrap();
        }
        let mut market_basic = storage.market_basic;
        market_basic.base_supply_index = SCALE_18;
        market_basic.base_borrow_index = SCALE_18;
        market_basic.tracking_supply_index = SCALE_18;
        market_basic.tracking_borrow_index = SCALE_18;
        storage.market_basic = market_basic;
    }

    #[storage(write, read)]
    fn pause(pause_config: PauseConfiguration) {
        let sender = get_caller();
        let config = get_config();
        require(sender == config.governor || sender == config.pause_guardian, Error::Unauthorized);
        storage.pause_config = Option::Some(pause_config);
    }

    #[storage(read)]
    fn get_configuration() -> MarketConfiguration {
        get_config()
    }

    #[storage(read)]
    fn get_user_basic(account: Address) -> UserBasic {
        storage.user_basic.get(account)
    }

    #[storage(read)]
    fn get_user_collateral(address: Address, asset: ContractId) -> u64 {
        storage.user_collateral.get((address, asset))
    }

    #[storage(read)]
    fn get_user_supply_borrow(account: Address) -> (u64, u64) {
        let principal_value = storage.user_basic.get(account).principal;
        let last_accrual_time = storage.market_basic.last_accrual_time;
        let (supply_index_, borrow_index_) = accrued_interest_indices(timestamp(), last_accrual_time);   // decimals (18, 18)
        if principal_value >= I64::new() {
            let supply = present_value_supply(supply_index_, principal_value.into());
            (supply, 0)
        } else {
            let borrow = present_value_borrow(borrow_index_, principal_value.flip().into());
            (0, borrow)
        }
    }

    fn balance_of(asset: ContractId) -> u64 {
        this_balance(asset)
    }

    #[storage(read)]
    fn get_market_basics() -> MarketBasics {
        storage.market_basic
    }

    #[storage(read)]
    fn totals_collateral(asset: ContractId) -> u64 {
        storage.totals_collateral.get(asset)
    }

    #[storage(read)]
    fn available_to_borrow(account: Address) -> u64 {
        let mut borrow_limit = U128::new();
        let mut index = 0;
        let config = get_config();
        while index < storage.asset_configs.len() {
            let asset_config = match storage.asset_configs.get(index) {
                Option::Some(asset_config) => asset_config,
                Option::None => {
                    index = index + 1;
                    continue
                },
            };

            let balance = storage.user_collateral.get((account, asset_config.asset));
            let balance = U128::from_u64(balance);
            let price = get_price(asset_config.asset, asset_config.price_feed); // decimals 9
            let price = U128::from_u64(price);

            let collateral_factor = U128::from_u64(asset_config.borrow_collateral_factor); // decimals 4
            let scale = U128::from_u64(10.pow(asset_config.decimals + 4 + 9 - config.base_token_decimals));

            borrow_limit += balance * price * collateral_factor / scale; //base_token_decimals
            index = index + 1;
        };
        borrow_limit.as_u64().unwrap()
    }
    //-----------------------------------
    #[storage(read)]
    fn get_utilization() -> u64 {
        get_utilization_internal()
    }

    #[storage(read)]
    fn get_supply_rate(utilization: u64) -> u64 {
        get_supply_rate_internal(utilization).as_u64().unwrap()
    }
    #[storage(read)]
    fn get_borrow_rate(utilization: u64) -> u64 {
        get_borrow_rate_internal(utilization).as_u64().unwrap()
    }
    #[storage(read)]
    fn is_liquidatable(account: Address) -> bool {
        is_liquidatable_internal(account)
    }
    #[storage(read)]
    fn get_collateral_reserves(asset: ContractId) -> I64 {
        get_collateral_reserves_internal(asset)
    }

    #[storage(read)]
    fn get_reserves() -> I64 {
        get_reserves_internal()
    }

    #[storage(read)]
    fn withdraw_reserves(to: Address, amount: u64) {
        withdraw_reserves_internal(to, amount)
    }
    #[storage(read)]
    fn quote_collateral(asset: ContractId, base_amount: u64) -> u64 {
        quote_collateral_internal(asset, base_amount)
    }

    #[storage(read)]
    fn collateral_value_to_sell(asset: ContractId, collateral_amount: u64) -> u64 {
        collateral_value_to_sell_internal(asset, collateral_amount)
    }

    #[storage(read, write)]
    fn absorb(accounts: Vec<Address>) {
        require(!is_absorb_paused(), Error::Paused);
        accrue_internal();
        let mut i = 0;
        while i < accounts.len() {
            absorb_internal(accounts.get(0).unwrap());
            i += 1;
        }
    }

    #[storage(read)]
    fn buy_collateral(asset: ContractId, min_amount: u64, recipient: Address) { // @Payment base_token
        buy_collateral_internal(asset, min_amount, recipient)
    }

    #[storage(read, write)]
    fn supply_collateral() { // @Payment any collateral asset
        supply_collateral_internal()
    }

    #[storage(read, write)]
    fn withdraw_collateral(asset: ContractId, amount: u64) {
        withdraw_collateral_internal(asset, amount)
    }

    #[storage(read, write)]
    fn supply_base() { // @Payment base_token
        supply_base_internal()
    }

    #[storage(read, write)]
    fn withdraw_base(amount: u64) {
        withdraw_base_internal(amount)
    }

    // #[storage(read)]
    // fn withdraw_reward_token(to: Address, amount: u64) {
    //     withdraw_reward_token_internal(to, amount)
    // }

    // #[storage(read, write)]
    // fn get_reward_owed(account: Address) -> u64 {
    //     get_reward_owed_internal(account)
    // }

    // #[storage(read, write)]
    // fn claim() {
    //     claim_internal()
    // }

    #[storage(read)]
    fn get_asset_config_by_asset_id(asset: ContractId) -> AssetConfig {
        get_asset_config_by_asset_id_internal(asset)
    }
}
