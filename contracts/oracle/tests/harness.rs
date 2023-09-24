use std::str::FromStr;

use crate::abigen_bindings::oracle_contract_mod;
use dotenv::dotenv;
use fuels::{prelude::*, types::Bits256};
use serde::Deserialize;
abigen!(Contract(
    name = "OracleContract",
    abi = "out/debug/oracle-abi.json"
));

const RPC: &str = "beta-4.fuel.network";
const ORACLE_ADDRESS: &str = "0xb19e156a8a6cc6d7fc2831c31c65f6bc10b8a4a80f42cbdbeb46c23f3851105e";
#[derive(Deserialize)]
struct TokenConfig {
    asset_id: String,
    // name: String,
    symbol: String,
    coingeco_id: String,
    default_price: u64,
    // decimals: u64,
}

#[tokio::test]
async fn sync_prices() {
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("SECRET").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));
    let token_configs: Vec<TokenConfig> =
        serde_json::from_str(&std::fs::read_to_string("tests/tokens.json").unwrap()).unwrap();

    let id = ContractId::from_str(ORACLE_ADDRESS).unwrap();
    let instance = OracleContract::new(id, wallet.clone());

    let c = reqwest::Client::new();
    let req = "https://api.coingecko.com/api/v3/simple/price?ids=usd-coin%2Cethereum%2Cchainlink%2Cbitcoin%2Cuniswap%2Ccompound-governance-token&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false&precision=9";
    let body = c.get(req).send().await.unwrap().text().await.unwrap();
    let responce: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
    let mut prices: Vec<(oracle_contract_mod::AssetId, u64)> = vec![];
    for config in &token_configs {
        let bits256 = Bits256::from_hex_str(&config.asset_id).unwrap();
        let price = match responce[config.coingeco_id.clone()]["usd"].as_f64() {
            Some(p) => (p * 10f64.powf(9f64)).round() as u64,
            _ => (config.default_price as f64 * 10f64.powf(9f64)) as u64,
        };
        prices.push((oracle_contract_mod::AssetId { value: bits256 }, price));
    }

    instance
        .methods()
        .set_prices(prices)
        .tx_params(TxParameters::default().with_gas_price(1))
        .call()
        .await
        .unwrap();

    for config in &token_configs {
        let bits256 = oracle_contract_mod::AssetId {
            value: Bits256::from_hex_str(&config.asset_id).unwrap(),
        };
        let price = instance
            .methods()
            .get_price(bits256)
            .tx_params(TxParameters::default().with_gas_price(1))
            .call()
            .await
            .unwrap()
            .value
            .price;
        let price = price as f64 / 10f64.powf(9f64);
        println!("price of {} = ${price}", config.symbol);
    }
}

// #[tokio::test]
async fn deploy() {
    if ORACLE_ADDRESS != "" {
        return;
    }
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("SECRET").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let bin_path = "out/debug/oracle.bin";
    let tx_params = TxParameters::default()
        .with_gas_price(1)
        .with_gas_limit(10_000_000);
    let configurables = OracleContractConfigurables::new().with_ADMIN(wallet.address().into());
    let config = LoadConfiguration::default().with_configurables(configurables);
    let id = Contract::load_from(bin_path, config)
        .unwrap()
        .deploy(&wallet, tx_params)
        .await
        .unwrap();

    let oracle = OracleContract::new(id, wallet.clone());
    println!("The oracle has been deployed {}", oracle.contract_id().hash);
}
