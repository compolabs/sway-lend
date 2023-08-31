use fuels::prelude::ViewOnlyAccount;
use fuels::types::Address;
use src20_sdk::token_factory_abi_calls;

use crate::utils::contracts_utils::market_utils::{
    deploy_market, get_market_config, market_abi_calls,
};
use crate::utils::contracts_utils::oracle_utils::{deploy_oracle, oracle_abi_calls};
use crate::utils::number_utils::parse_units;
use crate::utils::{debug_state, init_tokens, init_wallets, print_case_title, print_title};

// Multiplies all values by this number
// It is necessary in order to test how the protocol works with large amounts
const AMOUNT_COEFFICIENT: u64 = 10u64.pow(0);

#[tokio::test]
async fn main_test() {
    let scale_6 = 10u64.pow(6) as f64;
    let scale_9 = 10u64.pow(9) as f64;

    print_title("Main test with UNI");
    //--------------- WALLETS ---------------
    let wallets = init_wallets().await;
    let admin = &wallets[0];
    let alice = &wallets[1];
    let bob = &wallets[2];
    let chad = &wallets[3];

    let alice_address = Address::from(alice.address());
    let bob_address = Address::from(bob.address());
    let chad_address = Address::from(chad.address());

    //--------------- ORACLE ---------------
    let oracle = deploy_oracle(&admin).await;
    let contracts = oracle_abi_calls::get_as_settable_contract(&oracle);

    //--------------- TOKENS ---------------
    let (assets, asset_configs, factory) = init_tokens(&admin, oracle.contract_id().into()).await;
    let usdc = assets.get("USDC").unwrap();
    let uni = assets.get("UNI").unwrap();

    //--------------- MARKET ---------------

    let market = deploy_market(&admin).await;

    let market_config = get_market_config(
        admin.address().into(),
        admin.address().into(),
        usdc.bits256,
        usdc.decimals,
        oracle.contract_id().into(),
        assets.get("SWAY").unwrap().bits256,
    );

    // debug step
    let step: Option<u64> = Option::Some(10000);

    market_abi_calls::initialize(&market, &market_config, &asset_configs, step)
        .await
        .expect("❌ Cannot initialize market");

    // ==================== Set oracle prices ====================
    let amount = parse_units(1, 9); //1 USDC = $1
    oracle_abi_calls::set_price(&oracle, usdc.bits256, amount).await;
    let res = oracle_abi_calls::get_price(&oracle, usdc.bits256).await;
    assert!(res.price == amount);

    let amount = parse_units(5, 9); //1 UNI = $5
    oracle_abi_calls::set_price(&oracle, uni.bits256, amount).await;
    let res = oracle_abi_calls::get_price(&oracle, uni.bits256).await;
    assert!(res.price == amount);

    println!("1 USDC = $1 ⎮ 1 UNI = $5\n");
    debug_state(&market, &wallets, usdc, uni).await;

    // =================================================
    // ==================== Case #0 ====================
    // 👛 Wallet: Bob 🧛
    // 🤙 Call: supply_base
    // 💰 Amount: 100.00 USDC

    let amount = parse_units(100 * AMOUNT_COEFFICIENT, usdc.decimals);
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(0, "Bob", "supply_base", log_amount.as_str());
    println!("💸 Bob + {log_amount}");

    // Transfer of 100 USDC to the Bob's wallet
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

    debug_state(&market, &wallets, usdc, uni).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #1 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: supply_collateral
    // 💰 Amount: 40.00 UNI ~ $200.00

    let amount = parse_units(40 * AMOUNT_COEFFICIENT, uni.decimals);
    let log_amount = format!("{} UNI", amount as f64 / scale_9);
    print_case_title(1, "Alice", "supply_collateral", log_amount.as_str());
    println!("💸 Alice + {log_amount}");

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

    debug_state(&market, &wallets, usdc, uni).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #2 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_base
    // 💰 Amount: 50.00 USDC

    let amount = parse_units(50 * AMOUNT_COEFFICIENT, usdc.decimals);
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(2, "Alice", "withdraw_base", log_amount.as_str());

    // Alice calls withdraw_base
    let inst = market.with_account(alice.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &contracts, amount)
        .await
        .unwrap();

    // USDC balance check
    let balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    debug_state(&market, &wallets, usdc, uni).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #3 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: supply_collateral
    // 💰 Amount: 60.00 UNI ~ $300.00

    let amount = parse_units(60 * AMOUNT_COEFFICIENT, uni.decimals);
    let log_amount = format!("{} UNI", amount as f64 / scale_9);
    print_case_title(3, "Chad", "supply_collateral", log_amount.as_str());
    println!("💸 Chad + {log_amount}");

    // Transfer of 60 UNI to the Chad's wallet
    token_factory_abi_calls::mint(&factory, chad_address, &uni.symbol, amount)
        .await
        .unwrap();

    let balance = chad.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == amount);

    //Chad calls supply_collateral
    let inst = market.with_account(chad.clone()).unwrap();
    market_abi_calls::supply_collateral(&inst, uni.asset_id, amount)
        .await
        .unwrap();

    //Сheck supply balance equal to 60 UNI
    let res = market_abi_calls::get_user_collateral(&inst, chad_address, uni.bits256).await;
    assert!(res == amount);

    debug_state(&market, &wallets, usdc, uni).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #4 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: supply_base
    // 💰 Amount: 200.00 USDC

    let amount = parse_units(200 * AMOUNT_COEFFICIENT, usdc.decimals);
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(4, "Chad", "supply_base", log_amount.as_str());
    println!("💸 Chad + {log_amount}");

    // Transfer of 200 USDC to the Chad's wallet
    token_factory_abi_calls::mint(&factory, chad_address, &usdc.symbol, amount)
        .await
        .unwrap();

    let balance = chad.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Chad calls supply_base
    let inst = market.with_account(chad.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    //Сheck supply balance equal to 200 USDC
    let (supply_balance, _) = market_abi_calls::get_user_supply_borrow(&inst, chad_address).await;
    assert!(amount - 5 < supply_balance);

    debug_state(&market, &wallets, usdc, uni).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #5 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_base
    // 💰 Amount: 100.00 USDC

    let amount = parse_units(100 * AMOUNT_COEFFICIENT, usdc.decimals);
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(5, "Alice", "withdraw_base", log_amount.as_str());

    //Alice calls withdraw_base
    let inst = market.with_account(alice.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &contracts, amount)
        .await
        .unwrap();

    // USDC balance should be amount + 50 USDC from case #2
    let balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount + parse_units(50 * AMOUNT_COEFFICIENT, usdc.decimals));

    debug_state(&market, &wallets, usdc, uni).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #6 ====================
    // 👛 Wallet: Admin 🗿
    // 🤙 Drop of collateral price
    // 💰 Amount: -10%

    print_case_title(6, "Admin", "Drop of collateral price", "-10%");
    let res = oracle_abi_calls::get_price(&oracle, uni.bits256).await;
    let new_price = (res.price as f64 * 0.9) as u64;
    println!(
        "🔻 UNI price drops: ${}  -> ${}",
        res.price as f64 / scale_9,
        new_price as f64 / scale_9
    );
    oracle_abi_calls::set_price(&oracle, uni.bits256, new_price).await;
    let res = oracle_abi_calls::get_price(&oracle, uni.bits256).await;
    assert!(new_price == res.price);

    debug_state(&market, &wallets, usdc, uni).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #7 ====================
    // 👛 Wallet: Bob 🦹
    // 🤙 Call: absorb
    // 🔥 Target: Alice

    print_case_title(7, "Bob", "absorb", "Alice");

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

    debug_state(&market, &wallets, usdc, uni).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #8 ====================
    // 👛 Wallet: Bob 🤵
    // 🤙 Call: buy_collateral
    // 💰 Amount: 172.44 USDC

    let inst = market.with_account(bob.clone()).unwrap();
    let reservs = market_abi_calls::get_collateral_reserves(&market, uni.bits256).await;
    assert!(!reservs.negative);

    let reservs = reservs.value;
    let amount =
        market_abi_calls::collateral_value_to_sell(&market, &contracts, uni.bits256, reservs).await;

    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(8, "Bob", "buy_collateral", log_amount.as_str());

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

    //Check
    let balance = bob.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == 40_000_000_000 * AMOUNT_COEFFICIENT);

    debug_state(&market, &wallets, usdc, uni).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #9 ====================
    // 👛 Wallet: Bob 🧛
    // 🤙 Call: withdraw_base
    // 💰 Amount: 100.021671 USDC

    let (amount, _) = market_abi_calls::get_user_supply_borrow(&market, bob_address).await;
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(9, "Bob", "withdraw_base", log_amount.as_str());

    //Bob calls withdraw_base
    let inst = market.with_account(bob.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &contracts, amount)
        .await
        .unwrap();

    // USDC balance check
    let (supplied, _) = market_abi_calls::get_user_supply_borrow(&market, bob_address).await;
    assert!(supplied == 0);
    assert!(bob.get_asset_balance(&usdc.asset_id).await.unwrap() == amount);

    debug_state(&market, &wallets, usdc, uni).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #10 ====================
    // 👛 Wallet: Chad 🧛
    // 🤙 Call: withdraw_base
    // 💰 Amount: 200.0233392 USDC

    let (amount, _) = market_abi_calls::get_user_supply_borrow(&market, chad_address).await;
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(10, "Chad", "withdraw_base", log_amount.as_str());

    //Chad calls withdraw_base
    let inst = market.with_account(chad.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &contracts, amount)
        .await
        .unwrap();

    // USDC balance check
    let (supplied, _) = market_abi_calls::get_user_supply_borrow(&market, chad_address).await;
    assert!(supplied == 0);
    assert!(chad.get_asset_balance(&usdc.asset_id).await.unwrap() == amount);

    debug_state(&market, &wallets, usdc, uni).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #11 ====================
    // 👛 Wallet: Alice 🧛
    // 🤙 Call: withdraw_base
    // 💰 Amount: 17.276598 USDC

    let (amount, _) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(11, "Alice", "withdraw_base", log_amount.as_str());

    //Alice calls withdraw_base
    let inst = market.with_account(alice.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &contracts, amount)
        .await
        .unwrap();

    // USDC balance check
    let (supplied, _) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;
    assert!(supplied == 0);

    debug_state(&market, &wallets, usdc, uni).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #12 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: withdraw_collateral
    // 💰 Amount: 270 UNI

    let amount = market_abi_calls::get_user_collateral(&market, chad_address, uni.bits256).await;
    let log_amount = format!("{} UNI", amount as f64 / scale_9);
    print_case_title(12, "Chad", "withdraw_collateral", log_amount.as_str());

    //Chad calls withdraw_base
    let inst = market.with_account(chad.clone()).unwrap();

    market_abi_calls::withdraw_collateral(&inst, &contracts, uni.bits256, amount)
        .await
        .unwrap();

    // UNI balance check
    let balance = chad.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == amount);

    debug_state(&market, &wallets, usdc, uni).await;
}
