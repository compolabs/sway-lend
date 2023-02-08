use dotenv::dotenv;
use fuels::prelude::{
    abigen, Bech32ContractId, ContractId, Provider, WalletUnlocked, BASE_ASSET_ID,
};
use serenity::{model::prelude::ChannelId, prelude::GatewayIntents};
use std::{env, fs::read_to_string, str::FromStr, thread::sleep, time::Duration};
mod utils;
use utils::print_swaygang_sign::print_swaygang_sign;

use crate::utils::oracle_abi_calls::oracle_abi_calls::set_prices;

abigen!(Contract(
    name = "OracleContract",
    abi = "src/artefacts/oracle/oracle-abi.json"
));

const RPC: &str = "node-beta-2.fuel.network";
const ORACLE_ADDRESS: &str = "0x4bf2826201fb74fc479a6a785cb70f2ce8e45b67010acfd47906993d130a21ff";

#[tokio::main]
async fn main() {
    let deploy_config_json_str = read_to_string("src/artefacts/tokens.json")
        .expect("Should have been able to read the file");
    let assets: serde_json::Value = serde_json::from_str(deploy_config_json_str.as_str()).unwrap();
    let assets = assets.as_array().unwrap();
    // contract
    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };
    dotenv().ok();
    let secret = env::var("SECRET").expect("❌ Expected a account secret in the environment");
    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider));

    let bech32_id = Bech32ContractId::from(ContractId::from_str(ORACLE_ADDRESS).unwrap());
    let oracle = OracleContract::new(bech32_id, wallet.clone());
    //discord
    let token = env::var("DISCORD_TOKEN").expect("❌ Expected a token in the environment");
    let serenity_client = serenity::prelude::Client::builder(&token, GatewayIntents::default())
        .await
        .expect("Err creating client");
    let channel_id = env::var("CHANNEL_ID").expect("❌ Expected a channel id in the environment");

    let channel = ChannelId(channel_id.parse::<u64>().unwrap());

    print_swaygang_sign("✅ Oracle is alive");
    loop {
        let c = reqwest::Client::new();
        let req = "https://api.coingecko.com/api/v3/simple/price?ids=compound-governance-token%2Cbinancecoin%2Cbitcoin%2Cbinance-usd%2Cusd-coin%2Ctether%2Cuniswap%2Cethereum%2Cchainlink&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false&precision=9";
        let body = c.get(req).send().await.unwrap().text().await.unwrap();
        let responce: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
        let mut prices: Vec<(ContractId, u64)> = vec![];
        let mut message = String::from("🪬 Price oracle update\n");
        for asset in assets {
            let contract_id = ContractId::from_str(asset["asset_id"].as_str().unwrap())
                .expect("failed to create ContractId address from string");
            let bech32_address = Bech32ContractId::from(contract_id);

            let asset_id = ContractId::from(bech32_address);
            let symbol = asset["symbol"].as_str().unwrap();

            let price = match responce[asset["coingeco_id"].as_str().unwrap()]["usd"].as_f64() {
                Some(p) => (p * 10f64.powf(9f64)).round() as u64,
                _ => (asset["default_price"].as_f64().unwrap() * 10f64.powf(9f64)) as u64,
            };
            prices.push((asset_id, price));

            message += format!("1 {symbol} = ${}\n", price as f64 / 10f64.powf(9f64)).as_str();
        }
        let res = set_prices(&oracle, prices).await;
        if res.is_ok() {
            message += format!("\n⛽️ Gas used: {}", res.unwrap().gas_used).as_str();
            let balance = wallet.get_asset_balance(&BASE_ASSET_ID).await.unwrap();
            message += format!("\n⚖️ Balance: {} ETH", balance as f64 / 10f64.powf(9f64)).as_str();
            message += format!("\n👁 Oracle address: {ORACLE_ADDRESS}").as_str();
            message += format!("\n-----------------------------------").as_str();
            channel
                .say(serenity_client.cache_and_http.http.clone(), message)
                .await
                .unwrap();
        } else {
            channel
                .say(
                    serenity_client.cache_and_http.http.clone(),
                    "❌ Cannot update prices",
                )
                .await
                .unwrap();
        }

        sleep(Duration::from_secs(60));
    }
}
