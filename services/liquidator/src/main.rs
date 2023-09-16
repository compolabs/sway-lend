use dotenv::dotenv;
use fuels::prelude::{
    abigen, Address, Bech32ContractId, ContractId, Provider, SettableContract, WalletUnlocked,
};
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;
use std::{env, str::FromStr, thread::sleep, time::Duration};

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
const MARKET_ADDRESS: &str = "0x9795996ffca3540819dbe8ace726d4a83e5411bf2177ba7c4ca7e1b5a8df1972";
const ORACLE_ADDRESS: &str = "0x633fad7666495c53daa41cc329b78a554f215af4b826671ee576f2a30096999d";

#[tokio::main]
async fn main() {
    // contract
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = env::var("SECRET").unwrap();
    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider));

    let bech32_id = Bech32ContractId::from(ContractId::from_str(MARKET_ADDRESS).unwrap());
    let market = MarketContract::new(bech32_id.clone(), wallet.clone());

    let mut users = Users::new(MarketContract::new(bech32_id, wallet.clone()));
    users.fetch().await;

    let bech32_id = Bech32ContractId::from(ContractId::from_str(ORACLE_ADDRESS).unwrap());
    let oracle = OracleContract::new(bech32_id, wallet.clone());
    let contracts: [&dyn SettableContract; 1] = [&oracle];
    //discord
    let token = env::var("DISCORD_TOKEN").expect("❌ Expected a token in the environment");
    let client = Client::builder(&token, GatewayIntents::default())
        .await
        .expect("❌ Err creating client");
    let channel_id = env::var("CHANNEL_ID").expect("❌ Expected a channel id in the environment");

    let channel = ChannelId(channel_id.parse::<u64>().unwrap());

    print_swaygang_sign("✅ SwayLend liquidator is alive");
    loop {

        // market.methods().coll
        // let mut  i = 0;
        // while  {
            // 
        // }
        // oracle.methods().get_price(asset_id)

        users.fetch().await;
        let list = users.list.clone();
        // println!("Total users {}", list.len());
        let mut index = 0;
        while index < list.len() {
            let user = list[index];
            let res = market_abi_calls::is_liquidatable(&market, &contracts, user).await;
            if res.is_err() {
                println!("error = {:?}", res.err());
                continue;
            }
            let is_liquidatable = res.unwrap().value;
            if is_liquidatable {
                let res = market_abi_calls::absorb(&market, &contracts, vec![user]).await;
                if res.is_err() {
                    println!("error = {:?}", res.err());
                    continue;
                }

                // let tx_link =
                // format!("https://fuellabs.github.io/block-explorer-v2/transaction/{}");
                channel
                    .say(
                        client.cache_and_http.http.clone(),
                        format!("🔥 0x{user} has been liquidated."),
                    )
                    .await
                    .unwrap();
            }
            index += 1;
        }
        sleep(Duration::from_secs(10));
    }
}

struct Users {
    pub list: Vec<Address>,
    market: MarketContract<WalletUnlocked>,
    last_check_borrowers_amount: u64,
}

impl Users {
    fn new(market: MarketContract<WalletUnlocked>) -> Users {
        Users {
            list: vec![],
            market,
            last_check_borrowers_amount: 0,
        }
    }
    async fn fetch(&mut self) {
        let methods = self.market.methods();
        let amount = methods
            .get_borrowers_amount()
            .simulate()
            .await
            .unwrap()
            .value;
        let mut index = self.last_check_borrowers_amount;
        while index < amount {
            let borrower = methods.get_borrower(index).simulate().await.unwrap().value;
            self.list.push(borrower);
            index += 1;
        }
        self.last_check_borrowers_amount = amount;
    }
}
