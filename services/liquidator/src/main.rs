use dotenv::dotenv;
use fuels::{
    prelude::{
        abigen, Address, Bech32ContractId, ContractId, Provider, SettableContract, WalletUnlocked,
    },
    types::Bits256,
};
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;
use std::{collections::HashMap, env, str::FromStr, thread::sleep, time::Duration};

mod utils;
use utils::{market_abi_calls::market_abi_calls, print_swaygang_sign::print_swaygang_sign};

abigen!(
    Contract(
        name = "MarketContract",
        abi = "src/artefacts/market-abi.json"
    ),
    Contract(
        name = "OracleContract",
        abi = "src/artefacts/oracle-abi.json"
    )
);

const RPC: &str = "beta-4.fuel.network";
const INDEXER_URL: &str = "http://localhost:29987/api/sql/composabilitylabs/swaylend_indexer";
const MARKET_ADDRESS: &str = "0x9795996ffca3540819dbe8ace726d4a83e5411bf2177ba7c4ca7e1b5a8df1972";
const ORACLE_ADDRESS: &str = "0x633fad7666495c53daa41cc329b78a554f215af4b826671ee576f2a30096999d";

/*
    id: ID!
    asset_id: AssetId!
    price_feed: ContractId!
    decimals: UInt8!
    borrow_collateral_factor: UInt8!
    liquidate_collateral_factor: UInt8!
    liquidation_penalty: UInt8!
    supply_cap: UInt8!
    paused: Boolean!
*/

#[derive(serde::Deserialize, Debug)]
struct IndexerResponce<T> {
    data: [Vec<T>; 1],
}

#[derive(serde::Deserialize, Debug)]
struct CollateralConfig {
    asset_id: String,
    borrow_collateral_factor: u64,
    decimals: u64,
    id: String,
    liquidate_collateral_factor: u64,
    liquidation_penalty: u64,
    paused: bool,
    price_feed: String,
    supply_cap: u64,
}

#[tokio::main]
async fn main() {
    // contract
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = env::var("SECRET").unwrap();
    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider));

    let bech32_id = Bech32ContractId::from(ContractId::from_str(MARKET_ADDRESS).unwrap());
    let market = MarketContract::new(bech32_id.clone(), wallet.clone());

    // let mut users = Users::new(MarketContract::new(bech32_id, wallet.clone()));
    // users.fetch().await;

    let bech32_id = Bech32ContractId::from(ContractId::from_str(ORACLE_ADDRESS).unwrap());
    let oracle = OracleContract::new(bech32_id, wallet.clone());
    let contracts: [&dyn SettableContract; 1] = [&oracle];
    //discord
    let token = env::var("DISCORD_TOKEN").expect("❌ Expected a token in the environment");
    let discord_client = Client::builder(&token, GatewayIntents::default())
        .await
        .expect("❌ Err creating client");
    let channel_id = env::var("CHANNEL_ID").expect("❌ Expected a channel id in the environment");

    let channel = ChannelId(channel_id.parse::<u64>().unwrap());

    print_swaygang_sign("✅ SwayLend liquidator is alive");
    let reqwest_client = reqwest::Client::new();
    let responce = reqwest_client
        .post(INDEXER_URL)
        .header("Content-Type" , "application/json")
        .body("{\"query\":\"SELECT json_agg(t) FROM (SELECT * FROM composabilitylabs_swaylend_indexer.collateralconfigurationentity) t;\"}")
        .send()
        .await
        .unwrap();
    let collateral_configs: IndexerResponce<CollateralConfig> =
        serde_json::from_str(&responce.text().await.unwrap()).unwrap();
    let collateral_configs = &collateral_configs.data[0];

    let mut prices: HashMap<String, u64> = HashMap::new();
    for config in collateral_configs {
        let asset_id = Bits256::from_hex_str(&("0x".to_owned() + &config.asset_id)).unwrap();
        let price = oracle
            .methods()
            .get_price(asset_id)
            .simulate()
            .await
            .unwrap()
            .value
            .price;
        prices.insert(config.asset_id.clone(), price);
    }
    println!("prices = {:#?}", prices);
    return;
    loop {
        /*
        curl -X POST http://localhost:29987/api/sql/composabilitylabs/swaylend_indexer  \
                          -d  '{"query":"SELECT json_agg(t) FROM (SELECT * FROM composabilitylabs_swaylend_indexer.collateralconfigurationentity) t;"}' \
                          -H "Content-type: application/json"
         */

        // market.methods().coll
        // let mut  i = 0;
        // while  {
        //
        // }
        // oracle.methods().get_price(asset_id)

        // users.fetch().await;
        // let list = users.list.clone();
        // println!("Total users {}", list.len());
        // let mut index = 0;
        // while index < list.len() {
        //     let user = list[index];
        //     let res = market_abi_calls::is_liquidatable(&market, &contracts, user).await;
        //     if res.is_err() {
        //         println!("error = {:?}", res.err());
        //         continue;
        //     }
        //     let is_liquidatable = res.unwrap().value;
        //     if is_liquidatable {
        //         let res = market_abi_calls::absorb(&market, &contracts, vec![user]).await;
        //         if res.is_err() {
        //             println!("error = {:?}", res.err());
        //             continue;
        //         }

        //         // let tx_link =
        //         // format!("https://fuellabs.github.io/block-explorer-v2/transaction/{}");
        //         channel
        //             .say(
        //                 discord_client.cache_and_http.http.clone(),
        //                 format!("🔥 0x{user} has been liquidated."),
        //             )
        //             .await
        //             .unwrap();
        //     }
        //     index += 1;
        // }
        // sleep(Duration::from_secs(10));
    }
}
