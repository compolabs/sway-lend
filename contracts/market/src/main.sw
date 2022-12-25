// SPDX-License-Identifier: BUSL-1.1
contract;

/**
 *
 * @title Swaylend's Market Contract
 * @notice An efficient monolithic money market protocol
 * @author SWAY GANG
 */
dep constants;
dep market_configuration;

use constants::ZERO_B256;
use market_configuration::*;

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
    context::{
        balance_of,
        msg_amount,
    },
    contract_id::ContractId,
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
}

enum Error {
    Paused: (),
    Unauthorized: (),
}

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
