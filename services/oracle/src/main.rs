use dotenv::dotenv;
use fuels::prelude::{abigen, Address, Bech32ContractId, ContractId, Provider, WalletUnlocked};
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;
use std::{env, str::FromStr, thread::sleep, time::Duration};

mod utils;
use utils::{oracle_abi_calls, print_swaygang_sign::print_swaygang_sign};

abigen!(Contract(
    name = "OracleContract",
    abi = "src/artefacts/oracle/oracle-abi.json"
));

const RPC: &str = "node-beta-2.fuel.network";
const ORACLE_ADDRESS: &str = "0xde764394c83bb3c8a3aec5f75f383ff86e64728964fab4469df5910ca01b1a59";

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

    let bech32_id = Bech32ContractId::from(ContractId::from_str(ORACLE_ADDRESS).unwrap());
    let market = OracleContract::new(bech32_id, wallet.clone());
    //discord
    let token = env::var("DISCORD_TOKEN").expect("❌ Expected a token in the environment");
    let client = Client::builder(&token, GatewayIntents::default())
        .await
        .expect("Err creating client");
    let channel_id = env::var("CHANNEL_ID").expect("❌ Expected a channel id in the environment");

    let channel = ChannelId(channel_id.parse::<u64>().unwrap());

    print_swaygang_sign("✅ Oracle is alive");
    loop {
        sleep(Duration::from_secs(60));
    }
}

