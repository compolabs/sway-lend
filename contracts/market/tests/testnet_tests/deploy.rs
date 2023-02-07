use std::{collections::HashMap, fs, str::FromStr};

use dotenv::dotenv;
use fuels::prelude::{
    Address, AssetId, Bech32ContractId, ContractId, Provider, WalletUnlocked, BASE_ASSET_ID,
};

use crate::utils::{
    local_tests_utils::{
        market::{abigen_bindings::market_contract_mod, deploy_market_contract, market_abi_calls},
        oracle::OracleContract,
        token::TokenContract,
        Asset, DeployTokenConfig,
    },
    number_utils::parse_units,
};

const RPC: &str = "node-beta-2.fuel.network";
const ORACLE_ADDRESS: &str = "0xde764394c83bb3c8a3aec5f75f383ff86e64728964fab4469df5910ca01b1a59";

#[tokio::test]
async fn deploy() {
    //--------------- WALLET ---------------
    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    dotenv().ok();
    let secret = match std::env::var("SECRET") {
        Ok(s) => s,
        Err(error) => panic!("❌ Cannot find .env file: {:#?}", error),
    };

    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider));
    let address = Address::from(wallet.address());

    //--------------- ORACLE ---------------
    let oracle_dapp_id = Bech32ContractId::from(ContractId::from_str(ORACLE_ADDRESS).unwrap());
    let oracle = OracleContract::new(oracle_dapp_id, wallet.clone());
    let price_feed = ContractId::from(oracle.contract_id());
    //     //--------------- TOKENS ---------------

    let deploy_config_json_str = fs::read_to_string("tests/utils/local_tests_utils/tokens.json")
        .expect("Should have been able to read the file");
    let deploy_configs: serde_json::Value =
        serde_json::from_str(deploy_config_json_str.as_str()).unwrap();
    let deploy_configs = deploy_configs.as_array().unwrap();
    let mut assets: HashMap<String, Asset> = HashMap::new();
    let mut asset_configs: Vec<market_contract_mod::AssetConfig> = Vec::new();
    for config_value in deploy_configs {
        let config = DeployTokenConfig {
            name: String::from(config_value["name"].as_str().unwrap()),
            symbol: String::from(config_value["symbol"].as_str().unwrap()),
            decimals: config_value["decimals"].as_u64().unwrap() as u8,
            mint_amount: config_value["mint_amount"].as_u64().unwrap(),
        };

        let instance = if config.symbol != "ETH" {
            let contract_id = ContractId::from_str(config_value["asset_id"].as_str().unwrap());
            let bech32_id = Bech32ContractId::from(contract_id.unwrap());
            let instance = TokenContract::new(bech32_id, wallet.clone());
            Some(instance)
        } else {
            None
        };
        let contract_id = match instance {
            Option::Some(instance) => ContractId::from(instance.contract_id()),
            Option::None => ContractId::from_str(BASE_ASSET_ID.to_string().as_str())
                .expect("Cannot parse BASE_ASSET_ID to contract id"),
        };

        assets.insert(
            String::from(config_value["symbol"].as_str().unwrap()),
            Asset {
                config,
                contract_id,
                asset_id: AssetId::from(*contract_id),
                coingeco_id: String::from(config_value["coingeco_id"].as_str().unwrap()),
                default_price: parse_units(config_value["default_price"].as_u64().unwrap(), 9),
                instance: Option::None,
            },
        );

        if config_value["symbol"].as_str().unwrap() != String::from("USDC") {
            asset_configs.push(market_contract_mod::AssetConfig {
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
            })
        }
    }

    //--------------- MARKET ---------------
    let market_instance = deploy_market_contract(&wallet).await;

    let config_json_str = fs::read_to_string("tests/utils/local_tests_utils/config.json")
        .expect("Should have been able to read the file");
    let config: serde_json::Value = serde_json::from_str(config_json_str.as_str()).unwrap();
    let config = config.as_object().unwrap();

    let market_config = market_contract_mod::MarketConfiguration {
        governor: address,
        pause_guardian: address,
        base_token: assets.get("USDC").unwrap().contract_id,
        base_token_decimals: assets.get("USDC").unwrap().config.decimals,
        base_token_price_feed: price_feed,
        kink: config["kink"].as_u64().unwrap(), // decimals: 18
        supply_per_second_interest_rate_slope_low: config
            ["supply_per_second_interest_rate_slope_low"]
            .as_u64()
            .unwrap(), // decimals: 18
        supply_per_second_interest_rate_slope_high: config
            ["supply_per_second_interest_rate_slope_high"]
            .as_u64()
            .unwrap(), // decimals: 18
        borrow_per_second_interest_rate_slope_low: config
            ["borrow_per_second_interest_rate_slope_low"]
            .as_u64()
            .unwrap(), // decimals: 18
        borrow_per_second_interest_rate_slope_high: config
            ["borrow_per_second_interest_rate_slope_high"]
            .as_u64()
            .unwrap(), // decimals: 18
        borrow_per_second_interest_rate_base: config["borrow_per_second_interest_rate_base"]
            .as_u64()
            .unwrap(), // decimals: 18
        store_front_price_factor: config["store_front_price_factor"].as_u64().unwrap(), // decimals: 4
        base_tracking_supply_speed: config["base_tracking_supply_speed"].as_u64().unwrap(), // decimals 18
        base_tracking_borrow_speed: config["base_tracking_borrow_speed"].as_u64().unwrap(), // decimals 18
        base_min_for_rewards: config["base_min_for_rewards"].as_u64().unwrap(), // decimals base_token_decimals
        base_borrow_min: config["base_borrow_min"].as_u64().unwrap(), // decimals: base_token_decimals
        target_reserves: config["target_reserves"].as_u64().unwrap(), // decimals: base_token_decimals
        reward_token: assets.get("SWAY").unwrap().contract_id,
    };

    market_abi_calls::initialize(
        &market_instance,
        &market_config,
        &asset_configs,
        Option::None,
    )
    .await
    .expect("❌ Cannot initialize market");
    println!(
        "Market contract = {} {}",
        market_instance.contract_id().hash(),
        market_instance.contract_id()
    );
}
