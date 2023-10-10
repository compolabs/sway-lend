use std::str::FromStr;

use crate::utils::contracts_utils::market_utils::MarketContract;
use crate::utils::contracts_utils::token_utils::load_tokens;
use crate::utils::{debug_state, print_title};
use fuels::accounts::wallet::WalletUnlocked;
use fuels::prelude::Provider;
use fuels::types::ContractId;

const RPC: &str = "beta-4.fuel.network";
const MARKET_ADDRESS: &str = "0x9d1c482f1ccf2be50e490a0e25c3e441d05358758a010325ea0eb50fcba20fe5";
const ORACLE_ADDRESS: &str = "0x8f7a76602f1fce4e4f20135a0ab4d22b3d9a230215ccee16c0980cf286aaa93c";

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
