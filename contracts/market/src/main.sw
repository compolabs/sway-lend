// TODO:Add License Identifier
contract;

/**TODO: Add description like that
 *
 * @title Compound's Comet Contract
 * @notice An efficient monolithic money market protocol
 * @author Sway gang
 */
dep constants;
dep market_configuration;

use constants::ZERO_B256;
use market_configuration::MarketConfiguration;

use std::{
    address::*,
    auth::{
        AuthError,
        msg_sender,
    },
    call_frames::contract_id,
    context::{
        balance_of,
    },
    contract_id::ContractId,
    revert::require,
    storage::*,
    token::*,
};

abi Market {
    #[storage(write)]
    fn initialize(config: MarketConfiguration);

    #[storage(read)]
    fn configuration() -> MarketConfiguration;
}

storage {
    config: Option<MarketConfiguration> = Option::None,
}

#[storage(read)]
fn get_config() -> MarketConfiguration {
    match (storage.config) {
        Option::Some(c) => c,
        _ => revert(0),
    }
}

impl Market for Contract {
    /**
     * @notice Construct a new protocol instance
     * @param config The mapping of initial/constant parameters
     */
    #[storage(write)]
    fn initialize(config: MarketConfiguration) {
        storage.config = Option::Some(config);
    }

    #[storage(read)]
    fn configuration() -> MarketConfiguration {
        let config = get_config();
        config
    }
}
