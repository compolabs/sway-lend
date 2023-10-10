use std::str::FromStr;

use crate::utils::contracts_utils::market_utils::{market_abi_calls, MarketContract};
use crate::utils::contracts_utils::oracle_utils::OracleContract;
use crate::utils::contracts_utils::token_utils::load_tokens;
use crate::utils::print_title;
use fuels::accounts::wallet::WalletUnlocked;
use fuels::accounts::ViewOnlyAccount;
use fuels::prelude::Provider;
use fuels::types::{Address, ContractId};

const RPC: &str = "beta-4.fuel.network";
const MARKET_ADDRESS: &str = "0x9d1c482f1ccf2be50e490a0e25c3e441d05358758a010325ea0eb50fcba20fe5";
const ORACLE_ADDRESS: &str = "0x8f7a76602f1fce4e4f20135a0ab4d22b3d9a230215ccee16c0980cf286aaa93c";

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
