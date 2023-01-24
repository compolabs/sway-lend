use fuels_types::Address;

use crate::utils::{
    local_tests_utils::{
        market::{self, market_abi_calls},
        oracle::oracle_abi_calls,
    },
    number_utils::parse_units,
};

#[tokio::test]
async fn available_to_borrow() {
    let (wallets, assets, market, oracle) = market::setup_market().await;

    let uni = assets.get("UNI").unwrap();
    let usdc = assets.get("USDC").unwrap();

    let admin = wallets[0].clone();
    let alice = wallets[1].clone();
    let alice_address = Address::from(alice.address());
    let bob = wallets[2].clone();
    let bob_address = Address::from(bob.address());
    let chad = wallets[3].clone();
    let chad_address = Address::from(chad.address());

    // supply_base
    // supply_collateral
    // withdraw_base
    // absorb
    // buy_collateral
    // claim_paused
    //
    // pause on
    //
    // supply_base
    // supply_collateral
    // withdraw_base
    // absorb
    // buy_collateral
    // claim_paused
    //
    // pause off
    //
    // supply_base
    // supply_collateral
    // withdraw_base
    // absorb
    // buy_collateral
    // claim_paused
}
