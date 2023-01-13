use fuels::prelude::TxParameters;
use fuels::signers::WalletUnlocked;
use fuels::tx::{Address, ContractId};

use crate::utils::local_tests_utils::market::{market_abi_calls, MarketContract, market_contract_mod};
use crate::utils::local_tests_utils::oracle::oracle_abi_calls;
use crate::utils::number_utils::format_units;
use crate::utils::{local_tests_utils::market, number_utils::parse_units};

#[tokio::test]
async fn main_test() {
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

    println!("\n");
    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;

    // =================================================
    // ==================== Case #0 ====================
    // 👛 Wallet: Bob
    // 🤙 Call: supply_base
    // 💰 Amount: 400.00 USDC
    print_case_title(0, "Bob", "supply_base", "400.00 USDC");

    // Transfer of 400 USDC to the Bob's wallet
    let amount = parse_units(400, usdc.config.decimals);
    let tx_params = TxParameters::default();
    admin
        .transfer(bob.address(), amount, usdc.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    // Сheck balance equal to 400 USDC
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

    // =================================================
    // ==================== Case #1 ====================
    // 👛 Wallet: Alice
    // 🤙 Call: supply_collateral
    // 💰 Amount: $200.00/40.00 UNI
    print_case_title(1, "Alice", "supply_collateral", "$200.00/40.00 UNI");
    // Transfer of 40 UNI to the Alice's wallet
    let amount = parse_units(40, uni.config.decimals);
    admin
        .transfer(alice.address(), amount, uni.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    // Сheck balance equal to 40 UNI
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

    // =================================================
    // ==================== Case #2 ====================
    // 👛 Wallet: Alice
    // 🤙 Call: withdraw_base
    // 💰 Amount: -50.00 USDC
    print_case_title(2, "Alice", "withdraw_base", "-50.00 USDC");
    // Alice calls withdraw_base
    let amount = parse_units(50, usdc.config.decimals);
    let inst = market.with_wallet(alice.clone()).unwrap();
    let cotarcts = [oracle.get_contract_id().clone()];
    let _res = market_abi_calls::withdraw_base(&inst, &cotarcts, amount)
        .await
        .unwrap();

    // USDC balance check
    let balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Transfer money back
    alice
        .transfer(admin.address(), amount, usdc.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;

    // =================================================
    // ==================== Case #3 ====================
    // 👛 Wallet: Chad
    // 🤙 Call: supply_collateral
    // 💰 Amount: $300.00/60.00 UNI
    print_case_title(3, "Chad", "supply_collateral", "$300.00/60.00 UNI");
    // Transfer of 60 UNI to the Chad's wallet
    let amount = parse_units(60, uni.config.decimals);
    admin
        .transfer(chad.address(), amount, uni.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    //Сheck balance equal to 60 UNI
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

    // =================================================
    // ==================== Case #4 ====================
    // 👛 Wallet: Chad
    // 🤙 Call: supply_base
    // 💰 Amount: 200.00 USDC
    print_case_title(4, "Chad", "supply_base", "200.00 USDC");
    // Transfer of 200 USDC to the Chad's wallet
    let amount = parse_units(200, usdc.config.decimals);
    let tx_params = TxParameters::default();
    admin
        .transfer(chad.address(), amount, usdc.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    //Сheck balance equal to 200 USDC
    let balance = chad.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Chad calls supply_base
    let inst = market.with_wallet(chad.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    //Сheck supply balance equal to 200 USDC
    let (supply_balance, _borrow_balance) =
        market_abi_calls::get_user_supply_borrow(&inst, Address::from(chad.address())).await;
    assert!(supply_balance == amount);

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;

    // =================================================
    // ==================== Case #5 ====================
    // 👛 Wallet: Alice
    // 🤙 Call: withdraw_base
    // 💰 Amount: -100.00 USDC
    print_case_title(5, "Alice", "withdraw_base", "-100 USDC");
    //Alice calls withdraw_base
    let amount = parse_units(100, usdc.config.decimals);
    let inst = market.with_wallet(alice.clone()).unwrap();

    market_abi_calls::withdraw_base(&inst, &cotarcts, amount)
        .await
        .unwrap();

    // USDC balance check
    let balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Transfer money back
    alice
        .transfer(admin.address(), amount, usdc.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;

    // =================================================
    // ==================== Case #6 ====================
    // 👛 Wallet: Chad
    // 🤙 Call: withdraw_base
    // 💰 Amount: -300.00 USDC
    print_case_title(6, "Chad", "withdraw_base", "-300.00 USDC");
    //Chad calls withdraw_base
    let amount = parse_units(300, usdc.config.decimals);
    let inst = market.with_wallet(chad.clone()).unwrap();
    let _res = market_abi_calls::withdraw_base(&inst, &cotarcts, amount)
        .await
        .unwrap();

    // USDC balance check
    let balance = chad.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Transfer money back
    chad.transfer(admin.address(), amount, usdc.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;

    // =================================================
    // ==================== Case #7 ====================
    // 👛 Wallet: Alice
    // 🤙 Call: supply_base
    // 💰 Amount: 150.061096 USDC
    print_case_title(7, "Alice", "supply_base", "150.061096 USDC");
    // Transfer of 150.061096 USDC to the Alice's wallet
    let amount = 150_061_096; //TODO: calculate this value
    let tx_params = TxParameters::default();
    admin
        .transfer(alice.address(), amount, usdc.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    //Сheck balance equal to 150.061096 USDC
    let balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Alice calls supply_base
    let inst = market.with_wallet(alice.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    //TODO:
    //Сheck supply balance equal to 0.061096 USDC
    let (supply_balance, _borrow_balance) =
        market_abi_calls::get_user_supply_borrow(&inst, Address::from(alice.address())).await;
    assert!(supply_balance == amount - 150_000_000);

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;

    // =================================================
    // ==================== Case #8 ====================
    // 👛 Wallet: Chad
    // 🤙 Call: supply_base
    // 💰 Amount: 100.00 USDC
    print_case_title(8, "Chad", "supply_base", "100.00 USDC");
    // Transfer of 100 USDC to the Chad's wallet
    let amount = parse_units(100, usdc.config.decimals);
    let tx_params = TxParameters::default();
    admin
        .transfer(chad.address(), amount, usdc.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    //Сheck balance equal to 100 USDC
    let balance = chad.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Chad calls supply_base
    let inst = market.with_wallet(chad.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    //TODO: Сheck
    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    // =================================================
    // ==================== Case #9 ====================
    // 👛 Wallet: Bob
    // 🤙 Call: withdraw_base
    // 💰 Amount: -400.058340 USDC
    print_case_title(9, "Bob", "withdraw_base", "-400.058340 USDC");
    //Bob calls withdraw_base
    // let amount = 400_058_340; //TODO: calculate this value
    let amount = 400_000_000; //FIXME: 400_000_000 -> 400_058_340
    let inst = market.with_wallet(bob.clone()).unwrap();
    let _res = market_abi_calls::withdraw_base(&inst, &cotarcts, amount)
        .await
        .unwrap();

    // USDC balance check
    let balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance == amount);

    // Transfer money back
    bob.transfer(admin.address(), amount, usdc.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;

    // =================================================
    // ==================== Case #10 ====================
    // 👛 Wallet: Alice
    // 🤙 Call: withdraw_collateral
    // 💰 Amount: -$200.00/40.00 UNI
    print_case_title(10, "Alice", "withdraw_collateral", "-$200.00/40.00 UNI");
    //Alice calls withdraw_base
    let amount = parse_units(40, uni.config.decimals);
    let inst = market.with_wallet(alice.clone()).unwrap();

    let res = market_abi_calls::withdraw_collateral(&inst, &cotarcts, uni.contract_id, amount)
        .await
        .unwrap();
    println!("logs = {:?}", res.get_logs_with_type::<market_contract_mod::I64>());
    println!("logs = {:?}", res.get_logs());
    // UNI balance check
    let balance = alice.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == amount);

    // Transfer money back
    alice
        .transfer(admin.address(), amount, uni.asset_id, tx_params)
        .await
        .expect("❌ Cannot transfer");

    debug_state(&market, &wallets, usdc.contract_id, uni.contract_id).await;
    // =================================================
    // ==================== Case #11 ====================
    // TODO
    // 👛 Wallet: Chad
    // 🤙 Call: withdraw_collateral
    // 💰 Amount: $300.00/60.00 UN
}

async fn debug_state(
    market: &MarketContract,
    wallets: &Vec<WalletUnlocked>,
    usdc_contract_id: ContractId,
    uni_contract_id: ContractId,
) {
    let alice = wallets[1].clone();
    let alice_address = Address::from(alice.address());

    let bob = wallets[2].clone();
    let bob_address = Address::from(bob.address());

    let chad = wallets[3].clone();
    let chad_address = Address::from(chad.address());

    let market_basic = market_abi_calls::get_market_basics(&market).await;
    let total_supply_base = market_basic.total_supply_base;
    let total_borrow_base = market_basic.total_borrow_base;
    let usdc_balance = market_abi_calls::balance_of(&market, usdc_contract_id).await;
    let uni_balance = market_abi_calls::balance_of(&market, uni_contract_id).await;
    let utilization = market_abi_calls::get_utilization(&market).await;
    let s_rate = market_basic.base_supply_index;
    let b_rate = market_basic.base_borrow_index;
    println!("Total supply {} USDC", format_units(total_supply_base, 6));
    println!("Total borrow {} USDC", format_units(total_borrow_base, 6));
    println!(
        "Total USDC balance = {} USDC",
        format_units(usdc_balance, 6)
    );
    println!("Total UNI balance = {} UNI", format_units(uni_balance, 9));
    println!("Utilization {}", format_units(utilization, 18));
    println!("sRate {}", format_units(s_rate, 18));
    println!("bRate {}", format_units(b_rate, 18));

    let basic = market_abi_calls::get_user_basic(&market, alice_address).await;
    let sign = if basic.principal.negative { "-" } else { "+" };
    let value = format_units(basic.principal.value, 6);
    let (supply, borrow) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;
    let uni_collateral =
        market_abi_calls::get_user_collateral(&market, alice_address, uni_contract_id).await;
    let total_collateral = market_abi_calls::totals_collateral(&market, uni_contract_id).await;
    println!("\nAlice");
    println!("Principal = {}{}", sign, value);
    println!("Present supply = {} USDC", format_units(supply, 6));
    println!("Present borrow = {} USDC", format_units(borrow, 6));
    println!(
        "Supplied collateral {} UNI",
        format_units(uni_collateral, 9)
    );
    println!("Total collateral {} UNI", format_units(total_collateral, 9));

    let basic = market_abi_calls::get_user_basic(&market, bob_address).await;
    let sign = if basic.principal.negative { "-" } else { "+" };
    let value = format_units(basic.principal.value, 6);
    let (supply, borrow) = market_abi_calls::get_user_supply_borrow(&market, bob_address).await;
    let uni_collateral =
        market_abi_calls::get_user_collateral(&market, bob_address, uni_contract_id).await;
    let total_collateral = market_abi_calls::totals_collateral(&market, uni_contract_id).await;
    println!("\nBob");
    println!("Principal = {}{}", sign, value);
    println!("Present supply = {} USDC", format_units(supply, 6));
    println!("Present borrow = {} USDC", format_units(borrow, 6));
    println!(
        "Supplied collateral {} UNI",
        format_units(uni_collateral, 9)
    );
    println!("Total collateral {} UNI", format_units(total_collateral, 9));

    let basic = market_abi_calls::get_user_basic(&market, chad_address).await;
    let sign = if basic.principal.negative { "-" } else { "+" };
    let value = format_units(basic.principal.value, 6);
    let (supply, borrow) = market_abi_calls::get_user_supply_borrow(&market, chad_address).await;
    let uni_collateral =
        market_abi_calls::get_user_collateral(&market, chad_address, uni_contract_id).await;
    let total_collateral = market_abi_calls::totals_collateral(&market, uni_contract_id).await;
    println!("\nChad");
    println!("Principal = {}{}", sign, value);
    println!("Present supply = {} USDC", format_units(supply, 6));
    println!("Present borrow = {} USDC", format_units(borrow, 6));
    println!(
        "Supplied collateral {} UNI",
        format_units(uni_collateral, 9)
    );
    println!("Total collateral {} UNI", format_units(total_collateral, 9));
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
