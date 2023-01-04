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
pub mod market_abi_calls {

    use std::borrow::Borrow;

    use super::*;

    pub async fn initialize(
        contract: &MarketContract,
        config: MarketConfiguration,
    ) -> Result<FuelCallResponse<()>, Error> {
        contract.methods().initialize(config).call().await
    }

    // pub async fn pause(contract: &MarketContract, config: PauseConfiguration) -> Result<FuelCallResponse<()>, Error> {
    // contract.methods().pause(config).call().await
    // }

    pub async fn supply_base(
        market: &MarketContract,
        asset_id: AssetId,
        amount: u64,
    ) -> Result<FuelCallResponse<()>, Error> {
        let call_params = CallParameters::new(Some(amount), Some(asset_id), None);
        market
            .methods()
            .supply_base()
            .call_params(call_params)
            .estimate_tx_dependencies(None)
            .await
            .unwrap()
            .call()
            .await
    }

    // pub async fn borrow(
    //     market: &MarketContract,
    //     asset_id: AssetId,
    //     amount: u64,
    // ) -> Result<FuelCallResponse<()>, Error> {
    //     let call_params = CallParameters::new(Some(amount), Some(asset_id), None);
    //     market
    //         .methods()
    //         .withdraw_base()
    //         .call_params(call_params)
    //         .estimate_tx_dependencies(None)
    //         .await
    //         .unwrap()
    //         .call()
    //         .await
    // }

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

    // pub async fn get_configuration(
    //     contract: &MarketContract,
    // ) -> Result<CallResponse<MarketConfiguration>, Error> {
    //     contract.methods().get_configuration().simulate().await
    // }
}

pub async fn get_market_contract_instance(wallet: &WalletUnlocked) -> MarketContract {
    let id = Contract::deploy(
        "./out/debug/market.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::default(),
    )
    .await
    .unwrap();

    MarketContract::new(id, wallet.clone())
}

