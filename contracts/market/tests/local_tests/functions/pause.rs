use fuels::prelude::{TxParameters, ViewOnlyAccount};
use fuels::types::Address;
use src20_sdk::token_factory_abi_calls;

use crate::utils::contracts_utils::market_utils::{
    deploy_market, get_market_config, market_abi_calls, PauseConfiguration,
};
use crate::utils::contracts_utils::oracle_utils::{deploy_oracle, oracle_abi_calls};
use crate::utils::number_utils::parse_units;
use crate::utils::{convert_i64, debug_state, init_tokens, init_wallets, print_title};

// Multiplies all values by this number
// It is necessary in order to test how the protocol works with large amounts

#[tokio::test]
async fn pause_test() {
    print_title("Pause test");
    //--------------- WALLETS ---------------
    let wallets = init_wallets().await;
    let admin = &wallets[0];
    let alice = &wallets[1];
    let bob = &wallets[2];

    let alice_address = Address::from(alice.address());
    let bob_address = Address::from(bob.address());

    //--------------- ORACLE ---------------
    let oracle = deploy_oracle(&admin).await;
    let contracts = oracle_abi_calls::get_as_settable_contract(&oracle);

    //--------------- TOKENS ---------------
    let (assets, asset_configs, factory) = init_tokens(&admin, oracle.contract_id().into()).await;
    let usdc = assets.get("USDC").unwrap();
    let uni = assets.get("UNI").unwrap();

    //--------------- MARKET ---------------

    let market_config = get_market_config(
        admin.address().into(),
        admin.address().into(),
        usdc.bits256,
        usdc.decimals,
        oracle.contract_id().into(),
    );

    // debug step
    let step: Option<u64> = Option::Some(10000);
    let market = deploy_market(&admin, market_config, step).await;

    for config in &asset_configs {
        market
            .methods()
            .add_collateral_asset(config.clone())
            .tx_params(TxParameters::default().with_gas_price(1))
            .call()
            .await
            .unwrap();
    }
    // ==================== Set oracle prices ====================
    for asset in &assets {
        let price = asset.1.default_price * 10u64.pow(9);
        oracle_abi_calls::set_price(&oracle, asset.1.bits256, price).await;
        println!("1 {} = ${}", asset.1.symbol, asset.1.default_price);
    }
    println!("\n");
    debug_state(&market, &wallets, usdc, uni).await;
    // =================================================
    // ==================== Case #0 ====================
    // 👛 Wallet: Bob 🧛
    // 🤙 Call: supply_base
    // 💰 Amount: 400.00 USDC

    let amount = parse_units(400, usdc.decimals);

    // Transfer of 400 USDC to the Bob's wallet
    token_factory_abi_calls::mint(&factory, bob_address, &usdc.symbol, amount)
        .await
        .unwrap();

    let balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Bob calls supply_base
    let inst = market.with_account(bob.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    // Сheck supply balance equal to 400 USDC
    let (supply_balance, _) = market_abi_calls::get_user_supply_borrow(&inst, bob_address).await;
    assert!(supply_balance == amount);

    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #1 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: supply_collateral
    // 💰 Amount: 40.00 UNI ~ $200.00

    let amount = parse_units(40, uni.decimals);

    // Transfer of 40 UNI to the Alice's wallet
    token_factory_abi_calls::mint(&factory, alice_address, &uni.symbol, amount)
        .await
        .unwrap();

    let balance = alice.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == amount);

    // Alice calls supply_collateral
    let inst = market.with_account(alice.clone()).unwrap();
    market_abi_calls::supply_collateral(&inst, uni.asset_id, amount)
        .await
        .unwrap();

    // Сheck supply balance equal to 40 UNI
    let res = market_abi_calls::get_user_collateral(&inst, alice_address, uni.bits256).await;
    assert!(res == amount);

    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #2 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_base
    // 💰 Amount: 150.00 USDC

    let amount = parse_units(150, usdc.decimals);

    // Alice calls withdraw_base
    let inst = market.with_account(alice.clone()).unwrap();

    market_abi_calls::withdraw_base(&inst, &contracts, amount)
        .await
        .unwrap();

    // USDC balance check
    let balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #3 ====================
    // 👛 Wallet: Admin 🗿
    // 🤙 Drop of collateral price
    // 💰 Amount: -10%

    let res = oracle_abi_calls::get_price(&oracle, uni.bits256).await;
    let new_price = (res.price as f64 * 0.9) as u64;
    oracle_abi_calls::set_price(&oracle, uni.bits256, new_price).await;
    let res = oracle_abi_calls::get_price(&oracle, uni.bits256).await;
    assert!(new_price == res.price);

    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #4 ====================
    // 👛 Wallet: Bob 🦹
    // 🤙 Call: absorb
    // 🔥 Target: Alice

    assert!(market_abi_calls::is_liquidatable(&market, &contracts, alice_address).await);

    let inst = market.with_account(bob.clone()).unwrap();
    market_abi_calls::absorb(&inst, &contracts, vec![alice_address])
        .await
        .unwrap();

    //Check if absorb was ok
    let (_, borrow) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;
    assert!(borrow == 0);

    let amount = market_abi_calls::get_user_collateral(&market, alice_address, uni.bits256).await;
    assert!(amount == 0);

    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #5 ====================
    // 👛 Wallet: Bob 🤵
    // 🤙 Call: buy_collateral
    // 💰 Amount: 172.44 USDC

    let inst = market.with_account(bob.clone()).unwrap();
    let reservs =
        convert_i64(market_abi_calls::get_collateral_reserves(&market, uni.bits256).await);
    assert!(reservs > 0);

    let amount = market_abi_calls::collateral_value_to_sell(
        &market,
        &contracts,
        uni.bits256,
        reservs as u64,
    )
    .await;

    // Transfer of amount to the wallet
    token_factory_abi_calls::mint(&factory, bob_address, &usdc.symbol, amount)
        .await
        .unwrap();

    //Сheck balance
    let balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Bob calls buy_collateral
    let addr = bob_address;
    market_abi_calls::buy_collateral(
        &inst,
        &contracts,
        usdc.asset_id,
        amount,
        uni.bits256,
        1,
        addr,
    )
    .await
    .unwrap();

    market_abi_calls::debug_increment_timestamp(&market).await;

    // TODO claim_paused

    // =================================================
    // ==================== Case #6 ====================
    // 👛 Wallet: Admin 🗿
    // 🤙 Call: reset UNI price and pause

    let amount = parse_units(5, 9); //1 UNI = $5
    oracle_abi_calls::set_price(&oracle, uni.bits256, amount).await;
    let res = oracle_abi_calls::get_price(&oracle, uni.bits256).await;
    assert!(res.price == amount);

    let pause_config = PauseConfiguration {
        supply_paused: true,
        withdraw_paused: true,
        absorb_paused: true,
        buy_paused: true,
        claim_paused: true,
    };
    assert!(market_abi_calls::pause(&inst, &pause_config).await.is_err());
    market_abi_calls::pause(&market, &pause_config)
        .await
        .unwrap();

    // =================================================
    // ==================== Case #7 ====================
    // 👛 Wallet: Bob 🧛
    // 🤙 Call: supply_base
    // 💰 Amount: 400.00 USDC

    let amount = parse_units(400, usdc.decimals);

    // Transfer of 400 USDC to the Bob's wallet
    token_factory_abi_calls::mint(&factory, bob_address, &usdc.symbol, amount)
        .await
        .unwrap();

    let balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Bob calls supply_base
    let inst = market.with_account(bob.clone()).unwrap();
    let res = market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .is_err();
    assert!(res);

    // =================================================
    // ==================== Case #8 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: supply_collateral
    // 💰 Amount: 40.00 UNI ~ $200.00

    let amount = parse_units(40, uni.decimals);

    // Transfer of 40 UNI to the Alice's wallet
    token_factory_abi_calls::mint(&factory, alice_address, &uni.symbol, amount)
        .await
        .unwrap();

    let balance = alice.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == amount);

    // Alice calls supply_collateral
    let inst = market.with_account(alice.clone()).unwrap();
    let res = market_abi_calls::supply_collateral(&inst, uni.asset_id, amount)
        .await
        .is_err();
    assert!(res);

    // =================================================
    // ==================== Case #9 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_base
    // 💰 Amount: 150.00 USDC

    let amount = parse_units(150, usdc.decimals);

    // Alice calls withdraw_base
    let inst = market.with_account(alice.clone()).unwrap();

    let res = market_abi_calls::withdraw_base(&inst, &contracts, amount)
        .await
        .is_err();
    assert!(res);

    // =================================================
    // ==================== Case #4 ====================
    // 👛 Wallet: Bob 🦹
    // 🤙 Call: absorb
    // 🔥 Target: Alice

    let inst = market.with_account(bob.clone()).unwrap();
    let res = market_abi_calls::absorb(&inst, &contracts, vec![alice_address])
        .await
        .is_err();
    assert!(res);

    // =================================================
    // ==================== Case #5 ====================
    // 👛 Wallet: Bob 🤵
    // 🤙 Call: buy_collateral
    // 💰 Amount: 172.44 USDC

    // let inst = market.with_account(bob.clone()).unwrap();
    let reservs =
        convert_i64(market_abi_calls::get_collateral_reserves(&market, uni.bits256).await);
    assert!(reservs > 0);

    let amount = market_abi_calls::collateral_value_to_sell(
        &market,
        &contracts,
        uni.bits256,
        reservs as u64,
    )
    .await;

    // Transfer of amount to the wallet
    token_factory_abi_calls::mint(&factory, bob_address, &usdc.symbol, amount)
        .await
        .unwrap();

    // Bob calls buy_collateral
    // let addr = bob_address;
    // let res =
    //     market_abi_calls::buy_collateral(&inst, usdc.asset_id, amount, uni.bits256, 1, addr)
    //         .await
    //         .is_err();
    // assert!(res);
}
