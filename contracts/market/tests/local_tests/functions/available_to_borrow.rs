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
    let address = Address::from(wallets[0].address());
    let uni = assets.get("UNI").unwrap();
    let usdc = assets.get("USDC").unwrap();

    // ==================== Set oracle prices ====================
    let amount = parse_units(1, 9); //1 USDC = $1
    oracle_abi_calls::set_price(&oracle, usdc.contract_id, amount).await;
    let res = oracle_abi_calls::get_price(&oracle, usdc.contract_id).await;
    assert!(res.price == amount);

    let amount = parse_units(5, 9); //1 UNI = $5
    oracle_abi_calls::set_price(&oracle, uni.contract_id, amount).await;
    let res = oracle_abi_calls::get_price(&oracle, uni.contract_id).await;
    assert!(res.price == amount);

    let contracts = oracle_abi_calls::get_as_settable_contract(&oracle);

    let amount = 40_000_000_000;
    market_abi_calls::supply_collateral(&market, uni.asset_id, amount)
        .await
        .unwrap();

    let res = market_abi_calls::get_user_collateral(&market, address, uni.contract_id).await;
    assert!(res == amount);

    market_abi_calls::debug_increment_timestamp(&market).await;

    let res = market_abi_calls::available_to_borrow(&market, &contracts, address).await;
    assert!(res > 0);
}
