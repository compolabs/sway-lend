use fuels::prelude::CallParameters;
use fuels::tx::{Address, AssetId, ContractId};

use crate::utils::local_tests_utils::market::market_abi_calls;
use crate::utils::local_tests_utils::{print_balances, token_abi_calls};
use crate::utils::{
    local_tests_utils::{market, oracle_abi_calls},
    number_utils::parse_units,
};

#[tokio::test]
async fn main_test() {
    let (walets, assets, market, oracle) = market::setup_market().await;
    let pl_asset_id = AssetId::from(*market.get_contract_id().hash());

    let usdc = assets.get("USDC").unwrap();
    let usdc_asset_id = AssetId::from(*usdc.instance.as_ref().unwrap().get_contract_id().hash());
    let uni = assets.get("UNI").unwrap();
    let uni_asset_id = AssetId::from(*uni.instance.as_ref().unwrap().get_contract_id().hash());
    let admin = walets[0].clone();
    let alice = walets[1].clone();
    let bob = walets[2].clone();
    // println!(
    //     "Admin {}\nAlice{}\nBob{}\n",
    //     admin.address(),
    //     alice.address(),
    //     bob.address()
    // );
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

    let bob_usdc_inst = usdc
        .instance
        .as_ref()
        .unwrap()
        .with_wallet(bob.clone())
        .unwrap();

    token_abi_calls::mint(&bob_usdc_inst).await;
    assert!(
        bob.get_asset_balance(&usdc_asset_id).await.unwrap() == usdc.config.mint_amount * 1_000_000
    );
    // 💰 supply_base	Bob 100$/100USDC
    let _res = market::market_abi_calls::supply_base(
        &market.with_wallet(bob.clone()).unwrap(),
        usdc_asset_id,
        parse_units(100, usdc.config.decimals),
    )
    .await
    .unwrap();
    assert!(bob.get_asset_balance(&pl_asset_id).await.unwrap() == 100_000_000);

    let alice_uni_inst = uni
        .instance
        .as_ref()
        .unwrap()
        .with_wallet(alice.clone())
        .unwrap();

    token_abi_calls::mint(&alice_uni_inst).await;

    assert!(
        alice.get_asset_balance(&uni_asset_id).await.unwrap()
            == uni.config.mint_amount * 1_000_000_000
    );

    // 💰 supply_collateral	Alice 200$/40UNI
    let _res = market::market_abi_calls::supply_collateral(
        &market.with_wallet(alice.clone()).unwrap(),
        uni_asset_id,
        parse_units(40, uni.config.decimals),
    )
    .await
    .unwrap();
    let res = &market
        .with_wallet(alice.clone())
        .unwrap()
        .methods()
        .get_user_collateral(
            Address::from(alice.address()),
            ContractId::from(uni.instance.as_ref().unwrap().get_contract_id()),
        )
        .simulate()
        .await
        .unwrap();

    assert!(res.value == parse_units(40, uni.config.decimals));

    // 💸 withdraw_base	Alice -50$/50USDC
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
