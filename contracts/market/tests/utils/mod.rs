use std::{str::FromStr};

use fuels::{
    signers::WalletUnlocked,
    tx::{Address, AssetId, ContractId},
};

use crate::utils::{
    local_tests_utils::{
        market::market_abi_calls,
        token::{token_abi_calls, TokenContract},
    },
    number_utils::format_units,
};

use self::local_tests_utils::market::MarketContract;

pub mod local_tests_utils;
pub mod number_utils;

pub fn print_title(title: &str) {
    println!(
        r#"

███████╗██╗    ██╗ █████╗ ██╗   ██╗     ██████╗  █████╗ ███╗   ██╗ ██████╗ 
██╔════╝██║    ██║██╔══██╗╚██╗ ██╔╝    ██╔════╝ ██╔══██╗████╗  ██║██╔════╝ 
███████╗██║ █╗ ██║███████║ ╚████╔╝     ██║  ███╗███████║██╔██╗ ██║██║  ███╗
╚════██║██║███╗██║██╔══██║  ╚██╔╝      ██║   ██║██╔══██║██║╚██╗██║██║   ██║
███████║╚███╔███╔╝██║  ██║   ██║       ╚██████╔╝██║  ██║██║ ╚████║╚██████╔╝
╚══════╝ ╚══╝╚══╝ ╚═╝  ╚═╝   ╚═╝        ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝                                                                         

🏁 {title} 🏁 
Market config: ./src/tests/market.sw:293:5
"#
    );
}

pub fn print_case_title(num: u8, name: &str, call: &str, amount: &str) {
    println!(
        r#"
==================== Case #{num} ====================
👛 Wallet: {name}
🤙 Call: {call}
💰 Amount: {amount}
"#
    );
}

