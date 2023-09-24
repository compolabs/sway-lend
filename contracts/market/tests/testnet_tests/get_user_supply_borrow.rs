use std::str::FromStr;

use crate::utils::contracts_utils::market_utils::{market_abi_calls, MarketContract};
use crate::utils::print_title;
use fuels::accounts::wallet::WalletUnlocked;
use fuels::accounts::ViewOnlyAccount;
use fuels::prelude::Provider;
use fuels::types::{Address, ContractId};

// Multiplies all values by this number
// It is necessary in order to test how the protocol works with large amounts
const RPC: &str = "beta-4.fuel.network";
const MARKET_ADDRESS: &str = "0xacf860fcfdfb1cf5ab16d2955143a7875821f6f24087689ae320b22d80d77e06";

#[tokio::test]
async fn get_user_supply_borrow() {
    dotenv::dotenv().ok();

    print_title("Get user supply borrow");
    //--------------- WALLETS ---------------
    let provider = Provider::connect(RPC).await.unwrap();

    let admin_pk = std::env::var("ADMIN").unwrap().parse().unwrap();
    let admin = WalletUnlocked::new_from_private_key(admin_pk, Some(provider.clone()));

    let bob_pk = std::env::var("BOB").unwrap().parse().unwrap();
    let bob = WalletUnlocked::new_from_private_key(bob_pk, Some(provider.clone()));
    let bob_address = Address::from(bob.address());

    println!("bob address = {:?}", bob.address().to_string());

    //--------------- MARKET ---------------
    let id = ContractId::from_str(MARKET_ADDRESS).unwrap();
    let market = MarketContract::new(id, admin.clone());

    let (supply, borrow) = market_abi_calls::get_user_supply_borrow(&market, bob_address).await;
    println!("supply = {supply} borrow = {borrow}");
}
