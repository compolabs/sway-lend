use fuels::prelude::TxParameters;
use fuels::tx::Address;

use crate::utils::local_tests_utils::market::market_abi_calls;
use crate::utils::local_tests_utils::oracle::oracle_abi_calls;
use crate::utils::{local_tests_utils::market, number_utils::parse_units};

#[tokio::test]
async fn main_test() {
    let (walets, assets, market, oracle) = market::setup_market().await;

    // ==================== Assets ====================
    let usdc = assets.get("USDC").unwrap();
    let uni = assets.get("UNI").unwrap();

    // ==================== Wallets ====================
    let admin = walets[0].clone();
    let alice = walets[1].clone();
    let bob = walets[2].clone();
    let chad = walets[3].clone();

    // ==================== Set oracle prices ====================
    let amount = parse_units(1, 9); //1 USDC = $1
    oracle_abi_calls::set_price(&oracle, usdc.contract_id, amount).await;
    let res = oracle_abi_calls::get_price(&oracle, usdc.contract_id).await;
    assert!(res.price == amount);

    let amount = parse_units(5, 9); //1 UNI = $5
    oracle_abi_calls::set_price(&oracle, uni.contract_id, amount).await;
    let res = oracle_abi_calls::get_price(&oracle, uni.contract_id).await;
    assert!(res.price == amount);

    // ==================== Case #0 ====================
    // 👛 Wallet: Bob
    // 🤙 Call: supply_base
    // 💰 Amount: 400.00 USDC

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

    // ==================== Case #1 ====================
    // 👛 Wallet: Alice
    // 🤙 Call: supply_collateral
    // 💰 Amount: $200.00/40.00 UNI

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

    // FIXME
    // ==================== Case #2 ====================
    // 👛 Wallet: Alice
    // 🤙 Call: withdraw_base
    // 💰 Amount: -50.00 USDC

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

    // ==================== Case #3 ====================
    // 👛 Wallet: Chad
    // 🤙 Call: supply_collateral
    // 💰 Amount: $300.00/60.00 UNI

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

    // ==================== Case #4 ====================
    // 👛 Wallet: Chad
    // 🤙 Call: supply_base
    // 💰 Amount: 200.00 USDC

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

    // ==================== Case #5 ====================
    // 👛 Wallet: Alice
    // 🤙 Call: withdraw_base
    // 💰 Amount: -100.00 USDC

    //Alice calls withdraw_base
    // FIXME: replace 49 -> 100
    // let amount = parse_units(100, usdc.config.decimals);
    let amount = 49_999_999;
    let inst = market.with_wallet(alice.clone()).unwrap();
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
    // ==================== Case #6 ====================
    // TODO
    // 👛 Wallet: Chad
    // 🤙 Call: withdraw_base
    // 💰 Amount: -300.00 USDC

    //Alice calls withdraw_base
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
    // ==================== Case #7 ====================
    // TODO
    // 👛 Wallet: Alice
    // 🤙 Call: supply_base
    // 💰 Amount: 150.06 USDC

    // ========== CaseCase #8 ====================
    // TODO
    // 👛 Wallet: Chad
    // 🤙 Call: supply_base
    // 💰 Amount: 100.00 USDC

    // ========== CaseCase #9 ====================
    // TODO
    // 👛 Wallet: Bob
    // 🤙 Call: withdraw_base
    // 💰 Amount: -400.06 USDC

    // ========== CaseCase #10 ====================
    // TODO
    // 👛 Wallet: Alice
    // 🤙 Call: withdraw_collateral
    // 💰 Amount: $200.00/40.00 UN

    // ========== CaseCase #11 ====================
    // TODO
    // 👛 Wallet: Chad
    // 🤙 Call: withdraw_collateral
    // 💰 Amount: $300.00/60.00 UN
}
