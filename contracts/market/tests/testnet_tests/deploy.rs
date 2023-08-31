use std::{collections::HashMap, str::FromStr};

use dotenv::dotenv;
use fuels::{
    prelude::{AssetId, Provider, WalletUnlocked},
    types::{Bits256, ContractId},
};

use crate::utils::contracts_utils::{
    market_utils::{deploy_market, get_market_config, market_abi_calls, AssetConfig},
    oracle_utils::OracleContract,
    token_utils::{Asset, TokenConfig},
};

const RPC: &str = "beta-4.fuel.network";
const ORACLE_ADDRESS: &str = "0x633fad7666495c53daa41cc329b78a554f215af4b826671ee576f2a30096999d";

#[tokio::test]
async fn deploy() {
    //--------------- WALLET ---------------
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("SECRET").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let oracle_id = ContractId::from_str(ORACLE_ADDRESS).unwrap();
    let oracle = OracleContract::new(oracle_id, wallet.clone());

    let tokens_json = std::fs::read_to_string("tests/artefacts/tokens.json").unwrap();
    let token_configs: Vec<TokenConfig> = serde_json::from_str(&tokens_json).unwrap();

    let mut assets: HashMap<String, Asset> = HashMap::new();
    let mut asset_configs: Vec<AssetConfig> = Vec::new();
    for config in token_configs {
        let bits256 = Bits256::from_hex_str(&config.asset_id).unwrap();
        let symbol = config.symbol;
        assets.insert(
            symbol.clone(),
            Asset {
                bits256,
                asset_id: AssetId::from(bits256.0),
                default_price: config.default_price,
                decimals: config.decimals,
                symbol: symbol.clone(),
                coingeco_id: config.coingeco_id,
            },
        );

        if symbol != "USDC" {
            asset_configs.push(AssetConfig {
                asset: bits256,
                decimals: config.decimals,
                price_feed: oracle.contract_id().into(),
                borrow_collateral_factor: config.borrow_collateral_factor.unwrap(), // decimals: 4
                liquidate_collateral_factor: config.liquidate_collateral_factor.unwrap(), // decimals: 4
                liquidation_penalty: config.liquidation_penalty.unwrap(), // decimals: 4
                supply_cap: config.supply_cap.unwrap(), // decimals: asset decimals
            })
        }
    }
    let usdc = assets.get("USDC").unwrap();

    //--------------- MARKET ---------------
    let market = deploy_market(&wallet).await;
    let market_config = get_market_config(
        wallet.address().into(),
        wallet.address().into(),
        usdc.bits256,
        usdc.decimals,
        oracle.contract_id().into(),
        assets.get("SWAY").unwrap().bits256,
    );

    market_abi_calls::initialize(&market, &market_config, &asset_configs, Option::None)
        .await
        .expect("❌ Cannot initialize market");

    println!("Market contract = {}", market.contract_id().hash(),);
}
