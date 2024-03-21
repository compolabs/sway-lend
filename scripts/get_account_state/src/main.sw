script;

use std::constants::ZERO_B256;
use market_abi::{Market, structs::*};

fn main() -> (
    Vec<CollateralConfiguration>,
    Vec<(b256, u64)>
    ) {
    let contract_address = 0x3fffc28bdb0a460263eeda9b56f9c5157c8048c25ed116c3a4e5cee78bb24bb9;
    let market = abi(Market, contract_address);
    let collateral_configurations = market.get_collateral_configurations();
    
    let mut totals_collateral: Vec<(b256, u64)> = Vec::new();
    let mut index = 0;
    while index < collateral_configurations.len() {
        let config = collateral_configurations.get(index);
        if config.is_some(){
            let config = config.unwrap();
            let collateral_configuration = market.totals_collateral(config.asset_id);
            totals_collateral.push((config.asset_id, collateral_configuration));
        }
        index += 1;
    }

    (
     collateral_configurations,   
     totals_collateral
    )
}
