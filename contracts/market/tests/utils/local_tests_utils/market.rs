use std::fs;
use std::{collections::HashMap, str::FromStr};

use crate::utils::local_tests_utils::*;
use fuels::contract::call_response::FuelCallResponse;
use fuels::prelude::*;

/*
use std::{collections::HashMap, str::FromStr};

use crate::utils::local_tests_utils::{
    market::{market_contract_mod::*, *},
    *,
};
use fuels::{
    prelude::BASE_ASSET_ID,
    tx::{Address, ContractId},
};

*/

abigen!(MarketContract, "out/debug/market-abi.json");

// TODO: Сделать это классом чтоб не передавать вечно асет айди
pub mod market_abi_calls {

    use super::*;

    pub async fn initialize(
        contract: &MarketContract,
        config: MarketConfiguration,
        assets: Vec<market_contract_mod::AssetConfig>,
    ) -> Result<FuelCallResponse<()>, Error> {
        contract.methods().initialize(config, assets).call().await
    }

    // pub async fn pause(contract: &MarketContract, config: PauseConfiguration) -> Result<FuelCallResponse<()>, Error> {
    // contract.methods().pause(config).call().await
    // }

    pub async fn supply_base(
        market: &MarketContract,
        base_asset_id: AssetId,
        amount: u64,
    ) -> Result<FuelCallResponse<()>, Error> {
        let call_params = CallParameters::new(Some(amount), Some(base_asset_id), None);
        let tx_params = TxParameters::new(Some(0), Some(100_000_000), Some(0));
        market
            .methods()
            .supply_base()
            .tx_params(tx_params)
            .call_params(call_params)
            .estimate_tx_dependencies(None)
            .await
            .unwrap()
            .call()
            .await
    }

    pub async fn withdraw_base(
        market: &MarketContract,
        lp_asset_id: AssetId,
        amount: u64,
    ) -> Result<FuelCallResponse<()>, Error> {
        let call_params = CallParameters::new(Some(amount), Some(lp_asset_id), None);
        let tx_params = TxParameters::new(Some(0), Some(100_000_000), Some(0));
        market
            .methods()
            .withdraw_base()
            .tx_params(tx_params)
            .call_params(call_params)
            .estimate_tx_dependencies(None)
            .await
            .unwrap()
            .call()
            .await
    }

    // pub async fn buy_collateral(
    //     market: &MarketContract,
    //     base_asset_id: AssetId,
    //     amount: u64,
    //     asset: ContractId,
    //     min_amount: u64,
    //     recipient: Address,
    // ) -> Result<FuelCallResponse<()>, Error> {
    //     let call_params = CallParameters::new(Some(amount), Some(base_asset_id), None);
    //     market
    //         .methods()
    //         .buy_collateral(asset, min_amount, recipient)
    //         .call_params(call_params)
    //         .estimate_tx_dependencies(None)
    //         .await
    //         .unwrap()
    //         .call()
    //         .await
    // }

    // pub async fn absorb(
    //     market: &MarketContract,
    //     addresses: Vec<Address>,
    // ) -> Result<FuelCallResponse<()>, Error> {
    //     market.methods().absorb(addresses).call().await
    // }

    pub async fn withdraw_collateral(
        market: &MarketContract,
        contract_ids: &[Bech32ContractId],
        asset: ContractId,
        amount: u64,
    ) -> Result<FuelCallResponse<()>, Error> {
        let tx_params = TxParameters::new(Some(0), Some(100_000_000), Some(0));
        market
            .methods()
            .withdraw_collateral(asset, amount)
            .tx_params(tx_params)
            .set_contracts(contract_ids)
            .estimate_tx_dependencies(None)
            .await
            .unwrap()
            .call()
            .await
    }

    pub async fn supply_collateral(
        market: &MarketContract,
        asset_id: AssetId,
        amount: u64,
    ) -> Result<FuelCallResponse<()>, Error> {
        let call_params = CallParameters::new(Some(amount), Some(asset_id), None);
        market
            .methods()
            .supply_collateral(Address::from(market.get_wallet().address()))
            .call_params(call_params)
            .estimate_tx_dependencies(None)
            .await
            .unwrap()
            .call()
            .await
    }

    pub async fn get_user_collateral(
        market: &MarketContract,
        address: Address,
        asset: ContractId,
    ) -> u64 {
        market
            .methods()
            .get_user_collateral(address, asset)
            .simulate()
            .await
            .unwrap()
            .value
    }

    // pub async fn get_configuration(
    //     contract: &MarketContract,
    // ) -> Result<CallResponse<MarketConfiguration>, Error> {
    //     contract.methods().get_configuration().simulate().await
    // }

