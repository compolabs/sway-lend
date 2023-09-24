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
const MARKET_ADDRESS: &str = "0xacf860fcfdfb1cf5ab16d2955143a7875821f6f24087689ae320b22d80d77e06";
const ORACLE_ADDRESS: &str = "0xb19e156a8a6cc6d7fc2831c31c65f6bc10b8a4a80f42cbdbeb46c23f3851105e";

#[tokio::test]
async fn withdraw_collateral() {
    dotenv::dotenv().ok();

    print_title("Withdraw collateral");
    //--------------- WALLETS ---------------
    let provider = Provider::connect(RPC).await.unwrap();

    let admin_pk = std::env::var("ADMIN").unwrap().parse().unwrap();
    let admin = WalletUnlocked::new_from_private_key(admin_pk, Some(provider.clone()));

    let chad_pk = std::env::var("CHAD").unwrap().parse().unwrap();
    let chad = WalletUnlocked::new_from_private_key(chad_pk, Some(provider.clone()));
    let chad_address = Address::from(chad.address());

    println!("chad address = {:?}", chad.address().to_string());

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
        market_abi_calls::get_user_collateral(&market, chad_address, uni.bits256).await;
    println!("collateral = {:?}", collateral);

    market_abi_calls::withdraw_collateral(
        &market.with_account(chad).unwrap(),
        &[&oracle],
        uni.bits256,
        collateral,
    )
    .await
    .unwrap();

    let collateral =
        market_abi_calls::get_user_collateral(&market, chad_address, uni.bits256).await;
    println!("collateral = {:?}", collateral);
}
