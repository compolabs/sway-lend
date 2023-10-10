use dotenv::dotenv;
use fuels::prelude::{Address, ContractId, Provider, ViewOnlyAccount, WalletUnlocked};
use fuels::types::{AssetId, Bits256};
use src20_sdk::{token_factory_abi_calls, TokenFactoryContract};
use std::{collections::HashMap, env, str::FromStr};

mod utils;
use utils::indexer_utils::{fetch_collateral_configurations, fetch_user_basics};
use utils::market_utils::{market_abi_calls::*, MarketContract};
use utils::oracle_utils::oracle_abi_calls::get_price;
use utils::oracle_utils::OracleContract;
use utils::print_swaygang_sign::print_swaygang_sign;

const RPC: &str = "beta-4.fuel.network";
const MARKET_ADDRESS: &str = "0x3fffc28bdb0a460263eeda9b56f9c5157c8048c25ed116c3a4e5cee78bb24bb9";
const ORACLE_ADDRESS: &str = "0x8f7a76602f1fce4e4f20135a0ab4d22b3d9a230215ccee16c0980cf286aaa93c";
const FACTORY_ADDRESS: &str = "0xd8c627b9cd9ee42e2c2bd9793b13bc9f8e9aad32e25a99ea574f23c1dd17685a";
const USDC_ASSET_ID: &str = "0x8bf7951ea3222fe0bae9b811c2b142a1ff417361dcf7457855ed477d2d9a8550";
pub const INDEXER_URL: &str =
    "https://spark-indexer.spark-defi.com/api/sql/composabilitylabs/swaylend_indexer";

#[tokio::main]
async fn main() {
    // Wallet
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = env::var("SECRET").unwrap();
    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider));

    // Swaylend Market Contract
    let id = ContractId::from_str(MARKET_ADDRESS).unwrap();
    let market = MarketContract::new(id.clone(), wallet.clone());

    // Oracle Contract
    let id = ContractId::from_str(ORACLE_ADDRESS).unwrap();
    let oracle = OracleContract::new(id, wallet.clone());

    // Token Factory Contract
    let id = ContractId::from_str(FACTORY_ADDRESS).unwrap();
    let factory = TokenFactoryContract::new(id, wallet.clone());
    let usdc = AssetId::from_str(USDC_ASSET_ID).unwrap();
    print_swaygang_sign("✅ SwayLend liquidator is alive");

    loop {
        // Collateral configurations and prices update
        let collateral_configs = &fetch_collateral_configurations().await.data[0];

        let mut prices: HashMap<String, u64> = HashMap::new();
        for config in collateral_configs {
            let asset_id = Bits256::from_hex_str(&("0x".to_owned() + &config.asset_id)).unwrap();
            let price = get_price(&oracle, asset_id).await.price;
            prices.insert(config.asset_id.clone(), price);
        }
        let user_basics_res = &fetch_user_basics().await.data;
        if user_basics_res.len() != 0 {
            let user_basics = &user_basics_res[0];
            // Asorb
            for user_basic in user_basics {
                let address = Address::from_str(&user_basic.address).unwrap();

                if is_liquidatable(&market, &[&oracle], address).await {
                    absorb(&market, &[&oracle], vec![address]).await.unwrap();
                    println!("🔥 0x{} has been liquidated.", address.to_string());
                }
            }
        }

        // Buy collateral
        for config in collateral_configs {
            let asset_id = Bits256::from_hex_str(&("0x".to_owned() + &config.asset_id)).unwrap();
            let reservs = get_collateral_reserves(&market, asset_id).await;
            let amount =
                collateral_value_to_sell(&market, &[&oracle], asset_id, reservs.value).await;
            if !reservs.negative && amount > 0 {
                let recipient = wallet.address().into();
                if wallet.get_asset_balance(&usdc).await.unwrap() < amount {
                    token_factory_abi_calls::mint(&factory, recipient, "USDC", amount)
                        .await
                        .unwrap();
                }
                let res =
                    buy_collateral(&market, &[&oracle], usdc, amount, asset_id, 0, recipient).await;
                if res.is_ok() {
                    let unit_amount = amount as f64 / 10f64.powf(6.0);
                    println!("🤑 Bought the equivalent of ${unit_amount} worth of collateral");
                } else {
                    println!("⛔️ Cannot buy collateral = {:?}", res.err().unwrap());
                }
            }
        }
    }
}
