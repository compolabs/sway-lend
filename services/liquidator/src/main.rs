use dotenv::dotenv;
use fuels::prelude::{abigen, Address, Bech32ContractId, ContractId, Provider, WalletUnlocked};
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;
use std::{env, str::FromStr, thread::sleep, time::Duration};

mod utils;
use utils::{print_swaygang_sign::print_swaygang_sign, market_abi_calls};

abigen!(Contract(
    name = "MarketContract",
    abi = "src/artefacts/market/market-abi.json"
));

const RPC: &str = "node-beta-2.fuel.network";
const MARKET_ADDRESS: &str = ""; 

#[tokio::main]
async fn main() {
    // contract
    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };
    dotenv().ok();
    let secret = env::var("SECRET").expect("❌ Expected a account secret in the environment");
    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider));

    let bech32_id = Bech32ContractId::from(ContractId::from_str(MARKET_ADDRESS).unwrap());
    let market = MarketContract::new(bech32_id, wallet.clone());
    //discord
    let token = env::var("DISCORD_TOKEN").expect("❌ Expected a token in the environment");
    let client = Client::builder(&token, GatewayIntents::default())
        .await
        .expect("Err creating client");
    let channel_id = env::var("CHANNEL_ID").expect("❌ Expected a channel id in the environment");

    let channel = ChannelId(channel_id.parse::<u64>().unwrap());

    print_swaygang_sign("✅ SwayLend liquidator is alive");
    loop {
        let users = fetch_users().await;
        for user in users {
            if market_abi_calls::is_liquidatable(&market, user).await {
                market_abi_calls::absorb(&market, vec![user]).await.unwrap();
                channel
                    .say(
                        client.cache_and_http.http.clone(),
                        format!("🔥 {user} has been liquidated."),
                    )
                    .await
                    .unwrap();
            }
        }
        sleep(Duration::from_secs(60));
    }
}

async fn fetch_users() -> Vec<Address> {
    //FIXME
    vec![
        Address::from_str("0x1fd00ead353d0489f766e37f348432b9e12a775fdb11bf574faf22f0d02cbd5c")
            .unwrap(),
    ]
}