pub async fn debug_state(
    market: &MarketContract,
    wallets: &Vec<WalletUnlocked>,
    usdc_contract_id: ContractId,
    collateral_contract_id: ContractId,
) {
    let usdc_asset_id = AssetId::from_str(usdc_contract_id.to_string().as_str()).unwrap();
    let collateral_asset_id =
        AssetId::from_str(collateral_contract_id.to_string().as_str()).unwrap();
    let collateral_inst = TokenContract::new(collateral_contract_id.into(), wallets[0].clone());
    let collateral_config = token_abi_calls::config(&collateral_inst).await;
    let collateral_decimals = collateral_config.decimals;
    let collateral_symbol = collateral_config.symbol.to_string();
    let collateral_symbol = collateral_symbol.trim();

    let alice = wallets[1].clone();
    let alice_address = Address::from(alice.address());

    let bob = wallets[2].clone();
    let bob_address = Address::from(bob.address());

    let chad = wallets[3].clone();
    let chad_address = Address::from(chad.address());

    let scale18 = 10u64.pow(18) as f64;

    let market_basic = market_abi_calls::get_market_basics(&market).await;
    let usdc_balance =
        market_abi_calls::balance_of(&market, usdc_contract_id).await as f64 / 10u64.pow(6) as f64;
    let collateral_balance = format_units(
        market_abi_calls::balance_of(&market, collateral_contract_id).await,
        collateral_decimals,
    );
    let utilization = market_abi_calls::get_utilization(&market).await as f64 / scale18;
    let s_rate = market_basic.base_supply_index as f64 / scale18;
    let b_rate = market_basic.base_borrow_index as f64 / scale18;
    let total_collateral =
        market_abi_calls::totals_collateral(&market, collateral_contract_id).await;
    let last_accrual_time = market_basic.last_accrual_time;
    let usdc_reservs = market_abi_calls::get_reserves(&market).await;
    let usdc_reservs = format!(
        "{}{} USDC",
        if usdc_reservs.negative { "-" } else { "+" },
        usdc_reservs.value as f64 / 10u64.pow(6) as f64
    );
    let collateral_reservs =
        market_abi_calls::get_collateral_reserves(&market, collateral_contract_id).await;
    let collateral_reservs = format!(
        "{}{} {collateral_symbol}",
        if collateral_reservs.negative {
            "-"
        } else {
            "+"
        },
        format_units(collateral_reservs.value, collateral_decimals)
    );
    let supply_base = market_basic.total_supply_base as f64 * s_rate / 10u64.pow(6) as f64;
    let borrow_base = market_basic.total_borrow_base as f64 * b_rate / 10u64.pow(6) as f64;
    let borrowers_amount = market.methods().get_borrowers_amount().simulate().await.unwrap().value;
    // let mut borrowers: Vec<Address> = vec![];
    // let mut i = 0;
    // while i < borrowers_amount {
    //     let borrower = market.methods().get_borrower(i).simulate().await.unwrap().value;
    //     borrowers.push(borrower);
    //     i += 1;
    // }
    println!("🏦 Market\n  Total supply {supply_base} USDC | Total borrow {borrow_base} USDC",);
    println!(
        "  Total USDC balance = {usdc_balance} USDC | Total {collateral_symbol} balance = {collateral_balance} {collateral_symbol}"
    );
    println!("  Reservs: {usdc_reservs} | {collateral_reservs}");
    println!("  sRate {s_rate} | bRate {b_rate}");
    println!(
        "  Total collateral {} {collateral_symbol}",
        format_units(total_collateral, collateral_decimals)
    );
    println!("  Utilization {utilization} | Last accrual time {last_accrual_time}",);
    println!("  Borrowers amount {borrowers_amount}\n");

    let basic = market_abi_calls::get_user_basic(&market, alice_address).await;
    let sign = if basic.principal.negative { "-" } else { "+" };
    let value = format_units(basic.principal.value, 6);
    let (supply, borrow) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;
    let supply = format_units(supply, 6);
    let borrow = format_units(borrow, 6);
    let usdc_balance =
        alice.get_asset_balance(&usdc_asset_id).await.unwrap() as f64 / 10u64.pow(6) as f64;
    let collateral_balance = alice.get_asset_balance(&collateral_asset_id).await.unwrap() as f64
        / 10u64.pow(collateral_decimals as u32) as f64;
    let collateral =
        market_abi_calls::get_user_collateral(&market, alice_address, collateral_contract_id).await;
    println!("\nAlice 🦹");
    println!("  Principal = {}{}", sign, value);
    println!("  Present supply = {supply} USDC | borrow = {borrow} USDC");
    println!(
        "  Supplied collateral {} {collateral_symbol}",
        format_units(collateral, collateral_decimals)
    );
    println!("  Balance {usdc_balance} USDC | {collateral_balance} {collateral_symbol}");

    let basic = market_abi_calls::get_user_basic(&market, bob_address).await;
    let sign = if basic.principal.negative { "-" } else { "+" };
    let value = format_units(basic.principal.value, 6);
    let (supply, borrow) = market_abi_calls::get_user_supply_borrow(&market, bob_address).await;
    let supply = format_units(supply, 6);
    let borrow = format_units(borrow, 6);
    let usdc_balance =
        bob.get_asset_balance(&usdc_asset_id).await.unwrap() as f64 / 10u64.pow(6) as f64;
    let collateral_balance = bob.get_asset_balance(&collateral_asset_id).await.unwrap() as f64
        / 10u64.pow(collateral_decimals as u32) as f64;
    let collateral =
        market_abi_calls::get_user_collateral(&market, bob_address, collateral_contract_id).await;
    println!("\nBob 🧛");

    println!("  Principal = {}{}", sign, value);
    println!("  Present supply = {supply} USDC | borrow = {borrow} USDC");
    println!(
        "  Supplied collateral {} {collateral_symbol}",
        format_units(collateral, collateral_decimals)
    );
    println!("  Balance {usdc_balance} USDC | {collateral_balance} {collateral_symbol}");

    let basic = market_abi_calls::get_user_basic(&market, chad_address).await;
    let sign = if basic.principal.negative { "-" } else { "+" };
    let value = format_units(basic.principal.value, 6);
    let (supply, borrow) = market_abi_calls::get_user_supply_borrow(&market, chad_address).await;
    let supply = format_units(supply, 6);
    let borrow = format_units(borrow, 6);
    let usdc_balance =
        chad.get_asset_balance(&usdc_asset_id).await.unwrap() as f64 / 10u64.pow(6) as f64;
    let collateral_balance = chad.get_asset_balance(&collateral_asset_id).await.unwrap() as f64
        / 10u64.pow(collateral_decimals as u32) as f64;
    let collateral =
        market_abi_calls::get_user_collateral(&market, chad_address, collateral_contract_id).await;
    println!("\nChad 🤵");
    println!("  Principal = {}{}", sign, value);
    println!("  Present supply = {supply} USDC | borrow = {borrow} USDC");
    println!(
        "  Supplied collateral {} {collateral_symbol}",
        format_units(collateral, collateral_decimals)
    );
    println!("  Balance {usdc_balance} USDC | {collateral_balance} {collateral_symbol}");
}