    //     pub async fn get_market_state(_market: &MarketContract, _base_asset_id: AssetId) -> ContractId {
    //         let market_contact_id = ContractId::from(_market.get_contract_id());
    //         let methods = _market.methods();
    //         let market_address = AssetId::from(*_market.get_contract_id().hash());

    //         // AssetId::from(*market.get_contract_id().hash())
    //         let _balances = provider.get_balances(market_address).await?;
    //         // let base_asset_balance =
    //         return market_contact_id;

    //         // ////return these variables
    //         // //Total balance - base token кол-во у контракта
    //         // // let balance = methods.balance_of(contract_id(), config.base_token); // base_token_decimals
    //         // let balance = balance_of(market_contact_id, base_asset_id);
    //         // wallet.get_balances().await.unwrap();

    //         // return balance;

    //         //Total supply - долл эквивалент всех колл ассетов на контаркте - asset_configs
    //         // let total_supply =
    //         //     present_value_supply(base_supply_index_, storage.market_basic.total_supply_base); // base_token_decimals

    //         // //Total borrow - дол эвивалент взятого токена в залог
    //         // let total_borrow =
    //         //     present_value_borrow(base_borrow_index_, storage.market_basic.total_borrow_base); // base_token_decimals

    //         // //Total user collateral
    //         // let total_borrow =
    //         //     present_value_borrow(base_borrow_index_, storage.market_basic.total_borrow_base); // base_token_decimals

    //         // //Utilization - вызвать get_utilization
    //         // let utilization = get_utilization();

    //         //sRate - вызвать get_supply_rate
    //         // let s_rate = get_supply_rate();

    //         //todo return object with all data
    //     }
}

async fn init_wallets() -> Vec<WalletUnlocked> {
    launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(4),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await
}

pub async fn get_market_contract_instance(wallet: &WalletUnlocked) -> MarketContract {
    let id = Contract::deploy(
        "./out/debug/market.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/market-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    MarketContract::new(id, wallet.clone())
}

pub async fn setup_market() -> (
    Vec<WalletUnlocked>,
    HashMap<String, Asset>,
    MarketContract,
    OracleContract,
) {
    //--------------- WALLET ---------------
    let wallets = init_wallets().await;
    let address = Address::from(wallets[0].address());
    // println!("Wallet address {address}\n");

    //--------------- ORACLE ---------------
    let oracle_instance = get_oracle_contract_instance(&wallets[0]).await;
    let price_feed = ContractId::from(oracle_instance.get_contract_id());
    oracle_abi_calls::initialize(&oracle_instance, address).await;
    // oracle_abi_calls::sync_prices(&oracle_instance, &assets).await;

    //--------------- TOKENS ---------------

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
            Some(get_token_contract_instance(&wallets[0], &config).await)
        } else {
            None
        };
        let contract_id = match instance {
            Option::Some(instance) => ContractId::from(instance.get_contract_id()),
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
    let market_instance = get_market_contract_instance(&wallets[0]).await;

    let market_config = MarketConfiguration {
        governor: address,
        pause_guardian: address,
        base_token: assets.get("USDC").unwrap().contract_id,
        base_token_decimals: assets.get("USDC").unwrap().config.decimals,
        base_token_price_feed: price_feed,
        kink: 800000000000000000, // decimals: 18
        supply_per_second_interest_rate_slope_low: 10000000000, // decimals: 18
        supply_per_second_interest_rate_slope_high: 100000000000, // decimals: 18
        borrow_per_second_interest_rate_slope_low: 25000000000, // decimals: 18
        borrow_per_second_interest_rate_slope_high: 187500000000, // decimals: 18
        borrow_per_second_interest_rate_base: 15854895992, // decimals: 18
        store_front_price_factor: 6000, // decimals: 4
        base_tracking_supply_speed: 1868287030000000, // decimals 18
        base_tracking_borrow_speed: 3736574060000000, // decimals 18
        base_min_for_rewards: 20000000, // decimals base_token_decimals
        base_borrow_min: 10000000, // decimals: base_token_decimals
        target_reserves: 1000000000000, // decimals: base_token_decimals
        reward_token: assets.get("SWAY").unwrap().contract_id,
    };

    market_abi_calls::initialize(&market_instance, market_config, asset_configs)
        .await
        .expect("❌ Cannot initialize market");

    (wallets, assets, market_instance, oracle_instance)
}

// pub async fn user_market_state(
//     market: &MarketContract,
//     account: Address,
// ) -> Result<FuelCallResponse<()>, Error> {
//     market
//         .methods()
//         .get_user_basic(account)
//         .await
//         .unwrap()
//         .principal;
//     //todo спросить что такое present_value
//     // let user_present_value = market
//     //     .methods()
//     //     .get_user_basic(account)
//     //     .await
//     //     .unwrap().principal;
// }
