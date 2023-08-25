use fuels::{
    accounts::wallet::WalletUnlocked,
    types::{AssetId, ContractId},
};
use src20_sdk::{DeployTokenConfig, TokenContract};


pub struct Asset {
    pub config: DeployTokenConfig,
    pub contract_id: ContractId,
    pub asset_id: AssetId,
    pub instance: Option<TokenContract<WalletUnlocked>>,
    pub default_price: u64,
    pub decimals: u8,
    pub symbol: String,
    pub coingeco_id: String,
}
