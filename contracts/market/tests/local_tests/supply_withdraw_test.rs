use fuels::prelude::ViewOnlyAccount;
use fuels::types::Address;

use crate::utils::local_tests_utils::market::market_abi_calls;
use crate::utils::local_tests_utils::oracle::oracle_abi_calls;
use crate::utils::local_tests_utils::token::{token_abi_calls, TokenContract};
use crate::utils::{debug_state, print_case_title, print_title};
use crate::utils::{local_tests_utils::market, number_utils::parse_units};

// Multiplies all values by this number
// It is necessary in order to test how the protocol works with large amounts
const AMOUNT_COEFFICIENT: u64 = 10u64.pow(0);

#[tokio::test]
async fn supply_withdraw_test() {
    let scale_6 = 10u64.pow(6) as f64;
    let scale_9 = 10u64.pow(9) as f64;

    print_title("Supply & withdraw test");
    let (wallets, assets, market, oracle) = market::setup_market().await;

    // ==================== Wallets ====================
    let admin = wallets[0].clone();
    let alice = wallets[1].clone();
    let alice_address = Address::from(alice.address());
    let bob = wallets[2].clone();
    let bob_address = Address::from(bob.address());
    let chad = wallets[3].clone();
    let chad_address = Address::from(chad.address());

    // ==================== Assets ====================
    let usdc = assets.get("USDC").unwrap();
    let usdc_instance = TokenContract::new(usdc.contract_id, admin.clone());
    let uni = assets.get("UNI").unwrap();
    let uni_instance = TokenContract::new(uni.contract_id, admin.clone());

    // ==================== Set oracle prices ====================
    let amount = parse_units(1, 9); //1 USDC = $1
    oracle_abi_calls::set_price(&oracle, usdc.contract_id, amount).await;
    let res = oracle_abi_calls::get_price(&oracle, usdc.contract_id).await;
    assert!(res.price == amount);

    let amount = parse_units(5, 9); //1 UNI = $5
    oracle_abi_calls::set_price(&oracle, uni.contract_id, amount).await;
    let res = oracle_abi_calls::get_price(&oracle, uni.contract_id).await;
    assert!(res.price == amount);

    println!("1 USDC = $ ⎮ 1 UNI = $5\n");
    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;

    let contracts = oracle_abi_calls::get_as_settable_contract(&oracle);
    // =================================================
    // ==================== Case #0 ====================
    // 👛 Wallet: Bob 🧛
    // 🤙 Call: supply_base
    // 💰 Amount: 400.00 USDC

    let amount = parse_units(400 * AMOUNT_COEFFICIENT, usdc.config.decimals);
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(0, "Bob", "supply_base", log_amount.as_str());
    println!("💸 Bob + {log_amount}");

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

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #1 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: supply_collateral
    // 💰 Amount: 40.00 UNI ~ $200.00

    let amount = parse_units(40 * AMOUNT_COEFFICIENT, uni.config.decimals);
    let log_amount = format!("{} UNI", amount as f64 / scale_9);
    print_case_title(1, "Alice", "supply_collateral", log_amount.as_str());
    println!("💸 Alice + {log_amount}");

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

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #2 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_base
    // 💰 Amount: 50.00 USDC

    let amount = parse_units(50 * AMOUNT_COEFFICIENT, usdc.config.decimals);
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

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #3 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: supply_collateral
    // 💰 Amount: 60.00 UNI ~ $300.00

    let amount = parse_units(60 * AMOUNT_COEFFICIENT, uni.config.decimals);
    let log_amount = format!("{} UNI", amount as f64 / scale_9);
    print_case_title(3, "Chad", "supply_collateral", log_amount.as_str());
    println!("💸 Chad + {log_amount}");

    // Transfer of 60 UNI to the Chad's wallet
    token_abi_calls::mint_and_transfer(&uni_instance, amount, chad_address).await;

    let balance = chad.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == amount);

    //Chad calls supply_collateral
    let inst = market.with_account(chad.clone()).unwrap();
    market_abi_calls::supply_collateral(&inst, uni.asset_id, amount)
        .await
        .unwrap();

    //Сheck supply balance equal to 60 UNI
    let res = market_abi_calls::get_user_collateral(&inst, chad_address, uni.contract_id).await;
    assert!(res == amount);

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #4 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: supply_base
    // 💰 Amount: 200.00 USDC

    let amount = parse_units(200 * AMOUNT_COEFFICIENT, usdc.config.decimals);
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(4, "Chad", "supply_base", log_amount.as_str());
    println!("💸 Chad + {log_amount}");

    // Transfer of 200 USDC to the Chad's wallet
    token_abi_calls::mint_and_transfer(&usdc_instance, amount, chad_address).await;

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

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #5 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_base
    // 💰 Amount: 100.00 USDC

    let amount = parse_units(100 * AMOUNT_COEFFICIENT, usdc.config.decimals);
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(5, "Alice", "withdraw_base", log_amount.as_str());

    //Alice calls withdraw_base
    let inst = market.with_account(alice.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &contracts, amount)
        .await
        .unwrap();

    // USDC balance should be amount + 50 USDC from case #2
    let balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount + parse_units(50 * AMOUNT_COEFFICIENT, usdc.config.decimals));

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #6 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: withdraw_base
    // 💰 Amount: 300.00 USDC

    let amount = parse_units(300 * AMOUNT_COEFFICIENT, usdc.config.decimals);
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(6, "Chad", "withdraw_base", log_amount.as_str());

    //Chad calls withdraw_base
    let inst = market.with_account(chad.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &contracts, amount)
        .await
        .unwrap();

    // USDC balance check
    let balance = chad.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #7 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: supply_base
    // 💰 Amount: Present value (150.061118 USDC)

    let (_, amount) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;

    let log_amount = format!("Present value: {} USDC", amount as f64 / scale_6);
    print_case_title(7, "Alice", "supply_base", log_amount.as_str());

    let delta = amount - 150_000_000 * AMOUNT_COEFFICIENT;
    println!("💸 Alice + {} USDC", delta as f64 / scale_6);

    // Transfer some coins to pay protocol fee
    token_abi_calls::mint_and_transfer(&usdc_instance, delta, alice_address).await;

    // Alice calls supply_base
    let inst = market.with_account(alice.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    let (_, amount) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;
    assert!(amount == 0);

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #8 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: supply_base
    // 💰 Amount: USDC Balance ~ 100.046928 USDC

    let (_, amount) = market_abi_calls::get_user_supply_borrow(&market, chad_address).await;

    let delta = amount - 100u64 * AMOUNT_COEFFICIENT * scale_6 as u64;
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(8, "Chad", "supply_base", log_amount.as_str());
    println!("💸 Chad + {} USDC", delta as f64 / scale_6);

    // Transfer of 100.046928 USDC to the Chad's wallet
    token_abi_calls::mint_and_transfer(&usdc_instance, delta, chad_address).await;

    //Сheck balance
    let balance = chad.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount + 200_000_000 * AMOUNT_COEFFICIENT);

    // Chad calls supply_base
    let inst = market.with_account(chad.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    let (_, borrow) = market_abi_calls::get_user_supply_borrow(&market, chad_address).await;
    assert!(borrow == 0);

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #9 ====================
    // 👛 Wallet: Bob 🧛
    // 🤙 Call: withdraw_base
    // 💰 Amount: 400.058340 USDC

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

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #10 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_collateral
    // 💰 Amount: 40.00 UNI ~ $200.00

    let amount = parse_units(40 * AMOUNT_COEFFICIENT, uni.config.decimals);
    let log_amount = format!("{} UNI", amount as f64 / scale_9);
    print_case_title(10, "Alice", "withdraw_collateral", log_amount.as_str());

    //Alice calls withdraw_base
    let inst = market.with_account(alice.clone()).unwrap();

    market_abi_calls::withdraw_collateral(&inst, &contracts, uni.contract_id, amount)
        .await
        .unwrap();

    // UNI balance check
    let balance = alice.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == amount);

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #11 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: withdraw_collateral
    // 💰 Amount: 60.00 UNI ~ $300.00

    let amount = parse_units(60 * AMOUNT_COEFFICIENT, uni.config.decimals);
    let log_amount = format!("{} UNI", amount as f64 / scale_9);
    print_case_title(11, "Chad", "withdraw_collateral", log_amount.as_str());

    //Chad calls withdraw_base
    let inst = market.with_account(chad.clone()).unwrap();

    market_abi_calls::withdraw_collateral(&inst, &contracts, uni.contract_id, amount)
        .await
        .unwrap();

    // UNI balance check
    let balance = chad.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == amount);

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
}
