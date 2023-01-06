use fuels::tx::{AssetId, ContractId};

use crate::utils::{
    local_tests_utils::{init_wallet, market, oracle_abi_calls},
    number_utils::parse_units,
};

#[tokio::test]
async fn main_test() {
    let (_admin, assets, market, oracle) = market::setup_market().await;
    let usdc = assets.get("USDC").unwrap();
    let uni = assets.get("UNI").unwrap();

    let alice = init_wallet().await;
    let bob = init_wallet().await;
    // let chad = init_wallet().await;

    oracle_abi_calls::set_price(
        &oracle,
        ContractId::from(usdc.instance.as_ref().unwrap().get_contract_id()),
        parse_units(1, usdc.config.decimals),
    )
    .await;
    oracle_abi_calls::set_price(
        &oracle,
        ContractId::from(uni.instance.as_ref().unwrap().get_contract_id()),
        parse_units(5, uni.config.decimals),
    )
    .await;

    // 💰 supply_base	Bob 100$/100USDC
    let _res = market::market_abi_calls::supply_base(
        &market.with_wallet(bob.clone()).unwrap(),
        AssetId::from(*usdc.instance.as_ref().unwrap().get_contract_id().hash()),
        parse_units(100, usdc.config.decimals),
    )
    .await
    .unwrap();

    // 💰 supply_collateral	Alice 200$/40UNI
    // market::market_abi_calls::supply_collateral(
    //     &market.with_wallet(alice.clone()).unwrap(),
    //     AssetId::from(*uni.instance.as_ref().unwrap().get_contract_id().hash()),
    //     parse_units(40, uni.config.decimals),
    // )
    // .await
    // .unwrap();

    // // 💸 withdraw_base	Alice -50$/50USDC
    // market::market_abi_calls::withdraw_base(
    //     &market.with_wallet(alice.clone()).unwrap(),
    //     AssetId::from(*market.get_contract_id().hash()),
    //     parse_units(50, usdc.config.decimals),
    // )
    // .await
    // .unwrap();

    // // 💰 supply_collateral Chad 300$/60UNI
    // market::market_abi_calls::supply_collateral(
    //     &market.with_wallet(chad.clone()).unwrap(),
    //     AssetId::from(*uni.instance.as_ref().unwrap().get_contract_id().hash()),
    //     parse_units(60, uni.config.decimals),
    // )
    // .await
    // .unwrap();

    // // 💰 supply_base Chad 200$/200USDC
    // market::market_abi_calls::supply_base(
    //     &market.with_wallet(chad.clone()).unwrap(),
    //     AssetId::from(*usdc.instance.as_ref().unwrap().get_contract_id().hash()),
    //     parse_units(200, usdc.config.decimals),
    // )
    // .await
    // .unwrap();

    // // 💸 withdraw_base	Alice  -100$/100USDC
    // market::market_abi_calls::withdraw_base(
    //     &market.with_wallet(alice.clone()).unwrap(),
    //     AssetId::from(*market.get_contract_id().hash()),
    //     parse_units(100, usdc.config.decimals),
    // )
    // .await
    // .unwrap();

    // // 📉 *collateral price drops*
    // oracle_abi_calls::set_price(
    //     &oracle,
    //     ContractId::from(uni.instance.as_ref().unwrap().get_contract_id()),
    //     parse_units(5, uni.config.decimals), //TODO 5 -> 4.5
    // )
    // .await;

    // //  🔥 absorb Bob
    // market::market_abi_calls::absorb(
    //     &market.with_wallet(alice.clone()).unwrap(),
    //     vec![Address::from(bob.address())],
    // )
    // .await
    // .unwrap();

    //  🏦 buy_collateral	Bob
    // market::market_abi_calls::buy_collateral(
    //     &market.with_wallet(bob.clone()).unwrap(),
    //     AssetId::from(*usdc.instance.as_ref().unwrap().get_contract_id().hash()),
    //     parse_units(180, usdc.config.decimals),
    //     ContractId::from(uni.instance.as_ref().unwrap().get_contract_id()),
    //     parse_units(1, uni.config.decimals),
    //     Address::from(bob.address())
    // )
    // .await
    // .unwrap();

    // // 💸 withdraw_base	Bob
    // market::market_abi_calls::withdraw_base(
    //     &market.with_wallet(bob.clone()).unwrap(),
    //     AssetId::from(*market.get_contract_id().hash()),
    //     parse_units(100, usdc.config.decimals), //TODO
    // )
    // .await
    // .unwrap();

    // // 💸 withdraw_base	Chad
    // market::market_abi_calls::withdraw_base(
    //     &market.with_wallet(chad.clone()).unwrap(),
    //     AssetId::from(*market.get_contract_id().hash()),
    //     parse_units(100, usdc.config.decimals), //TODO
    // )
    // .await
    // .unwrap();

    // // 💸 withdraw_base	Alice
    // market::market_abi_calls::withdraw_base(
    //     &market.with_wallet(alice.clone()).unwrap(),
    //     AssetId::from(*market.get_contract_id().hash()),
    //     parse_units(100, usdc.config.decimals), //TODO
    // )
    // .await
    // .unwrap();

    // //  💸 withdraw_collateral	Chad
    // market::market_abi_calls::withdraw_collateral(
    //     &market.with_wallet(chad.clone()).unwrap(),
    //     ContractId::from(usdc.instance.as_ref().unwrap().get_contract_id()),
    //     parse_units(100, usdc.config.decimals), //TODO
    // )
    // .await
    // .unwrap();
    //------------------------------------------------------------------------------------
    // let data = market::market_abi_calls::get_market_state(
    //     &market.with_wallet(bob).unwrap(),
    //     AssetId::from(*usdc.instance.as_ref().unwrap().get_contract_id().hash())
    // ).await.unwrap();
    // println!("✅ usdc balance after Bob supply {}",data);
}
