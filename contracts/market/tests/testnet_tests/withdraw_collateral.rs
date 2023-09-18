use std::str::FromStr;

use crate::utils::contracts_utils::market_utils::{market_abi_calls, MarketContract};
use crate::utils::contracts_utils::oracle_utils::OracleContract;
use crate::utils::contracts_utils::token_utils::load_tokens;
use crate::utils::print_title;
use fuels::accounts::wallet::WalletUnlocked;
use fuels::accounts::ViewOnlyAccount;
use fuels::prelude::Provider;
use fuels::types::{Address, ContractId};

// Multiplies all values by this number
// It is necessary in order to test how the protocol works with large amounts
const RPC: &str = "beta-4.fuel.network";
const MARKET_ADDRESS: &str = "0x06e9b35a0d196ca4358757c934a98da1d5874c4d91a8eff41fe940029dba2fa7";
const ORACLE_ADDRESS: &str = "0x633fad7666495c53daa41cc329b78a554f215af4b826671ee576f2a30096999d";

#[tokio::test]
async fn withdraw_collateral() {
    dotenv::dotenv().ok();

    print_title("Withdraw collateral");
    //--------------- WALLETS ---------------
    let provider = Provider::connect(RPC).await.unwrap();

    let admin_pk = std::env::var("ADMIN").unwrap().parse().unwrap();
    let admin = WalletUnlocked::new_from_private_key(admin_pk, Some(provider.clone()));

    let alice_pk = std::env::var("CHAD").unwrap().parse().unwrap();
    let alice = WalletUnlocked::new_from_private_key(alice_pk, Some(provider.clone()));
    let alice_address = Address::from(alice.address());

    println!("alice address = {:?}", alice.address().to_string());

    //--------------- MARKET ---------------
    let id = ContractId::from_str(MARKET_ADDRESS).unwrap();
    let market = MarketContract::new(id, admin.clone());

    //--------------- ORACLE ---------------
    let id = ContractId::from_str(ORACLE_ADDRESS).unwrap();
    let oracle = OracleContract::new(id, admin.clone());

    //--------------- Tokens ---------------
    let (assets, _) = load_tokens("tests/artefacts/tokens.json", id).await;
    let uni = assets.get("UNI").unwrap();

    let collateral =
        market_abi_calls::get_user_collateral(&market, alice_address, uni.bits256).await;
    println!("collateral = {:?}", collateral);

    market_abi_calls::withdraw_collateral(
        &market.with_account(alice).unwrap(),
        &[&oracle],
        uni.bits256,
        collateral,
    )
    .await
    .unwrap();

    let collateral =
        market_abi_calls::get_user_collateral(&market, alice_address, uni.bits256).await;
    println!("collateral = {:?}", collateral);
}
