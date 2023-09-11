use std::str::FromStr;

use crate::utils::contracts_utils::market_utils::{market_abi_calls, MarketContract};
use crate::utils::contracts_utils::oracle_utils::{oracle_abi_calls, OracleContract};
use crate::utils::print_title;
use fuels::accounts::wallet::WalletUnlocked;
use fuels::accounts::ViewOnlyAccount;
use fuels::prelude::Provider;
use fuels::types::{Address, ContractId};

// Multiplies all values by this number
// It is necessary in order to test how the protocol works with large amounts
const RPC: &str = "beta-4.fuel.network";
const ORACLE_ADDRESS: &str = "0x633fad7666495c53daa41cc329b78a554f215af4b826671ee576f2a30096999d";
const MARKET_ADDRESS: &str = "0x2679d41c2981bcd6ab69d66f0ae656077efecac2c2dd729f88f5a6457ce1d5f1";

#[tokio::test]
async fn availtable_to_borrow_test() {
    dotenv::dotenv().ok();

    print_title("Available to borrow");
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
    let contracts = oracle_abi_calls::get_as_settable_contract(&oracle);

    //--------------- MARKET ---------------
    let id = ContractId::from_str(MARKET_ADDRESS).unwrap();
    let market = MarketContract::new(id, admin.clone());

    let value = market_abi_calls::available_to_borrow(&market, &contracts, alice_address).await;
    println!("Available to borrow = {}", value);
}
