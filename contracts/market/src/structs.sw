// SPDX-License-Identifier: BUSL-1.1
library;
// use signed_integers::i64::I64;
use i64::*;

const SCALE_18: u64 = 1_000_000_000_000_000_000; // 1e18

pub struct MarketConfiguration {
    governor: Address,
    pause_guardian: Address,
    base_token: AssetId,
    base_token_decimals: u64,
    base_token_price_feed: ContractId,
    kink: u64, // decimals: 18
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
    reward_token: AssetId, 
}

pub struct AssetConfig {
    asset: AssetId,
    price_feed: ContractId,
    decimals: u64,
    borrow_collateral_factor: u64, // decimals: 4
    liquidate_collateral_factor: u64, // decimals: 4
    liquidation_penalty: u64, // decimals: 4
    supply_cap: u64, // decimals: asset decimals
}

pub struct PauseConfiguration {
    supply_paused: bool, 
    withdraw_paused: bool, 
    absorb_paused: bool, 
    buy_pause: bool,
    // claim_paused: bool,
}

pub struct UserBasic {
    principal: I64, // decimals: base_asset_decimal
    base_tracking_index: u64,	// decimals: 18
    base_tracking_accrued: u64,	// decimals: native_asset_decimal
    reward_claimed: u64,	// decimals: native_asset_decimal
}

impl UserBasic {
    pub fn default() -> Self {
        UserBasic {
            principal: I64::new(),
            base_tracking_index: 0,
            base_tracking_accrued: 0,
            reward_claimed: 0,
        }
     }
}

pub struct MarketBasics {
    base_supply_index: u64,// decimals	18
    base_borrow_index: u64,// decimals	18
    tracking_supply_index: u64,// decimals	18
    tracking_borrow_index: u64,// decimals	18
    total_supply_base: u64,// decimals	base_asset_decimal
    total_borrow_base: u64,// decimals	base_asset_decimal
    last_accrual_time: u64,
}

impl MarketBasics {
    pub fn default() -> Self {
        MarketBasics {
            base_supply_index: SCALE_18,
            base_borrow_index: SCALE_18,
            tracking_supply_index: SCALE_18,
            tracking_borrow_index: SCALE_18,
            total_supply_base: 0,
            total_borrow_base: 0,
            last_accrual_time: 0,
        }
     }
}

pub enum Error {
    AlreadyInitialized: (),
    Paused: (),
    Unauthorized: (),
    InsufficientReserves: (),
    NotLiquidatable: (),
    NotForSale: (),
    TooMuchSlippage: (),
    SupplyCapExceeded: (),
    NotCollateralized: (),
    BorrowTooSmall: (),
    NotPermitted: (),
    InvalidPayment: (),
    DebuggingDisabled: (),
}

