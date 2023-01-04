use fuels::tx::AssetId;

use crate::utils::{
    local_tests_utils::{init_wallet, market},
    number_utils::parse_units,
};

#[tokio::test]
async fn main_test() {
    let (_admin, assets, market, _oracle) = market::setup_market().await;
    let usdc = assets.get("USDC").unwrap();
    let uni = assets.get("UNI").unwrap();

    let alice = init_wallet().await;
    let bob = init_wallet().await;
    let chad = init_wallet().await;

    // 💰 supply_base | Bob
    market::market_abi_calls::supply_base(
        &market.with_wallet(bob).unwrap(),
        AssetId::from(*usdc.instance.as_ref().unwrap().get_contract_id().hash()),
        parse_units(100, usdc.config.decimals),
    )
    .await
    .unwrap();

    //  💰 supply_collateral | Alice
    market::market_abi_calls::supply_collateral(
        &market.with_wallet(alice).unwrap(),
        AssetId::from(*uni.instance.as_ref().unwrap().get_contract_id().hash()),
        parse_units(100, uni.config.decimals),
    )
    .await
    .unwrap();
    // 🤑 borrow
    /*

    📉 *collateral price drops*
    🔥 absorb
    💰 buy_collateral
    💸 withdraw
    💸 withdraw_collateral*/
}
