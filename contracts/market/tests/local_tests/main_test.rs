use fuels::tx::Address;

use crate::utils::local_tests_utils::market::market_abi_calls;
use crate::utils::local_tests_utils::oracle::oracle_abi_calls;
use crate::utils::local_tests_utils::token::{token_abi_calls, TokenContract};
use crate::utils::{debug_state, print_case_title, print_title};
use crate::utils::{local_tests_utils::market, number_utils::parse_units};

// Multiplies all values by this number
// It is necessary in order to test how the protocol works with large amounts

#[tokio::test]
async fn main_test() {
    let coefficient = 10u64.pow(0);
    do_main_test(coefficient, "UNI", 5, 0.1).await;
    do_main_test(coefficient, "ETH", 2000, 0.2).await;
    do_main_test(coefficient, "SWAY", 50, 0.1).await;
    do_main_test(coefficient, "LINK", 5, 0.15).await;
    do_main_test(coefficient, "BTC", 20000, 0.1).await;
}

async fn do_main_test(amount_coefficient: u64, collateral_symbol: &str, collateral_price: u64, drop_percent: f64) {
    let scale_9 = 10u64.pow(9) as f64;
    print_title(("Main test with ".to_owned() + collateral_symbol).as_str());
    let (wallets, assets, market, oracle) = market::setup_market().await;

    let contracts = oracle_abi_calls::get_as_settable_contract(&oracle);
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
    let usdc_instance = TokenContract::new(usdc.contract_id.into(), admin.clone());
    let usdc_scale = 10u64.pow(6) as f64;
    let collateral = assets.get(collateral_symbol).unwrap();
    let collateral_instance = TokenContract::new(collateral.contract_id.into(), admin.clone());
    let collateral_scale = 10_i32.pow(collateral.config.decimals as u32) as f64;
    // ==================== Set oracle prices ====================
    let amount = parse_units(1, 9); //1 USDC = $1
    oracle_abi_calls::set_price(&oracle, usdc.contract_id, amount).await;
    let res = oracle_abi_calls::get_price(&oracle, usdc.contract_id).await;
    assert!(res.price == amount);

    let amount = parse_units(collateral_price, 9);
    oracle_abi_calls::set_price(&oracle, collateral.contract_id, amount).await;
    let res = oracle_abi_calls::get_price(&oracle, collateral.contract_id).await;
    assert!(res.price == amount);

    println!("1 USDC = $1 ⎮ 1 {collateral_symbol} = ${collateral_price}\n");
    debug_state(&market, &wallets, usdc.contract_id, collateral.contract_id).await;

    // =================================================
    // ==================== Case #0 ====================
    // 👛 Wallet: Bob 🧛
    // 🤙 Call: supply_base
    // 💰 Amount: 100.00 USDC

    let amount = parse_units(100 * amount_coefficient, usdc.config.decimals);
    let log_amount = format!("{} USDC", amount as f64 / usdc_scale);
    print_case_title(0, "Bob", "supply_base", log_amount.as_str());
    println!("💸 Bob + {log_amount}");

    // Transfer of 100 USDC to the Bob's wallet
    token_abi_calls::mint_and_transfer(&usdc_instance, amount, bob_address).await;

    let balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Bob calls supply_base
    let inst = market.with_wallet(bob.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    // Сheck supply balance equal to 400 USDC
    let (supply_balance, _) = market_abi_calls::get_user_supply_borrow(&inst, bob_address).await;
    assert!(supply_balance == amount);

    debug_state(&market, &wallets, usdc.contract_id, collateral.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #1 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: supply_collateral
    // 💰 Amount: $200.00

    let amount = (200f64 / collateral_price as f64 * amount_coefficient as f64 * collateral_scale) as u64;
    let log_amount = format!("{} {collateral_symbol}", amount as f64 / collateral_scale);
    print_case_title(1, "Alice", "supply_collateral", log_amount.as_str());
    println!("💸 Alice + {log_amount}");

    if collateral.config.symbol != "ETH" {
        // Transfer of amount to the Alice's wallet
        token_abi_calls::mint_and_transfer(&collateral_instance, amount, alice_address).await;

        let balance = alice.get_asset_balance(&collateral.asset_id).await.unwrap();
        assert!(balance == amount);
    }

    // Alice calls supply_collateral
    let inst = market.with_wallet(alice.clone()).unwrap();
    market_abi_calls::supply_collateral(&inst, collateral.asset_id, amount)
        .await
        .unwrap();

    // Сheck supply balance equal to amount
    let res = market_abi_calls::get_user_collateral(&inst, alice_address, collateral.contract_id).await;
    assert!(res == amount);

    debug_state(&market, &wallets, usdc.contract_id, collateral.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #2 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_base
    // 💰 Amount: 50.00 USDC

    let amount = parse_units(50 * amount_coefficient, usdc.config.decimals);
    let log_amount = format!("{} USDC", amount as f64 / usdc_scale);
    print_case_title(2, "Alice", "withdraw_base", log_amount.as_str());

    // Alice calls withdraw_base
    let inst = market.with_wallet(alice.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &contracts, amount)
        .await
        .unwrap();

    // USDC balance check
    let balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    debug_state(&market, &wallets, usdc.contract_id, collateral.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #3 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: supply_collateral
    // 💰 Amount: 0.15 ETH ~ $300.00

    let amount = (300f64 / collateral_price as f64 * amount_coefficient as f64 * collateral_scale) as u64;
    let log_amount = format!("{} {collateral_symbol}", amount as f64 / collateral_scale);
    print_case_title(3, "Chad", "supply_collateral", log_amount.as_str());
    println!("💸 Chad + {log_amount}");

    if collateral_symbol != "ETH" {
        // Transfer of amount to the Chad's wallet
        token_abi_calls::mint_and_transfer(&collateral_instance, amount, chad_address).await;

        let balance = chad.get_asset_balance(&collateral.asset_id).await.unwrap();
        assert!(balance == amount);
    }

    //Chad calls supply_collateral
    let inst = market.with_wallet(chad.clone()).unwrap();
    market_abi_calls::supply_collateral(&inst, collateral.asset_id, amount)
        .await
        .unwrap();

    //Сheck supply balance equal to amount
    let res = market_abi_calls::get_user_collateral(&inst, chad_address, collateral.contract_id).await;
    assert!(res == amount);

    debug_state(&market, &wallets, usdc.contract_id, collateral.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #4 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: supply_base
    // 💰 Amount: 200.00 USDC

    let amount = parse_units(200 * amount_coefficient, usdc.config.decimals);
    let log_amount = format!("{} USDC", amount as f64 / usdc_scale);
    print_case_title(4, "Chad", "supply_base", log_amount.as_str());
    println!("💸 Chad + {log_amount}");

    // Transfer of 200 USDC to the Chad's wallet
    token_abi_calls::mint_and_transfer(&usdc_instance, amount, chad_address).await;

    let balance = chad.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Chad calls supply_base
    let inst = market.with_wallet(chad.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    //Сheck supply balance equal to 200 USDC
    let (supply_balance, _) = market_abi_calls::get_user_supply_borrow(&inst, chad_address).await;
    assert!(amount - 5 < supply_balance);

    debug_state(&market, &wallets, usdc.contract_id, collateral.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #5 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_base
    // 💰 Amount: 100.00 USDC

    let amount = parse_units(100 * amount_coefficient, usdc.config.decimals);
    let log_amount = format!("{} USDC", amount as f64 / usdc_scale);
    print_case_title(5, "Alice", "withdraw_base", log_amount.as_str());

    //Alice calls withdraw_base
    let inst = market.with_wallet(alice.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &contracts, amount)
        .await
        .unwrap();

    // USDC balance should be amount + 50 USDC from case #2
    let balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount + parse_units(50 * amount_coefficient, usdc.config.decimals));

    debug_state(&market, &wallets, usdc.contract_id, collateral.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #6 ====================
    // 👛 Wallet: Admin 🗿
    // 🤙 Drop of collateral price
    // 💰 Amount: -drop_percent%

    print_case_title(6, "Admin", "Drop of collateral price", "-10%");
    let res = oracle_abi_calls::get_price(&oracle, collateral.contract_id).await;
    let new_price = (res.price as f64 * (1f64 - drop_percent)) as u64;
    println!(
        "🔻 {collateral_symbol} price drops: ${}  -> ${}",
        res.price as f64 / scale_9,
        new_price as f64 / scale_9
    );
    oracle_abi_calls::set_price(&oracle, collateral.contract_id, new_price).await;
    let res = oracle_abi_calls::get_price(&oracle, collateral.contract_id).await;
    assert!(new_price == res.price);

    debug_state(&market, &wallets, usdc.contract_id, collateral.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #7 ====================
    // 👛 Wallet: Bob 🦹
    // 🤙 Call: absorb
    // 🔥 Target: Alice

    print_case_title(7, "Bob", "absorb", "Alice");

    assert!(market_abi_calls::is_liquidatable(&market, &contracts, alice_address).await);

    let inst = market.with_wallet(bob.clone()).unwrap();
    market_abi_calls::absorb(&inst, &contracts, vec![alice_address])
        .await
        .unwrap();

    //Check if absorb was ok
    let (_, borrow) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;
    assert!(borrow == 0);

    let amount =
        market_abi_calls::get_user_collateral(&market, alice_address, collateral.contract_id).await;
    assert!(amount == 0);

    debug_state(&market, &wallets, usdc.contract_id, collateral.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #8 ====================
    // 👛 Wallet: Bob 🤵
    // 🤙 Call: buy_collateral
    // 💰 Amount: 172.44 USDC

    let inst = market.with_wallet(bob.clone()).unwrap();
    let reservs = market_abi_calls::get_collateral_reserves(&market, collateral.contract_id).await;
    assert!(!reservs.negative);

    let reservs = reservs.value;
    let amount =
        market_abi_calls::collateral_value_to_sell(&market, &contracts, collateral.contract_id, reservs)
            .await;

    let log_amount = format!("{} USDC", amount as f64 / usdc_scale);
    print_case_title(8, "Bob", "buy_collateral", log_amount.as_str());

    // Transfer of amount to the wallet
    token_abi_calls::mint_and_transfer(&usdc_instance, amount, bob_address).await;

    //Сheck balance
    let balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Bob calls buy_collateral
    let addr = bob_address;
    market_abi_calls::buy_collateral(&inst, usdc.asset_id, amount, collateral.contract_id, 1, addr)
        .await
        .unwrap();

    //TODO: Check
    // let balance = bob.get_asset_balance(&collateral.asset_id).await.unwrap();
    // assert!(balance >= 40_000_000_000 * amount_coefficient);

    debug_state(&market, &wallets, usdc.contract_id, collateral.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #9 ====================
    // 👛 Wallet: Bob 🧛
    // 🤙 Call: withdraw_base
    // 💰 Amount: 100.021671 USDC

    let (amount, _) = market_abi_calls::get_user_supply_borrow(&market, bob_address).await;
    let log_amount = format!("{} USDC", amount as f64 / usdc_scale);
    print_case_title(9, "Bob", "withdraw_base", log_amount.as_str());

    //Bob calls withdraw_base
    let inst = market.with_wallet(bob.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &contracts, amount)
        .await
        .unwrap();

    // USDC balance check
    let (supplied, _) = market_abi_calls::get_user_supply_borrow(&market, bob_address).await;
    assert!(supplied == 0);
    assert!(bob.get_asset_balance(&usdc.asset_id).await.unwrap() == amount);

    debug_state(&market, &wallets, usdc.contract_id, collateral.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #10 ====================
    // 👛 Wallet: Chad 🧛
    // 🤙 Call: withdraw_base
    // 💰 Amount: 200.0233392 USDC

    let (amount, _) = market_abi_calls::get_user_supply_borrow(&market, chad_address).await;
    let log_amount = format!("{} USDC", amount as f64 / usdc_scale);
    print_case_title(10, "Chad", "withdraw_base", log_amount.as_str());

    //Chad calls withdraw_base
    let inst = market.with_wallet(chad.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &contracts, amount)
        .await
        .unwrap();

    // USDC balance check
    let (supplied, _) = market_abi_calls::get_user_supply_borrow(&market, chad_address).await;
    assert!(supplied == 0);
    assert!(chad.get_asset_balance(&usdc.asset_id).await.unwrap() == amount);

    debug_state(&market, &wallets, usdc.contract_id, collateral.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #11 ====================
    // 👛 Wallet: Alice 🧛
    // 🤙 Call: withdraw_base
    // 💰 Amount: 17.276598 USDC

    let (amount, _) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;
    let log_amount = format!("{} USDC", amount as f64 / usdc_scale);
    print_case_title(11, "Alice", "withdraw_base", log_amount.as_str());

    //Alice calls withdraw_base
    let inst = market.with_wallet(alice.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &contracts, amount)
        .await
        .unwrap();

    // USDC balance check
    let (supplied, _) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;
    assert!(supplied == 0);

    debug_state(&market, &wallets, usdc.contract_id, collateral.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #12 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: withdraw_collateral
    // 💰 Amount: All collateral

    let amount =
        market_abi_calls::get_user_collateral(&market, chad_address, collateral.contract_id).await;
    let log_amount = format!("{} {collateral_symbol}", amount as f64 / scale_9);
    print_case_title(12, "Chad", "withdraw_collateral", log_amount.as_str());

    //Chad calls withdraw_base
    let inst = market.with_wallet(chad.clone()).unwrap();

    market_abi_calls::withdraw_collateral(&inst, &contracts, collateral.contract_id, amount)
        .await
        .unwrap();

    // balance check
    let balance = chad.get_asset_balance(&collateral.asset_id).await.unwrap();
    assert!(balance >= amount);

    debug_state(&market, &wallets, usdc.contract_id, collateral.contract_id).await;
}