pub async fn setup_market() -> (
    WalletUnlocked,
    HashMap<String, Asset>,
    MarketContract,
    OracleContract,
) {
    //--------------- WALLET ---------------
    let wallet = init_wallet().await;
    let address = Address::from(wallet.address());
    // println!("Wallet address {address}\n");

    //--------------- TOKENS ---------------
    let eth_config = DeployTokenConfig {
        name: String::from("Etherium"),
        symbol: String::from("ETH"),
        decimals: 9,
        mint_amount: 1,
    };

    let usdc_config = DeployTokenConfig {
        name: String::from("USD Coin"),
        symbol: String::from("USDC"),
        decimals: 6,
        mint_amount: 10000,
    };
    let usdc_instance = get_token_contract_instance(&wallet, &usdc_config).await;

    let link_config = DeployTokenConfig {
        name: String::from("Chainlink"),
        symbol: String::from("LINK"),
        decimals: 9,
        mint_amount: 1000,
    };
    let link_instance = get_token_contract_instance(&wallet, &link_config).await;

    let btc_config = DeployTokenConfig {
        name: String::from("Bitcoin"),
        symbol: String::from("BTC"),
        decimals: 8,
        mint_amount: 1,
    };
    let btc_instance = get_token_contract_instance(&wallet, &btc_config).await;

    let uni_config = DeployTokenConfig {
        name: String::from("Uniswap"),
        symbol: String::from("UNI"),
        decimals: 9,
        mint_amount: 1000,
    };
    let uni_instance = get_token_contract_instance(&wallet, &uni_config).await;

    let sway_config = DeployTokenConfig {
        name: String::from("Sway Lend Token"),
        symbol: String::from("SWAY"),
        decimals: 9,
        mint_amount: 1000,
    };
    let sway_instance = get_token_contract_instance(&wallet, &sway_config).await;

    //--------------- ORACLE ---------------
    let mut assets: HashMap<String, Asset> = HashMap::new();
    assets.insert(
        String::from("ETH"),
        Asset {
            config: eth_config,
            asset_id: ContractId::from_str(BASE_ASSET_ID.to_string().as_str())
                .expect("Cannot parse BASE_ASSET_ID to contract id"),
            coingeco_id: String::from(String::from("ethereum")),
            default_price: parse_units(1200, 9),
            instance: Option::None,
        },
    );
    assets.insert(
        String::from("USDC"),
        Asset {
            config: usdc_config,
            asset_id: ContractId::from(usdc_instance.get_contract_id()),
            coingeco_id: String::from("usd-coin"),
            default_price: parse_units(1, 9),
            instance: Option::Some(usdc_instance),
        },
    );
    assets.insert(
        String::from("LINK"),
        Asset {
            config: link_config,
            asset_id: ContractId::from(link_instance.get_contract_id()),
            coingeco_id: String::from("chainlink"),
            default_price: parse_units(5, 9),
            instance: Option::Some(link_instance),
        },
    );
    assets.insert(
        String::from("BTC"),
        Asset {
            config: btc_config,
            asset_id: ContractId::from(btc_instance.get_contract_id()),
            coingeco_id: String::from("bitcoin"),
            default_price: parse_units(19000, 9),
            instance: Option::Some(btc_instance),
        },
    );
    assets.insert(
        String::from("UNI"),
        Asset {
            config: uni_config,
            asset_id: ContractId::from(uni_instance.get_contract_id()),
            coingeco_id: String::from("uniswap"),
            default_price: parse_units(5, 9),
            instance: Option::Some(uni_instance),
        },
    );
    assets.insert(
        String::from("SWAY"),
        Asset {
            config: sway_config,
            asset_id: ContractId::from(sway_instance.get_contract_id()),
            coingeco_id: String::from("uniswap"),
            default_price: parse_units(300, 9),
            instance: Option::Some(sway_instance),
        },
    );
    let oracle_instance = get_oracle_contract_instance(&wallet).await;
    let price_feed = ContractId::from(oracle_instance.get_contract_id());
    oracle_abi_calls::initialize(&oracle_instance, address).await;
    oracle_abi_calls::sync_prices(&oracle_instance, &assets).await;

    //--------------- MARKET ---------------
    let market_instance = get_market_contract_instance(&wallet).await;

    let market_config = MarketConfiguration {
        governor: address,
        pause_guardian: address,
        base_token: assets.get("USDC").unwrap().asset_id,
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
        reward_token: assets.get("SWAY").unwrap().asset_id,
        asset_configs: vec![
            market_contract_mod::AssetConfig {
                asset: assets.get("LINK").unwrap().asset_id,
                decimals: assets.get("LINK").unwrap().config.decimals,
                price_feed: price_feed,
                borrow_collateral_factor: 7900,    // decimals: 4
                liquidate_collateral_factor: 8500, // decimals: 4
                liquidation_penalty: 700,          // decimals: 4
                supply_cap: 200000000000000,       // decimals: asset decimals
            },
            market_contract_mod::AssetConfig {
                asset: assets.get("UNI").unwrap().asset_id,
                decimals: assets.get("UNI").unwrap().config.decimals,
                price_feed: price_feed,
                borrow_collateral_factor: 7500,    // decimals: 4
                liquidate_collateral_factor: 8100, // decimals: 4
                liquidation_penalty: 700,          // decimals: 4
                supply_cap: 200000000000000,       // decimals: asset decimals
            },
            market_contract_mod::AssetConfig {
                asset: assets.get("BTC").unwrap().asset_id,
                decimals: assets.get("BTC").unwrap().config.decimals,
                price_feed: price_feed,
                borrow_collateral_factor: 7000,    // decimals: 4
                liquidate_collateral_factor: 7700, // decimals: 4
                liquidation_penalty: 500,          // decimals: 4
                supply_cap: 1000000000000,         // decimals: asset decimals
            },
            market_contract_mod::AssetConfig {
                asset: assets.get("ETH").unwrap().asset_id,
                decimals: assets.get("ETH").unwrap().config.decimals,
                price_feed: price_feed,
                borrow_collateral_factor: 8300,    // decimals: 4
                liquidate_collateral_factor: 9000, // decimals: 4
                liquidation_penalty: 500,          // decimals: 4
                supply_cap: 100000000000000,       // decimals: asset decimals
            },
        ],
    };

    market_abi_calls::initialize(&market_instance, market_config)
        .await
        .expect("❌ Cannot initialize market");

    //FIXME: not implemented: Cannot decode Vectors until we get support from the compiler.
    // let _res = market_abi_calls::get_configuration(&market_instance)
    //     .await
    //     .expect("❌ Cannot read configuration")
    //     .value;
    // println!("Market config:\n{:#?}", _res);
    (wallet, assets, market_instance, oracle_instance)
}
