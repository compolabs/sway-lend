use fuels::{prelude::ViewOnlyAccount, types::Address};

use crate::utils::{
    local_tests_utils::{
        market::{self, market_abi_calls, PauseConfiguration},
        oracle::oracle_abi_calls,
        token::{token_abi_calls, TokenContract},
    },
    number_utils::parse_units,
};

#[tokio::test]
async fn pause() {
    let (wallets, assets, market, oracle) = market::setup_market().await;

    // ==================== Wallets ====================
    let admin = wallets[0].clone();
    let alice = wallets[1].clone();
    let alice_address = Address::from(alice.address());
    let bob = wallets[2].clone();
    let bob_address = Address::from(bob.address());

    // ==================== Assets ====================
    let usdc = assets.get("USDC").unwrap();
    let usdc_instance = TokenContract::new(usdc.contract_id.into(), admin.clone());
    let uni = assets.get("UNI").unwrap();
    let uni_instance = TokenContract::new(uni.contract_id.into(), admin.clone());

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
    // =================================================
    // ==================== Case #0 ====================
    // 👛 Wallet: Bob 🧛
    // 🤙 Call: supply_base
    // 💰 Amount: 400.00 USDC

    let amount = parse_units(400, usdc.config.decimals);

    // Transfer of 400 USDC to the Bob's wallet
    token_abi_calls::mint_and_transfer(&usdc_instance, amount, bob_address).await;

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

    let amount = parse_units(40, uni.config.decimals);

    // Transfer of 40 UNI to the Alice's wallet
    token_abi_calls::mint_and_transfer(&uni_instance, amount, alice_address).await;

    let balance = alice.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == amount);

    // Alice calls supply_collateral
    let inst = market.with_account(alice.clone()).unwrap();
    market_abi_calls::supply_collateral(&inst, uni.asset_id, amount)
        .await
        .unwrap();

    // Сheck supply balance equal to 40 UNI
    let res = market_abi_calls::get_user_collateral(&inst, alice_address, uni.contract_id).await;
    assert!(res == amount);

    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #2 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_base
    // 💰 Amount: 150.00 USDC

    let amount = parse_units(150, usdc.config.decimals);

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

    let res = oracle_abi_calls::get_price(&oracle, uni.contract_id).await;
    let new_price = (res.price as f64 * 0.9) as u64;
    oracle_abi_calls::set_price(&oracle, uni.contract_id, new_price).await;
    let res = oracle_abi_calls::get_price(&oracle, uni.contract_id).await;
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

    let amount =
        market_abi_calls::get_user_collateral(&market, alice_address, uni.contract_id).await;
    assert!(amount == 0);

    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #5 ====================
    // 👛 Wallet: Bob 🤵
    // 🤙 Call: buy_collateral
    // 💰 Amount: 172.44 USDC

    let inst = market.with_account(bob.clone()).unwrap();
    let reservs = market_abi_calls::get_collateral_reserves(&market, uni.contract_id).await;
    assert!(!reservs.negative);

    let reservs = reservs.value;
    let amount =
        market_abi_calls::collateral_value_to_sell(&market, &contracts, uni.contract_id, reservs)
            .await;

    // Transfer of amount to the wallet
    token_abi_calls::mint_and_transfer(&usdc_instance, amount, bob_address).await;

    //Сheck balance
    let balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Bob calls buy_collateral
    let addr = bob_address;
    market_abi_calls::buy_collateral(&inst, usdc.asset_id, amount, uni.contract_id, 1, addr)
        .await
        .unwrap();

    market_abi_calls::debug_increment_timestamp(&market).await;

    // TODO claim_paused

    // =================================================
    // ==================== Case #6 ====================
    // 👛 Wallet: Admin 🗿
    // 🤙 Call: reset UNI price and pause

    let amount = parse_units(5, 9); //1 UNI = $5
    oracle_abi_calls::set_price(&oracle, uni.contract_id, amount).await;
    let res = oracle_abi_calls::get_price(&oracle, uni.contract_id).await;
    assert!(res.price == amount);

    let pause_config = PauseConfiguration {
        supply_paused: true,
        withdraw_paused: true,
        absorb_paused: true,
        buy_pause: true,
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

    let amount = parse_units(400, usdc.config.decimals);

    // Transfer of 400 USDC to the Bob's wallet
    token_abi_calls::mint_and_transfer(&usdc_instance, amount, bob_address).await;

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

    let amount = parse_units(40, uni.config.decimals);

    // Transfer of 40 UNI to the Alice's wallet
    token_abi_calls::mint_and_transfer(&uni_instance, amount, alice_address).await;

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

    let amount = parse_units(150, usdc.config.decimals);

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
    let reservs = market_abi_calls::get_collateral_reserves(&market, uni.contract_id).await;
    assert!(!reservs.negative);

    let reservs = reservs.value;
    let amount =
        market_abi_calls::collateral_value_to_sell(&market, &contracts, uni.contract_id, reservs)
            .await;

    // Transfer of amount to the wallet
    token_abi_calls::mint_and_transfer(&usdc_instance, amount, bob_address).await;

    // Bob calls buy_collateral
    // let addr = bob_address;
    // let res =
    //     market_abi_calls::buy_collateral(&inst, usdc.asset_id, amount, uni.contract_id, 1, addr)
    //         .await
    //         .is_err();
    // assert!(res);
}
