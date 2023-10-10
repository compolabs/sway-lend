use fuels::accounts::wallet::WalletUnlocked;
use fuels::prelude::Provider;
use std::str::FromStr;
use std::time::Duration;

use fuels::prelude::ViewOnlyAccount;
use fuels::types::{Address, ContractId};
use src20_sdk::{token_factory_abi_calls, TokenFactoryContract};
use tokio::time::sleep;

use crate::utils::contracts_utils::market_utils::{market_abi_calls, MarketContract};
use crate::utils::contracts_utils::oracle_utils::{oracle_abi_calls, OracleContract};
use crate::utils::contracts_utils::token_utils::load_tokens;
use crate::utils::number_utils::parse_units;
use crate::utils::{debug_state, print_case_title, print_title};

const RPC: &str = "beta-4.fuel.network";
const MARKET_ADDRESS: &str = "0x9d1c482f1ccf2be50e490a0e25c3e441d05358758a010325ea0eb50fcba20fe5";
const ORACLE_ADDRESS: &str = "0x8f7a76602f1fce4e4f20135a0ab4d22b3d9a230215ccee16c0980cf286aaa93c";
const FACTORY_ADDRESS: &str = "0xd8c627b9cd9ee42e2c2bd9793b13bc9f8e9aad32e25a99ea574f23c1dd17685a";

