library;

use ::structs::*;

pub struct CollateralConfigurationEvent{configuration: CollateralConfiguration}

pub struct UserBasicEvent{user_basic: UserBasic, address: Address}

pub struct UserCollateralEvent{
    address: Address,
    asset_id: AssetId,
    amount: u64,
}