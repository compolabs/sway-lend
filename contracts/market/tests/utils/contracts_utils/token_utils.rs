use fuels::types::{AssetId, Bits256};

pub struct Asset {
    // pub config: DeployTokenConfig,
    pub bits256: Bits256,
    // pub contract_id: ContractId,
    pub asset_id: AssetId,
    // pub instance: Option<TokenContract<WalletUnlocked>>,
    pub default_price: u64,
    pub decimals: u64,
    pub symbol: String,
    pub coingeco_id: String,
    // pub asset_config: AssetConfig,
}
