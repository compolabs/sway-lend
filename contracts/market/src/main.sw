// SPDX-License-Identifier: BUSL-1.1
contract;

/**
 *
 * @title Swaylend's Market Contract
 * @notice An efficient monolithic money market protocol
 * @author SWAY GANG
 */
dep structs;
use structs::*;

use std::{
    address::*,
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
        balance_of,
        msg_amount,
    },
    contract_id::ContractId,
    hash::sha256,
    revert::require,
    storage::*,
    token::*,
};

abi Market {
    #[storage(write)]
    fn initialize(config: MarketConfiguration);

    #[storage(write, read)]
    fn pause(config: PauseConfiguration);

    #[storage(write, read)]
    fn supply();

    #[storage(read)]
    fn get_configuration() -> MarketConfiguration;
}

storage {
    config: Option<MarketConfiguration> = Option::None,
    pause_config: Option<PauseConfiguration> = Option::None,
    totals_collateral: StorageMap<ContractId, u64> = StorageMap {},
    user_collateral: StorageMap<b256, u64> = StorageMap {}, // sha256((address, asset_id) -> b256
    user_basic: StorageMap<Address, UserBasic> = StorageMap {},
    // TODO: fullfill storage.market_basic
    market_basic: MarketBasics = MarketBasics {
        base_supply_index: 0,
        base_borrow_index: 0,
        tracking_supply_index: 0,
        tracking_borrow_index: 0,
        total_supply_base: 0,
        total_borrow_base: 0,
        last_accrual_time: 0,
    },
}

// #[storage(read)]
// fn get_user_collateral(address: Address, asset: ContractId) -> u64 {
//     let key = sha256((address, asset));
//     storage.user_collateral.get(key)
// }

// #[storage(write)]
// fn set_user_collateral(address: Address, asset: ContractId, amount: u64) {
//     let key = sha256((address, asset));
//     storage.user_collateral.insert(key, amount);
// }

#[storage(read)]
fn get_config() -> MarketConfiguration {
    match (storage.config) {
        Option::Some(c) => c,
        _ => revert(0),
    }
}

pub fn get_msg_sender_address_or_panic() -> Address {
    let sender: Result<Identity, AuthError> = msg_sender();
    if let Identity::Address(address) = sender.unwrap() {
        address
    } else {
        revert(0);
    }
}

#[storage(read)]
fn is_supply_paused() -> bool {
    match (storage.pause_config) {
        Option::Some(config) => config.supply_paused,
        _ => false,
    }
}

#[storage(read)]
fn supply_internal(caller: Address, asset: ContractId, amount: u64) {
    require(!is_supply_paused(), Error::Paused);
    // require(!has_permission(from, operator), Error::Unauthorized);
    let config = get_config();
    // TODO: Inplement
    if asset == config.base_token {} else {}
}

// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
// ----------------------------------------------------------------------

const BASE_INDEX_SCALE: u64 = 1000000000000000000; //1e18
const FACTOR_SCALE: u64 = 1000000000000000000; // 1e18
// -> base_asset_decimal
pub fn present_value_supply(base_supply_index: u64, principal_value: u64) -> u64 {
    principal_value * base_supply_index / BASE_INDEX_SCALE
}

// -> base_asset_decimal
pub fn present_value_borrow(base_borrow_index: u64, principal_value: u64) -> u64 {
    principal_value * base_borrow_index / BASE_INDEX_SCALE
}

// -> base_asset_decimal
pub fn principal_value_supply(base_supply_index: u64, present_value: u64) -> u64 {
    present_value * BASE_INDEX_SCALE / base_supply_index
}

// TODO: че за -1 бля?
pub fn principal_value_borrow(base_borrow_index: u64, present_value: u64) -> u64 {
    (present_value * BASE_INDEX_SCALE + base_borrow_index - 1) / base_borrow_index
}


#[storage(read)]
fn get_utilization() -> u64 {
    let sender = get_msg_sender_address_or_panic();
    let market_basic = storage.market_basic;
    let total_supply = present_value_supply(market_basic.base_supply_index, market_basic.total_supply_base);
    let total_borrow = present_value_borrow(market_basic.base_borrow_index, market_basic.total_borrow_base);
    if total_supply == 0 {0} else {total_borrow * FACTOR_SCALE / total_supply}
}

#[storage(read)]
fn get_supply_rate(utilization: u64) -> u64 {
    let config = get_config();   
    let kink = config.kink;
    let interest_rate_base = config.interest_rate_base_per_second;
    let interest_rate_slope_low = config.supply_per_second_interest_rate_slope_low;
    let interest_rate_slope_high = config.supply_per_second_interest_rate_slope_high;
    if (utilization <= kink) {
        interest_rate_base + interest_rate_slope_low * utilization
    } else {
        interest_rate_base + interest_rate_slope_low * kink + interest_rate_slope_high * (utilization - kink)
    }
}

#[storage(read)]
fn get_borrow_rate(utilization: u64) -> u64 {
    let config = get_config();   
    let kink = config.kink;
    let interest_rate_base = config.interest_rate_base_per_second;
    let interest_rate_slope_low = config.borrow_per_second_interest_rate_slope_low;
    let interest_rate_slope_high = config.borrow_per_second_interest_rate_slope_high;
    if (utilization <= kink) {
        interest_rate_base + interest_rate_slope_low * utilization
    } else {
        interest_rate_base + interest_rate_slope_low * kink + interest_rate_slope_high * (utilization - kink)
    }
}

#[storage(read)]
fn accrued_interest_indices(time_elapsed: u64) -> (u64, u64) {
    let mut base_supply_index = storage.market_basic.base_supply_index;
    let mut base_borrow_index = storage.market_basic.base_borrow_index;
    if (time_elapsed > 0) {
        let utilization = get_utilization();
        let supply_rate = get_supply_rate(utilization);
        let borrow_rate = get_borrow_rate(utilization);
        base_supply_index += base_supply_index * supply_rate * time_elapsed; 
        base_borrow_index += base_supply_index * supply_rate * time_elapsed;
    }
    return (base_supply_index, base_borrow_index);
}
// fn get_price(asset: ContractId) -> u64 {
    // return 0
// }
// fn is_borrow_collateralized(account: Address) -> bool {
//     let config = get_config();
//     let borrow_amount = present_value_borrow(0, 0) * get_price(config.base_token);
//     let mut borrow_limit = 0 ;
//     // for asset in supplied_assets:
//     //     borrow_limit += balance(asset) * price(asset) * borrow_collateral_factor(asset)
//     // return borrow_limit >= borrow_amount
//     false
// }


// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
impl Market for Contract {
    #[storage(write)]
    fn initialize(config: MarketConfiguration) {
        storage.config = Option::Some(config);
    }

    #[storage(write, read)]
    fn pause(pause_config: PauseConfiguration) {
        let sender = get_msg_sender_address_or_panic();
        let config = get_config();
        require(sender == config.governor || sender == config.pause_guardian, Error::Unauthorized);
        storage.pause_config = Option::Some(pause_config);
    }

    #[storage(write, read)]
    fn supply() {
        let sender = get_msg_sender_address_or_panic();
        let asset = msg_asset_id();
        let amount = msg_amount();
        return supply_internal(sender, asset, amount);
    }

    #[storage(read)]
    fn get_configuration() -> MarketConfiguration {
        get_config()
    }
}
