use fuels::accounts::ViewOnlyAccount;
use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::BASE_ASSET_ID,
    test_helpers::{launch_custom_provider_and_get_wallets, WalletsConfig},
    types::{Address, AssetId, ContractId},
};
use src20_sdk::{deploy_token_contract, DeployTokenConfig};
use std::{collections::HashMap, fs, str::FromStr};

use crate::utils::contracts_utils::market_utils::market_abi_calls;
use crate::utils::number_utils::format_units;

use self::contracts_utils::market_utils::{AssetConfig, MarketContract};
use self::contracts_utils::token_utils::Asset;

pub mod contracts_utils;
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

pub async fn init_wallets() -> Vec<WalletUnlocked> {
    let config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    launch_custom_provider_and_get_wallets(config, None, None).await
}

pub async fn init_tokens(
    admin: &WalletUnlocked,
    price_feed: ContractId,
) -> (HashMap<String, Asset>, Vec<AssetConfig>) {
    let deploy_config_json_str = fs::read_to_string("tests/artefacts/tokens.json")
        .expect("Should have been able to read the file");
    let deploy_configs: serde_json::Value =
        serde_json::from_str(deploy_config_json_str.as_str()).unwrap();
    let deploy_configs = deploy_configs.as_array().unwrap();
    let mut assets: HashMap<String, Asset> = HashMap::new();
    let mut asset_configs: Vec<AssetConfig> = vec![];
    for config_value in deploy_configs {
        let config = DeployTokenConfig {
            name: String::from(config_value["name"].as_str().unwrap()),
            symbol: String::from(config_value["symbol"].as_str().unwrap()),
            decimals: config_value["decimals"].as_u64().unwrap() as u8,
        };

        let instance = if config.symbol != "ETH" {
            Some(deploy_token_contract(&admin, &config, "tests/artefacts/token/FRC20.bin").await)
        } else {
            None
        };
        let contract_id = match instance {
            Option::Some(instance) => ContractId::from(instance.contract_id()),
            Option::None => ContractId::from_str(BASE_ASSET_ID.to_string().as_str())
                .expect("Cannot parse BASE_ASSET_ID to contract id"),
        };
        if config_value["symbol"].as_str().unwrap() != String::from("USDC") {
            asset_configs.push(AssetConfig {
                asset: contract_id,
                decimals: config_value["decimals"].as_u64().unwrap() as u8,
                price_feed: price_feed,
                borrow_collateral_factor: config_value["borrow_collateral_factor"]
                    .as_u64()
                    .unwrap(), // decimals: 4
                liquidate_collateral_factor: config_value["liquidate_collateral_factor"]
                    .as_u64()
                    .unwrap(), // decimals: 4
                liquidation_penalty: config_value["liquidation_penalty"].as_u64().unwrap(), // decimals: 4
                supply_cap: config_value["supply_cap"].as_u64().unwrap(), // decimals: asset decimals
            });
        }

        assets.insert(
            String::from(config_value["symbol"].as_str().unwrap()),
            Asset {
                config,
                contract_id,
                asset_id: AssetId::from(*contract_id),
                default_price: config_value["default_price"].as_u64().unwrap_or(0) * 10u64.pow(9),
                instance: Option::None,
                decimals: config_value["decimals"].as_u64().unwrap() as u8,
                symbol: config_value["symbol"].as_str().unwrap().into(),
                coingeco_id: config_value["coingeco_id"].as_str().unwrap().into(),
            },
        );
    }
    (assets, asset_configs)
}

fn convertI64(value: contracts_utils::market_utils::I64) -> i64 {
    // let is_negative = value.underlying < 9223372036854775808u64;
    // let value = value.underlying - 9223372036854775808u64;
    value.underlying as i64 - 9223372036854775808u64 as i64
}

