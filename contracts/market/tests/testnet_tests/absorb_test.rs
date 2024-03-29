use std::str::FromStr;

use crate::utils::contracts_utils::market_utils::{market_abi_calls, MarketContract};
use crate::utils::contracts_utils::oracle_utils::OracleContract;
use crate::utils::print_title;
use fuels::accounts::wallet::WalletUnlocked;
use fuels::accounts::ViewOnlyAccount;
use fuels::prelude::Provider;
use fuels::types::{Address, ContractId};

const RPC: &str = "beta-4.fuel.network";
const MARKET_ADDRESS: &str = "0x3fffc28bdb0a460263eeda9b56f9c5157c8048c25ed116c3a4e5cee78bb24bb9";
const ORACLE_ADDRESS: &str = "0x8f7a76602f1fce4e4f20135a0ab4d22b3d9a230215ccee16c0980cf286aaa93c";

#[tokio::test]
async fn absorb_test() {
    dotenv::dotenv().ok();

    print_title("🔥 Absorb");
    //--------------- WALLETS ---------------
    let provider = Provider::connect(RPC).await.unwrap();

    let admin_pk = std::env::var("ADMIN").unwrap().parse().unwrap();
    let admin = WalletUnlocked::new_from_private_key(admin_pk, Some(provider.clone()));

    let alice_pk = std::env::var("ALICE").unwrap().parse().unwrap();
    let alice = WalletUnlocked::new_from_private_key(alice_pk, Some(provider.clone()));
    let alice_address = Address::from(alice.address());

    println!("Alice address = {:?}", alice.address().to_string());
    //--------------- ORACLE ---------------
    let id = ContractId::from_str(ORACLE_ADDRESS).unwrap();
    let oracle = OracleContract::new(id, admin.clone());

    //--------------- MARKET ---------------
    let id = ContractId::from_str(MARKET_ADDRESS).unwrap();
    let market = MarketContract::new(id, admin.clone());

    let value = market_abi_calls::is_liquidatable(&market, &[&oracle], alice_address).await;
    println!("Alice is_liquidatable = {}", value);
}
