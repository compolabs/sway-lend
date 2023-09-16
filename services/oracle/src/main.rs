use dotenv::dotenv;
use fuels::{
    prelude::{abigen, ContractId, Provider, WalletUnlocked, BASE_ASSET_ID},
    types::Bits256,
};
use serenity::{model::prelude::ChannelId, prelude::GatewayIntents};
use std::{env, str::FromStr, thread::sleep, time::Duration};
mod utils;
use fuels::accounts::ViewOnlyAccount;
use serde::Deserialize;
use utils::print_swaygang_sign::print_swaygang_sign;

use crate::utils::oracle_abi_calls::oracle_abi_calls::set_prices;

abigen!(Contract(
    name = "OracleContract",
    abi = "src/artefacts/oracle/oracle-abi.json"
));

const RPC: &str = "beta-4.fuel.network";
const ORACLE_ADDRESS: &str = "0x633fad7666495c53daa41cc329b78a554f215af4b826671ee576f2a30096999d";

#[derive(Deserialize)]
struct TokenConfig {
    asset_id: String,
    symbol: String,
    coingeco_id: String,
    default_price: u64,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("SECRET").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));
    let token_configs: Vec<TokenConfig> =
        serde_json::from_str(&std::fs::read_to_string("src/tokens.json").unwrap()).unwrap();

    let id = ContractId::from_str(ORACLE_ADDRESS).unwrap();
    let oracle = OracleContract::new(id, wallet.clone());

    //discord
    let token = env::var("DISCORD_TOKEN").expect("❌ Expected a token in the environment");
    let serenity_client = serenity::prelude::Client::builder(&token, GatewayIntents::default())
        .await
        .expect("❌ Err creating client");
    let channel_id = env::var("CHANNEL_ID").expect("❌ Expected a channel id in the environment");

    let channel = ChannelId(channel_id.parse::<u64>().unwrap());

    print_swaygang_sign("✅ Oracle is alive");
    loop {
        let c = reqwest::Client::new();
        let req = "https://api.coingecko.com/api/v3/simple/price?ids=compound-governance-token%2Cbinancecoin%2Cbitcoin%2Cbinance-usd%2Cusd-coin%2Ctether%2Cuniswap%2Cethereum%2Cchainlink&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false&precision=9";
        let body = c.get(req).send().await.unwrap().text().await.unwrap();
        let responce: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
        let mut prices: Vec<(Bits256, u64)> = vec![];
        let mut message = String::from("🪬 Price oracle update\n");
        for config in &token_configs {
            let price =
            //  if config.symbol == "UNI" {
            //     (3.92 * 10f64.powf(9f64)) as u64
            // } else {
                match responce[config.coingeco_id.clone()]["usd"].as_f64() {
                Some(p) => (p * 10f64.powf(9f64)).round() as u64,
                _ => config.default_price,
            // }
        };
            prices.push((Bits256::from_hex_str(&config.asset_id).unwrap(), price));
            let unit_price = price as f64 / 10f64.powf(9f64);
            message += format!("1 {} = ${unit_price}\n", config.symbol).as_str();
        }
        let res = set_prices(&oracle, prices).await;
        if res.is_ok() {
            message += format!("\n⛽️ Gas used: {}", res.unwrap().gas_used).as_str();
            let balance = wallet.get_asset_balance(&BASE_ASSET_ID).await.unwrap();
            message += format!("\n⚖️ Balance: {} ETH", balance as f64 / 10f64.powf(9f64)).as_str();
            message += format!("\n👁 Oracle address: {ORACLE_ADDRESS}").as_str();
            message += format!("\n-----------------------------------").as_str();
            println!("{message}");
            channel
                .say(serenity_client.cache_and_http.http.clone(), message)
                .await
                .unwrap();
        } else {
            let message = "❌ Cannot update prices";
            println!("{message}");
            channel
                .say(serenity_client.cache_and_http.http.clone(), message)
                .await
                .unwrap();
        }

        sleep(Duration::from_secs(5 * 60));
    }
}
