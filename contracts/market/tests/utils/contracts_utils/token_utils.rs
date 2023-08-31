use fuels::types::{AssetId, Bits256};
use serde::Deserialize;

pub struct Asset {
    pub bits256: Bits256,
    pub asset_id: AssetId,
    pub default_price: u64,
    pub decimals: u64,
    pub symbol: String,
    pub coingeco_id: String,
}

#[derive(Deserialize)]
pub struct TokenConfig {
    pub asset_id: String,
    pub name: String,
    pub symbol: String,
    pub coingeco_id: String,
    pub default_price: u64,
    pub decimals: u64,
    pub borrow_collateral_factor: Option<u64>,
    pub liquidate_collateral_factor: Option<u64>,
    pub liquidation_penalty: Option<u64>,
    pub supply_cap: Option<u64>,
}
