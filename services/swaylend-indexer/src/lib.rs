extern crate alloc;
use fuel_indexer_utils::prelude::*;

#[indexer(manifest = "swaylend_indexer.manifest.yaml")]
pub mod swaylend_indexer_index_mod {

    fn handle_asset_collateral_event(event: CollateralConfigurationEvent, block: BlockData) {
        let entry = CollateralConfigurationEntity {
            id: uid(&event.configuration.asset_id.0),
            asset_id: event.configuration.asset_id.0.into(),
            price_feed: event.configuration.price_feed,
            decimals: event.configuration.decimals,
            borrow_collateral_factor: event.configuration.borrow_collateral_factor,
            liquidate_collateral_factor: event.configuration.liquidate_collateral_factor,
            liquidation_penalty: event.configuration.liquidation_penalty,
            supply_cap: event.configuration.supply_cap,
            paused: event.configuration.paused,
        };
        entry.save();

        let height = block.height;
        info!(
            "⚡️ Swaylend (height: {height}): 💰 AssetCollateralEvent: \n{:#?}",
            entry
        );
    }

    fn handle_user_basic_event(event: UserBasicEvent, block: BlockData) {
        let entry = UserBasicEntity {
            id: uid(&event.address),
            address: event.address,
            principal_absolute: event.user_basic.principal.value,
            principal_negative: event.user_basic.principal.negative,
            base_tracking_index: event.user_basic.base_tracking_index,
            base_tracking_accrued: event.user_basic.base_tracking_accrued,
            reward_claimed: event.user_basic.reward_claimed,
        };
        entry.save();

        let height = block.height;
        info!(
            "⚡️ Swaylend (height: {height}): 🕺 UserBasicEvent: \n{:#?}",
            entry
        );
    }

    fn handle_user_collateral_event(event: UserCollateralEvent, block: BlockData) {
        let entry = UserCollateralEntity {
            id: uid([event.address.into(), event.asset_id.0].concat()),
            address: event.address,
            asset_id: event.asset_id.0.into(),
            amount: event.amount,
        };
        entry.save();

        let height = block.height;
        info!(
            "⚡️ Swaylend (height: {height}): ✨ UserCollateralEvent: \n{:#?}",
            entry
        );
    }
}
