use std::str::FromStr;

use fuels::prelude::TxParameters;
use fuels::signers::WalletUnlocked;
use fuels::tx::{Address, AssetId, ContractId};

use crate::utils::local_tests_utils::market::{market_abi_calls, MarketContract};
use crate::utils::local_tests_utils::oracle::oracle_abi_calls;
use crate::utils::number_utils::format_units;
use crate::utils::{local_tests_utils::market, number_utils::parse_units};

#[tokio::test]
async fn main_test() {
    let tx_params = TxParameters::default();

    print_title("Supply & withdraw test");
    let (wallets, assets, market, oracle) = market::setup_market().await;

    // ==================== Assets ====================
    let usdc = assets.get("USDC").unwrap();
    let uni = assets.get("UNI").unwrap();

    // ==================== Wallets ====================
    let admin = wallets[0].clone();
    let alice = wallets[1].clone();
    let bob = wallets[2].clone();
    let chad = wallets[3].clone();

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

    // =================================================
    // ==================== Case #0 ====================
    // 👛 Wallet: Bob 🧛
    // 🤙 Call: supply_base
    // 💰 Amount: 400.00 USDC

    print_case_title(0, "Bob", "supply_base", "400.00 USDC");
    println!("💸 Bob + 400.00 USDC");

    // Transfer of 400 USDC to the Bob's wallet
    let amount = parse_units(400, usdc.config.decimals);
    admin
        .transfer(bob.address(), amount, usdc.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    let balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Bob calls supply_base
    let inst = market.with_wallet(bob.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    // Сheck supply balance equal to 400 USDC
    let (supply_balance, _borrow_balance) =
        market_abi_calls::get_user_supply_borrow(&inst, Address::from(bob.address())).await;
    assert!(supply_balance == amount);

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #1 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: supply_collateral
    // 💰 Amount: 40.00 UNI ~ $200.00

    print_case_title(1, "Alice", "supply_collateral", "40.00 UNI ($200.00)");
    println!("💸 Alice + 40.00 UNI");

    // Transfer of 40 UNI to the Alice's wallet
    let amount = parse_units(40, uni.config.decimals);
    admin
        .transfer(alice.address(), amount, uni.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    let balance = alice.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == amount);

    // Alice calls supply_collateral
    let inst = market.with_wallet(alice.clone()).unwrap();
    market_abi_calls::supply_collateral(&inst, uni.asset_id, amount)
        .await
        .unwrap();

    // Сheck supply balance equal to 40 UNI
    let address = Address::from(alice.address());
    let res = market_abi_calls::get_user_collateral(&inst, address, uni.contract_id).await;
    assert!(res == amount);

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #2 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_base
    // 💰 Amount: 50.00 USDC

    print_case_title(2, "Alice", "withdraw_base", "50.00 USDC");

    // Alice calls withdraw_base
    let amount = parse_units(50, usdc.config.decimals);
    let inst = market.with_wallet(alice.clone()).unwrap();
    let cotarcts = [oracle.get_contract_id().clone()];
    market_abi_calls::withdraw_base(&inst, &cotarcts, amount)
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

    print_case_title(3, "Chad", "supply_collateral", "60.00 UNI ($300.00)");
    println!("💸 Chad + 60.00 UNI");

    // Transfer of 60 UNI to the Chad's wallet
    let amount = parse_units(60, uni.config.decimals);
    admin
        .transfer(chad.address(), amount, uni.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    let balance = chad.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == amount);

    //Chad calls supply_collateral
    let inst = market.with_wallet(chad.clone()).unwrap();
    market_abi_calls::supply_collateral(&inst, uni.asset_id, amount)
        .await
        .unwrap();

    //Сheck supply balance equal to 60 UNI
    let address = Address::from(chad.address());
    let res = market_abi_calls::get_user_collateral(&inst, address, uni.contract_id).await;
    assert!(res == amount);

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #4 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: supply_base
    // 💰 Amount: 200.00 USDC

    print_case_title(4, "Chad", "supply_base", "200.00 USDC");
    println!("💸 Chad + 200.00 USDC");

    // Transfer of 200 USDC to the Chad's wallet
    let amount = parse_units(200, usdc.config.decimals);
    admin
        .transfer(chad.address(), amount, usdc.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    let balance = chad.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Chad calls supply_base
    let inst = market.with_wallet(chad.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    //Сheck supply balance equal to 200 USDC
    let (_supply_balance, _borrow_balance) =
        market_abi_calls::get_user_supply_borrow(&inst, Address::from(chad.address())).await;
    assert!(amount - 5 < supply_balance);

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #5 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_base
    // 💰 Amount: 100.00 USDC

    print_case_title(5, "Alice", "withdraw_base", "100 USDC");

    //Alice calls withdraw_base
    let amount = parse_units(100, usdc.config.decimals);
    let inst = market.with_wallet(alice.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &cotarcts, amount)
        .await
        .unwrap();

    // USDC balance should be amount + 50 USDC from case #2
    let balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount + parse_units(50, usdc.config.decimals));

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #6 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: withdraw_base
    // 💰 Amount: 300.00 USDC

    print_case_title(6, "Chad", "withdraw_base", "300.00 USDC");

    //Chad calls withdraw_base
    let amount = parse_units(300, usdc.config.decimals);
    let inst = market.with_wallet(chad.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &cotarcts, amount)
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

    let address = Address::from(alice.address());
    let (_, amount) = market_abi_calls::get_user_supply_borrow(&market, address).await;
    let amount = amount + 47242; //FIXME
    let scale = 10u64.pow(6) as f64;
    let log_amount = format!("Present value: {} USDC", amount as f64 / scale);
    print_case_title(7, "Alice", "supply_base", log_amount.as_str());

    let delta_value = amount - 150_000_000;
    println!("💸 Alice + {} USDC", delta_value as f64 / scale);

    // Transfer some coins to pay protocol fee
    admin
        .transfer(alice.address(), delta_value, usdc.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    // Alice calls supply_base
    let inst = market.with_wallet(alice.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    let (_, amount) = market_abi_calls::get_user_supply_borrow(&market, address).await;
    assert!(amount == 0);

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #8 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: supply_base
    // 💰 Amount: USDC Balance ~ 100.046928 USDC

    let delta = 46_928;
    let amount = 100_000_000 + delta; //FIXME
    let scale = 10u64.pow(6) as f64;
    let log_amount = format!("{} USDC", amount as f64 / scale);
    print_case_title(8, "Chad", "supply_base", log_amount.as_str());
    println!("💸 Chad + {} USDC", delta as f64 / scale);

    // Transfer of 100.046928 USDC to the Chad's wallet
    admin
        .transfer(chad.address(), amount, usdc.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    //Сheck balance
    let balance = chad.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount + 300_000_000);

    // Chad calls supply_base
    let inst = market.with_wallet(chad.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    //TODO assert

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #9 ====================
    // 👛 Wallet: Bob 🧛
    // 🤙 Call: withdraw_base
    // 💰 Amount: -400.058340 USDC

    print_case_title(9, "Bob", "withdraw_base", "-400.058340 USDC");
    let delta = 58_340; //FIXME
    let amount = 400_000_000 + delta; //FIXME
    let scale = 10u64.pow(6) as f64;
    println!("💸 Bob + {} USDC", delta as f64 / scale);

    //Bob calls withdraw_base
    let inst = market.with_wallet(bob.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &cotarcts, amount)
        .await
        .unwrap();

    // USDC balance check
    let balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    //TODO assert

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #10 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_collateral
    // 💰 Amount: 40.00 UNI ~ $200.00

    print_case_title(10, "Alice", "withdraw_collateral", "40.00 UNI ($200.00)");

    //Alice calls withdraw_base
    let amount = parse_units(40, uni.config.decimals);
    let inst = market.with_wallet(alice.clone()).unwrap();

    market_abi_calls::withdraw_collateral(&inst, &cotarcts, uni.contract_id, amount)
        .await
        .unwrap();

    // UNI balance check
    let balance = alice.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == amount);

    //TODO assert

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    market_abi_calls::debug_increment_timestamp(&market).await;

    // =================================================
    // ==================== Case #11 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: withdraw_collateral
    // 💰 Amount: 60.00 UNI ~ $300.00
    print_case_title(11, "Chad", "withdraw_collateral", "60.00 UNI ($300.00)");
    //Chad calls withdraw_base
    let amount = parse_units(60, uni.config.decimals);
    let inst = market.with_wallet(chad.clone()).unwrap();

    market_abi_calls::withdraw_collateral(&inst, &cotarcts, uni.contract_id, amount)
        .await
        .unwrap();

    // UNI balance check
    let balance = chad.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == amount);

    //TODO assert

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
}

async fn debug_state(
    market: &MarketContract,
    wallets: &Vec<WalletUnlocked>,
    usdc_contract_id: ContractId,
    uni_contract_id: ContractId,
) {
    let usdc_asset_id = AssetId::from_str(usdc_contract_id.to_string().as_str()).unwrap();
    let uni_asset_id = AssetId::from_str(uni_contract_id.to_string().as_str()).unwrap();

    let alice = wallets[1].clone();
    let alice_address = Address::from(alice.address());

    let bob = wallets[2].clone();
    let bob_address = Address::from(bob.address());

    let chad = wallets[3].clone();
    let chad_address = Address::from(chad.address());

    let scale18 = 10u64.pow(18) as f64;

    let market_basic = market_abi_calls::get_market_basics(&market).await;
    let supply_base = market_basic.total_supply_base / 10u64.pow(6);
    let borrow_base = market_basic.total_borrow_base / 10u64.pow(6);
    let usdc_balance = market_abi_calls::balance_of(&market, usdc_contract_id).await / 10u64.pow(6);
    let uni_balance = market_abi_calls::balance_of(&market, uni_contract_id).await / 10u64.pow(9);
    let utilization = market_abi_calls::get_utilization(&market).await as f64 / scale18;
    let s_rate = market_basic.base_supply_index as f64 / scale18;
    let b_rate = market_basic.base_borrow_index as f64 / scale18;
    let total_collateral = market_abi_calls::totals_collateral(&market, uni_contract_id).await;
    let last_accrual_time = market_basic.last_accrual_time;
    println!("🏦 Market\n  Total supply {supply_base} USDC | Total supply {borrow_base} USDC",);
    println!("  Total USDC balance = {usdc_balance} USDC | Total UNI balance = {uni_balance} UNI");
    println!("  sRate {s_rate} | bRate {b_rate}");
    println!(
        "  Total collateral {} UNI",
        format_units(total_collateral, 9)
    );
    println!("  Utilization {utilization} | Last accrual time {last_accrual_time}",);

    let basic = market_abi_calls::get_user_basic(&market, alice_address).await;
    let sign = if basic.principal.negative { "-" } else { "+" };
    let value = format_units(basic.principal.value, 6);
    let (supply, borrow) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;
    let supply = format_units(supply, 6);
    let borrow = format_units(borrow, 6);
    let usdc_balance = alice.get_asset_balance(&usdc_asset_id).await.unwrap() / 10u64.pow(6);
    let uni_balance = alice.get_asset_balance(&uni_asset_id).await.unwrap() / 10u64.pow(9);
    let collateral =
        market_abi_calls::get_user_collateral(&market, alice_address, uni_contract_id).await;
    println!("\nAlice 🦹");
    println!("  Principal = {}{}", sign, value);
    println!("  Present supply = {supply} USDC | borrow = {borrow} USDC");
    println!("  Supplied collateral {} UNI", format_units(collateral, 9));
    println!("  Balance {usdc_balance} USDC | {uni_balance} UNI");

    let basic = market_abi_calls::get_user_basic(&market, bob_address).await;
    let sign = if basic.principal.negative { "-" } else { "+" };
    let value = format_units(basic.principal.value, 6);
    let (supply, borrow) = market_abi_calls::get_user_supply_borrow(&market, bob_address).await;
    let supply = format_units(supply, 6);
    let borrow = format_units(borrow, 6);
    let usdc_balance = bob.get_asset_balance(&usdc_asset_id).await.unwrap() / 10u64.pow(6);
    let uni_balance = bob.get_asset_balance(&uni_asset_id).await.unwrap() / 10u64.pow(9);
    let collateral =
        market_abi_calls::get_user_collateral(&market, bob_address, uni_contract_id).await;
    println!("\nBob 🧛");

    println!("  Principal = {}{}", sign, value);
    println!("  Present supply = {supply} USDC | borrow = {borrow} USDC");
    println!("  Supplied collateral {} UNI", format_units(collateral, 9));
    println!("  Balance {usdc_balance} USDC | {uni_balance} UNI");

    let basic = market_abi_calls::get_user_basic(&market, chad_address).await;
    let sign = if basic.principal.negative { "-" } else { "+" };
    let value = format_units(basic.principal.value, 6);
    let (supply, borrow) = market_abi_calls::get_user_supply_borrow(&market, chad_address).await;
    let supply = format_units(supply, 6);
    let borrow = format_units(borrow, 6);
    let usdc_balance = chad.get_asset_balance(&usdc_asset_id).await.unwrap() / 10u64.pow(6);
    let uni_balance = chad.get_asset_balance(&uni_asset_id).await.unwrap() / 10u64.pow(9);
    let collateral =
        market_abi_calls::get_user_collateral(&market, chad_address, uni_contract_id).await;
    println!("\nChad 🤵");
    println!("  Principal = {}{}", sign, value);
    println!("  Present supply = {supply} USDC | borrow = {borrow} USDC");
    println!("  Supplied collateral {} UNI", format_units(collateral, 9));
    println!("  Balance {usdc_balance} USDC | {uni_balance} UNI");
}

fn print_case_title(num: u8, name: &str, call: &str, amount: &str) {
    println!(
        r#"
==================== Case #{num} ====================
👛 Wallet: {name}
🤙 Call: {call}
💰 Amount: {amount}
"#
    );
}

fn print_title(title: &str) {
    println!(
        r#"

███████╗██╗    ██╗ █████╗ ██╗   ██╗     ██████╗  █████╗ ███╗   ██╗ ██████╗ 
██╔════╝██║    ██║██╔══██╗╚██╗ ██╔╝    ██╔════╝ ██╔══██╗████╗  ██║██╔════╝ 
███████╗██║ █╗ ██║███████║ ╚████╔╝     ██║  ███╗███████║██╔██╗ ██║██║  ███╗
╚════██║██║███╗██║██╔══██║  ╚██╔╝      ██║   ██║██╔══██║██║╚██╗██║██║   ██║
███████║╚███╔███╔╝██║  ██║   ██║       ╚██████╔╝██║  ██║██║ ╚████║╚██████╔╝
╚══════╝ ╚══╝╚══╝ ╚═╝  ╚═╝   ╚═╝        ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝                                                                         

🏁 {title} 🏁 
Market config: ./src/market.sw:293:5
"#
    );
}
