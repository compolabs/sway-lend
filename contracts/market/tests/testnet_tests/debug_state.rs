use std::str::FromStr;

use crate::utils::contracts_utils::market_utils::MarketContract;
use crate::utils::contracts_utils::token_utils::load_tokens;
use crate::utils::{debug_state, print_title};
use fuels::accounts::wallet::WalletUnlocked;
use fuels::prelude::Provider;
use fuels::types::ContractId;

// Multiplies all values by this number
// It is necessary in order to test how the protocol works with large amounts
const RPC: &str = "beta-4.fuel.network";
const MARKET_ADDRESS: &str = "0x06e9b35a0d196ca4358757c934a98da1d5874c4d91a8eff41fe940029dba2fa7";
const ORACLE_ADDRESS: &str = "0x633fad7666495c53daa41cc329b78a554f215af4b826671ee576f2a30096999d";

#[tokio::test]
async fn debug_state_test() {
    dotenv::dotenv().ok();

    print_title("Debug state");
    //--------------- WALLETS ---------------
    let provider = Provider::connect(RPC).await.unwrap();

    let admin_pk = std::env::var("ADMIN").unwrap().parse().unwrap();
    let admin = WalletUnlocked::new_from_private_key(admin_pk, Some(provider.clone()));

    let alice_pk = std::env::var("ALICE").unwrap().parse().unwrap();
    let alice = WalletUnlocked::new_from_private_key(alice_pk, Some(provider.clone()));

    let bob_pk = std::env::var("BOB").unwrap().parse().unwrap();
    let bob = WalletUnlocked::new_from_private_key(bob_pk, Some(provider.clone()));

    let chad_pk = std::env::var("CHAD").unwrap().parse().unwrap();
    let chad = WalletUnlocked::new_from_private_key(chad_pk, Some(provider.clone()));

    //--------------- ORACLE ---------------
    let oracle_id = ContractId::from_str(ORACLE_ADDRESS).unwrap();

    //--------------- MARKET ---------------
    let id = ContractId::from_str(MARKET_ADDRESS).unwrap();
    let market = MarketContract::new(id, admin.clone());

    //--------------- TOKENS ---------------
    let (assets, _) = load_tokens("tests/artefacts/tokens.json", oracle_id).await;
    let usdc = assets.get("USDC").unwrap();
    let uni = assets.get("UNI").unwrap();

    debug_state(&market, &vec![admin, alice, bob, chad], usdc, uni).await;
}