pub async fn debug_state(
    market: &MarketContract<WalletUnlocked>,
    wallets: &Vec<WalletUnlocked>,
    usdc: &Asset,
    collateral: &Asset,
) {
    let usdc_asset_id = usdc.asset_id;
    let collateral_asset_id = collateral.asset_id;
    let collateral_decimals = collateral.decimals;
    let collateral_symbol = collateral.symbol.clone();

    let alice = wallets[1].clone();
    let alice_address = Address::from(alice.address());

    let bob = wallets[2].clone();
    let bob_address = Address::from(bob.address());

    let chad = wallets[3].clone();
    let chad_address = Address::from(chad.address());

    let scale18 = 10u64.pow(18) as f64;

    let market_basic = market_abi_calls::get_market_basics(&market).await;
    let usdc_balance =
        market_abi_calls::balance_of(&market, usdc.contract_id).await as f64 / 10u64.pow(6) as f64;
    let collateral_balance = format_units(
        market_abi_calls::balance_of(&market, collateral.contract_id).await,
        collateral_decimals,
    );
    let utilization = market_abi_calls::get_utilization(&market).await as f64 / scale18;
    let s_rate = market_basic.base_supply_index as f64 / scale18;
    let b_rate = market_basic.base_borrow_index as f64 / scale18;
    let total_collateral =
        market_abi_calls::totals_collateral(&market, collateral.contract_id).await;
    let last_accrual_time = market_basic.last_accrual_time;
    let usdc_reservs = convertI64(market_abi_calls::get_reserves(&market).await);

    let usdc_reservs = format!("{} USDC", usdc_reservs as f64 / 10u64.pow(6) as f64);
    let collateral_reservs = convertI64(
        market_abi_calls::get_collateral_reserves(&market, collateral.contract_id).await,
    );
    let collateral_reservs = format!(
        "{} {collateral_symbol}",
        collateral_reservs as f64 / 10u64.pow(collateral_decimals as u32) as f64
    );
    let supply_base = market_basic.total_supply_base as f64 * s_rate / 10u64.pow(6) as f64;
    let borrow_base = market_basic.total_borrow_base as f64 * b_rate / 10u64.pow(6) as f64;
    let borrowers_amount = market
        .methods()
        .get_borrowers_amount()
        .simulate()
        .await
        .unwrap()
        .value;
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
    let (supply, borrow) = market_abi_calls::get_user_supply_borrow(&market, alice_address).await;
    let supply = format_units(supply, 6);
    let borrow = format_units(borrow, 6);
    let usdc_balance =
        alice.get_asset_balance(&usdc_asset_id).await.unwrap() as f64 / 10u64.pow(6) as f64;
    let collateral_balance = alice.get_asset_balance(&collateral_asset_id).await.unwrap() as f64
        / 10u64.pow(collateral_decimals as u32) as f64;
    let collateral_amount =
        market_abi_calls::get_user_collateral(&market, alice_address, collateral.contract_id).await;
    println!("\nAlice 🦹");
    println!("  Principal = {}", convertI64(basic.principal));
    println!("  Present supply = {supply} USDC | borrow = {borrow} USDC");
    println!(
        "  Supplied collateral {} {collateral_symbol}",
        format_units(collateral_amount, collateral_decimals)
    );
    println!("  Balance {usdc_balance} USDC | {collateral_balance} {collateral_symbol}");

    let basic = market_abi_calls::get_user_basic(&market, bob_address).await;
    let (supply, borrow) = market_abi_calls::get_user_supply_borrow(&market, bob_address).await;
    let supply = format_units(supply, 6);
    let borrow = format_units(borrow, 6);
    let usdc_balance =
        bob.get_asset_balance(&usdc_asset_id).await.unwrap() as f64 / 10u64.pow(6) as f64;
    let collateral_balance = bob.get_asset_balance(&collateral_asset_id).await.unwrap() as f64
        / 10u64.pow(collateral_decimals as u32) as f64;
    let collateral_amount =
        market_abi_calls::get_user_collateral(&market, bob_address, collateral.contract_id).await;
    println!("\nBob 🧛");

    println!("  Principal = {}", convertI64(basic.principal));
    println!("  Present supply = {supply} USDC | borrow = {borrow} USDC");
    println!(
        "  Supplied collateral {} {collateral_symbol}",
        format_units(collateral_amount, collateral_decimals)
    );
    println!("  Balance {usdc_balance} USDC | {collateral_balance} {collateral_symbol}");

    let basic = market_abi_calls::get_user_basic(&market, chad_address).await;
    let (supply, borrow) = market_abi_calls::get_user_supply_borrow(&market, chad_address).await;
    let supply = format_units(supply, 6);
    let borrow = format_units(borrow, 6);
    let usdc_balance =
        chad.get_asset_balance(&usdc_asset_id).await.unwrap() as f64 / 10u64.pow(6) as f64;
    let collateral_balance = chad.get_asset_balance(&collateral_asset_id).await.unwrap() as f64
        / 10u64.pow(collateral_decimals as u32) as f64;
    let collateral_amount =
        market_abi_calls::get_user_collateral(&market, chad_address, collateral.contract_id).await;
    println!("\nChad 🤵");
    println!("  Principal = {}", convertI64(basic.principal));
    println!("  Present supply = {supply} USDC | borrow = {borrow} USDC");
    println!(
        "  Supplied collateral {} {collateral_symbol}",
        format_units(collateral_amount, collateral_decimals)
    );
    println!("  Balance {usdc_balance} USDC | {collateral_balance} {collateral_symbol}");
}