#[tokio::test]
async fn main_test() {
    dotenv::dotenv().ok();

    let scale_6 = 10u64.pow(6) as f64;
    let scale_9 = 10u64.pow(9) as f64;

    print_title("Main test with UNI");
    //--------------- WALLETS ---------------
    let provider = Provider::connect(RPC).await.unwrap();

    let admin_pk = std::env::var("ADMIN").unwrap().parse().unwrap();
    let admin = WalletUnlocked::new_from_private_key(admin_pk, Some(provider.clone()));

    let alice_pk = std::env::var("ALICE").unwrap().parse().unwrap();
    let alice = WalletUnlocked::new_from_private_key(alice_pk, Some(provider.clone()));
    let alice_address = Address::from(alice.address());

    let bob_pk = std::env::var("BOB").unwrap().parse().unwrap();
    let bob = WalletUnlocked::new_from_private_key(bob_pk, Some(provider.clone()));
    let bob_address = Address::from(bob.address());

    let chad_pk = std::env::var("CHAD").unwrap().parse().unwrap();
    let chad = WalletUnlocked::new_from_private_key(chad_pk, Some(provider.clone()));
    let chad_address = Address::from(chad.address());

    let wallets = vec![admin.clone(), alice.clone(), bob.clone(), chad.clone()];
    println!("alice address = {:?}", alice.address().to_string());
    println!("bob address = {:?}", bob.address().to_string());
    println!("chad address = {:?}", chad.address().to_string());

    //--------------- ORACLE ---------------

    let id = ContractId::from_str(ORACLE_ADDRESS).unwrap();
    let oracle: OracleContract<WalletUnlocked> = OracleContract::new(id, admin.clone());

    //--------------- TOKENS ---------------
    let id = ContractId::from_str(FACTORY_ADDRESS).unwrap();
    let factory: TokenFactoryContract<WalletUnlocked> =
        TokenFactoryContract::new(id, admin.clone());

    let (assets, _) = load_tokens("tests/artefacts/tokens.json", id).await;

    let usdc = assets.get("USDC").unwrap();
    let uni = assets.get("UNI").unwrap();

    //--------------- MARKET ---------------

    let id = ContractId::from_str(MARKET_ADDRESS).unwrap();
    let market = MarketContract::new(id, admin.clone());

    // ==================== Set oracle prices ====================
    for asset in &assets {
        let price = asset.1.default_price * 10u64.pow(9);
        oracle_abi_calls::set_price(&oracle, asset.1.bits256, price).await;
        // println!("1 {} = ${}", asset.1.symbol, asset.1.default_price);
    }

    // =================================================
    // ==================== Step #0 ====================
    // 👛 Wallet: Bob 🧛
    // 🤙 Call: supply_base
    // 💰 Amount: 100.00 USDC

    let amount = parse_units(100, usdc.decimals);
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(0, "Bob", "supply_base", log_amount.as_str());
    println!("💸 Bob + {log_amount}");

    // Transfer of 100 USDC to the Bob's wallet
    token_factory_abi_calls::mint(&factory, bob_address, &usdc.symbol, amount)
        .await
        .unwrap();

    let balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance >= amount);

    // Bob calls supply_base
    let inst = market.with_account(bob.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    // Сheck supply balance equal to 400 USDC
    let (supply_balance, _) = market_abi_calls::get_user_supply_borrow(&inst, bob_address).await;
    assert!(supply_balance >= amount);

    debug_state(&market, &wallets, usdc, uni).await;
    sleep(Duration::from_secs(10)).await;

    // =================================================
    // ==================== Step #1 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: supply_collateral
    // 💰 Amount: 40.00 UNI ~ $200.00

    let amount = parse_units(40, uni.decimals);
    let log_amount = format!("{} UNI", amount as f64 / scale_9);
    print_case_title(1, "Alice", "supply_collateral", log_amount.as_str());
    println!("💸 Alice + {log_amount}");

    // Transfer of 40 UNI to the Alice's wallet
    token_factory_abi_calls::mint(&factory, alice_address, &uni.symbol, amount)
        .await
        .unwrap();

    let balance = alice.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance >= amount);

    // Alice calls supply_collateral
    let inst = market.with_account(alice.clone()).unwrap();
    market_abi_calls::supply_collateral(&inst, uni.asset_id, amount)
        .await
        .unwrap();

    // Сheck supply balance equal to 40 UNI
    let res = market_abi_calls::get_user_collateral(&inst, alice_address, uni.bits256).await;
    assert!(res >= amount);

    debug_state(&market, &wallets, usdc, uni).await;
    sleep(Duration::from_secs(10)).await;

    // =================================================
    // ==================== Step #2 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_base
    // 💰 Amount: 50.00 USDC

    let amount = parse_units(50, usdc.decimals);
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(2, "Alice", "withdraw_base", log_amount.as_str());

    // Alice calls withdraw_base
    let inst = market.with_account(alice.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &[&oracle], amount)
        .await
        .unwrap();

    // USDC balance check
    let balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance >= amount);

    debug_state(&market, &wallets, usdc, uni).await;
    sleep(Duration::from_secs(10)).await;

    // =================================================
    // ==================== Step #3 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: supply_collateral
    // 💰 Amount: 60.00 UNI ~ $300.00

    let amount = parse_units(60, uni.decimals);
    let log_amount = format!("{} UNI", amount as f64 / scale_9);
    print_case_title(3, "Chad", "supply_collateral", log_amount.as_str());
    println!("💸 Chad + {log_amount}");

    // Transfer of 60 UNI to the Chad's wallet
    token_factory_abi_calls::mint(&factory, chad_address, &uni.symbol, amount)
        .await
        .unwrap();

    let balance = chad.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance >= amount);

    //Chad calls supply_collateral
    let inst = market.with_account(chad.clone()).unwrap();
    market_abi_calls::supply_collateral(&inst, uni.asset_id, amount)
        .await
        .unwrap();

    //Сheck supply balance equal to 60 UNI
    let res = market_abi_calls::get_user_collateral(&inst, chad_address, uni.bits256).await;
    assert!(res >= amount);

    debug_state(&market, &wallets, usdc, uni).await;
    sleep(Duration::from_secs(10)).await;

    // =================================================
    // ==================== Step #4 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: supply_base
    // 💰 Amount: 200.00 USDC

    let amount = parse_units(200, usdc.decimals);
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(4, "Chad", "supply_base", log_amount.as_str());
    println!("💸 Chad + {log_amount}");

    // Transfer of 200 USDC to the Chad's wallet
    token_factory_abi_calls::mint(&factory, chad_address, &usdc.symbol, amount)
        .await
        .unwrap();

    let balance = chad.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance >= amount);

    // Chad calls supply_base
    let inst = market.with_account(chad.clone()).unwrap();
    market_abi_calls::supply_base(&inst, usdc.asset_id, amount)
        .await
        .unwrap();

    //Сheck supply balance equal to 200 USDC
    let (supply_balance, _) = market_abi_calls::get_user_supply_borrow(&inst, chad_address).await;
    assert!(amount - 5 < supply_balance);

    debug_state(&market, &wallets, usdc, uni).await;
    sleep(Duration::from_secs(10)).await;

    // =================================================
    // ==================== Step #5 ====================
    // 👛 Wallet: Alice 🦹
    // 🤙 Call: withdraw_base
    // 💰 Amount: ~99.96 USDC (available_to_borrow)
    let amount = market_abi_calls::available_to_borrow(&market, &[&oracle], alice_address).await;
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(5, "Alice", "withdraw_base", log_amount.as_str());

    //Alice calls withdraw_base
    let inst = market.with_account(alice.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &[&oracle], amount)
        .await
        .unwrap();

    //available_to_borrow should be 0 and we cannout do withdraw_base more
    let res = market_abi_calls::available_to_borrow(&market, &[&oracle], alice_address).await;
    assert!(res == 0);
    let res = market_abi_calls::withdraw_base(&inst, &[&oracle], 1)
        .await
        .is_err();
    assert!(res);

    // USDC balance should be amount + 50 USDC from case #2
    let balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert!(balance >= amount + parse_units(50, usdc.decimals));

    debug_state(&market, &wallets, usdc, uni).await;
    sleep(Duration::from_secs(10)).await;

    // =================================================
    // ==================== Step #6 ====================
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
    sleep(Duration::from_secs(10)).await;

    // =================================================
    // ==================== Step #7 ====================
    // 👛 Wallet: Bob 🦹
    // 🤙 Call: absorb
    // 🔥 Target: Alice

    print_case_title(7, "Bob", "absorb", "Alice");

    assert!(market_abi_calls::is_liquidatable(&market, &[&oracle], alice_address).await);

    let inst = market.with_account(bob.clone()).unwrap();
    market_abi_calls::absorb(&inst, &[&oracle], vec![alice_address])
        .await
        .unwrap();

    //Check if absorb was ok
    let (_, borrow) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;
    assert!(borrow == 0);

    let amount = market_abi_calls::get_user_collateral(&market, alice_address, uni.bits256).await;
    assert!(amount == 0);

    debug_state(&market, &wallets, usdc, uni).await;
    sleep(Duration::from_secs(10)).await;

    // =================================================
    // ==================== Step #8 ====================
    // 👛 Wallet: Bob 🤵
    // 🤙 Call: buy_collateral
    // 💰 Amount: 172.44 USDC

    let inst = market.with_account(bob.clone()).unwrap();
    let reservs = market_abi_calls::get_collateral_reserves(&market, uni.bits256).await;
    assert!(!reservs.negative);

    let reservs = reservs.value;
    println!("reserves = {:?}", reservs);
    let amount =
        market_abi_calls::collateral_value_to_sell(&market, &[&oracle], uni.bits256, reservs).await;

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
        &[&oracle],
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
    assert!(balance == 40_000_000_000);

    debug_state(&market, &wallets, usdc, uni).await;
    sleep(Duration::from_secs(10)).await;

    // =================================================
    // ==================== Step #9 ====================
    // 👛 Wallet: Bob 🧛
    // 🤙 Call: withdraw_base
    // 💰 Amount: 100.021671 USDC

    let (amount, _) = market_abi_calls::get_user_supply_borrow(&market, bob_address).await;
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(9, "Bob", "withdraw_base", log_amount.as_str());

    //Bob calls withdraw_base
    let inst = market.with_account(bob.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &[&oracle], amount)
        .await
        .unwrap();

    // USDC balance check
    let (supplied, _) = market_abi_calls::get_user_supply_borrow(&market, bob_address).await;
    assert!(supplied == 0);
    assert!(bob.get_asset_balance(&usdc.asset_id).await.unwrap() == amount);

    debug_state(&market, &wallets, usdc, uni).await;
    sleep(Duration::from_secs(10)).await;

    // =================================================
    // ==================== Step #10 ====================
    // 👛 Wallet: Chad 🧛
    // 🤙 Call: withdraw_base
    // 💰 Amount: 200.0233392 USDC

    let (amount, _) = market_abi_calls::get_user_supply_borrow(&market, chad_address).await;
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(10, "Chad", "withdraw_base", log_amount.as_str());

    //Chad calls withdraw_base
    let inst = market.with_account(chad.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &[&oracle], amount)
        .await
        .unwrap();

    // USDC balance check
    let (supplied, _) = market_abi_calls::get_user_supply_borrow(&market, chad_address).await;
    assert!(supplied == 0);
    assert!(chad.get_asset_balance(&usdc.asset_id).await.unwrap() == amount);

    debug_state(&market, &wallets, usdc, uni).await;
    sleep(Duration::from_secs(10)).await;

    // =================================================
    // ==================== Step #11 ====================
    // 👛 Wallet: Alice 🧛
    // 🤙 Call: withdraw_base
    // 💰 Amount: 17.276598 USDC

    let (amount, _) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;
    let log_amount = format!("{} USDC", amount as f64 / scale_6);
    print_case_title(11, "Alice", "withdraw_base", log_amount.as_str());

    //Alice calls withdraw_base
    let inst = market.with_account(alice.clone()).unwrap();
    market_abi_calls::withdraw_base(&inst, &[&oracle], amount)
        .await
        .unwrap();

    // USDC balance check
    let (supplied, _) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;
    assert!(supplied == 0);

    debug_state(&market, &wallets, usdc, uni).await;
    sleep(Duration::from_secs(10)).await;

    // =================================================
    // ==================== Step #12 ====================
    // 👛 Wallet: Chad 🤵
    // 🤙 Call: withdraw_collateral
    // 💰 Amount: 270 UNI

    let amount = market_abi_calls::get_user_collateral(&market, chad_address, uni.bits256).await;
    let log_amount = format!("{} UNI", amount as f64 / scale_9);
    print_case_title(12, "Chad", "withdraw_collateral", log_amount.as_str());

    //Chad calls withdraw_base
    let inst = market.with_account(chad.clone()).unwrap();

    market_abi_calls::withdraw_collateral(&inst, &[&oracle], uni.bits256, amount)
        .await
        .unwrap();

    // UNI balance check
    let balance = chad.get_asset_balance(&uni.asset_id).await.unwrap();
    assert!(balance == amount);

    debug_state(&market, &wallets, usdc, uni).await;
}
