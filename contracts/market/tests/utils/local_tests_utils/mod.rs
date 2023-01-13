pub mod market;
pub mod oracle;
pub mod token;

use crate::utils::number_utils::parse_units;
use fuels::signers::WalletUnlocked;
use fuels::tx::AssetId;
use fuels::tx::ContractId;

use self::token::TokenContract;

pub struct DeployTokenConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub mint_amount: u64,
}

pub struct Asset {
    pub config: DeployTokenConfig,
    pub contract_id: ContractId,
    pub asset_id: AssetId,
    pub coingeco_id: String,
    pub instance: Option<TokenContract>,
    pub default_price: u64,
}

pub async fn _print_balances(wallet: &WalletUnlocked) {
    
    let balances = wallet.get_balances().await.unwrap();
    println!("{:#?}\n", balances);
}
