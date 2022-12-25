// SPDX-License-Identifier: BUSL-1.1
library market_configuration;
/*   
███████╗██╗    ██╗ █████╗ ██╗   ██╗     ██████╗  █████╗ ███╗   ██╗ ██████╗ 
██╔════╝██║    ██║██╔══██╗╚██╗ ██╔╝    ██╔════╝ ██╔══██╗████╗  ██║██╔════╝ 
███████╗██║ █╗ ██║███████║ ╚████╔╝     ██║  ███╗███████║██╔██╗ ██║██║  ███╗
╚════██║██║███╗██║██╔══██║  ╚██╔╝      ██║   ██║██╔══██║██║╚██╗██║██║   ██║
███████║╚███╔███╔╝██║  ██║   ██║       ╚██████╔╝██║  ██║██║ ╚████║╚██████╔╝
╚══════╝ ╚══╝╚══╝ ╚═╝  ╚═╝   ╚═╝        ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝                                                                         
*/

pub struct MarketConfiguration {
    governor: Address,
    pause_guardian: Address,
    base_token: ContractId,
    base_token_decimals: u8,
    base_token_price_feed: ContractId,
    kink: u64, // decimals: 4
    supply_per_second_interest_rate_slope_low: u64, // decimals: 18
    supply_per_second_interest_rate_slope_high: u64, // decimals: 18
    borrow_per_second_interest_rate_slope_low: u64, // decimals: 18
    borrow_per_second_interest_rate_slope_high: u64, // decimals: 18
    borrow_per_second_interest_rate_base: u64, // decimals: 18
    store_front_price_factor: u64, // decimals: 4
    base_tracking_supply_speed: u64, // decimals 18
    base_tracking_borrow_speed: u64, // decimals 18
    base_min_for_rewards: u64, // decimals base_token_decimals
    base_borrow_min: u64, // decimals: base_token_decimals
    target_reserves: u64, // decimals: base_token_decimals
    asset_configs: Vec<AssetConfig>,
}

pub struct AssetConfig {
    asset: ContractId,
    price_feed: ContractId,
    decimals: u8,
    borrow_collateral_factor: u64, // decimals: 4
    liquidate_collateral_factor: u64, // decimals: 4
    liquidation_penalty: u64, // decimals: 4
    supply_cap: u64, // decimals: asset decimals
}
