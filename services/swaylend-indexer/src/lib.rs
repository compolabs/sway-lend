extern crate alloc;
use fuel_indexer_utils::prelude::*;

#[indexer(manifest = "swaylend_indexer.manifest.yaml")]
pub mod swaylend_indexer_index_mod {

    fn handle_block(block: BlockData) {
        let height = block.height;
        let txs = block.transactions.len();
        info!("🧱 Block height: {height} | transacrions: {txs}");
    }

    fn handle_asset_collateral_event(event: AssetCollateralEvent) {
        info!("💰 AssetCollateralEvent: {:#?}", event);
    }

    fn handle_user_basic_event(event: UserBasicEvent) {
        info!("👩🏻‍🚀 UserBasicEvent: {:#?}", event);
    }

    fn handle_user_collateral_event(event: UserCollateralEvent) {
        info!("✨ UserCollateralEvent: {:#?}", event);
    }
}
