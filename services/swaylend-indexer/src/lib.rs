extern crate alloc;
use fuel_indexer_utils::prelude::*;

#[indexer(manifest = "swaylend_indexer.manifest.yaml")]
pub mod swaylend_indexer_index_mod {

    fn handle_block(block: BlockData) {
        let height = block.height;
        let txs = block.transactions.len();
        info!("🧱 Block height: {height} | transacrions: {txs}");
    }
}
