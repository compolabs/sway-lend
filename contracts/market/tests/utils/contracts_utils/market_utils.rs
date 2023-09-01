use fuels::programs::call_utils::TxDependencyExtension;
use serde::Deserialize;

use fuels::prelude::{abigen, Contract, LoadConfiguration, TxParameters, WalletUnlocked};
use fuels::programs::call_response::FuelCallResponse;
use fuels::types::{Address, Bits256, ContractId};
use rand::Rng;

abigen!(Contract(
    name = "MarketContract",
    abi = "out/debug/market-abi.json"
));

// TODO: Make it a class not to pass a contract instance in the arguments
pub mod market_abi_calls {

    use fuels::{
        prelude::{CallParameters, SettableContract},
        types::{AssetId, Bits256},
    };

    use super::*;

    pub async fn debug_increment_timestamp(
        market: &MarketContract<WalletUnlocked>,
    ) -> FuelCallResponse<()> {
        let res = market.methods().debug_increment_timestamp().call().await;
        res.unwrap()
    }

    pub async fn supply_base(
        market: &MarketContract<WalletUnlocked>,
        base_asset_id: AssetId,
        amount: u64,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        let call_params = CallParameters::default()
            .with_amount(amount)
            .with_asset_id(base_asset_id);

        let tx_params = TxParameters::default()
            .with_gas_limit(100_000_000)
            .with_gas_price(1);

        market
            .methods()
            .supply_base()
            .tx_params(tx_params)
            .call_params(call_params)
            .unwrap()
            .call()
            .await
    }

    pub async fn withdraw_base(
        market: &MarketContract<WalletUnlocked>,
        contract_ids: &[&dyn SettableContract],
        amount: u64,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        let tx_params = TxParameters::default()
            .with_gas_limit(100_000_000)
            .with_gas_price(1);
        market
            .methods()
            .withdraw_base(amount)
            .append_variable_outputs(1)
            .tx_params(tx_params)
            .with_contracts(contract_ids)
            .call()
            .await
    }

    pub async fn withdraw_collateral(
        market: &MarketContract<WalletUnlocked>,
        contract_ids: &[&dyn SettableContract],
        asset_id: Bits256,
        amount: u64,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        let tx_params = TxParameters::default()
            .with_gas_limit(100_000_000)
            .with_gas_price(1);
        market
            .methods()
            .withdraw_collateral(asset_id, amount)
            .tx_params(tx_params)
            .with_contracts(contract_ids)
            .append_variable_outputs(1)
            .call()
            .await
    }

    pub async fn supply_collateral(
        market: &MarketContract<WalletUnlocked>,
        asset_id: AssetId,
        amount: u64,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        let call_params = CallParameters::default()
            .with_amount(amount)
            .with_asset_id(asset_id);
        market
            .methods()
            .supply_collateral()
            .append_variable_outputs(1)
            .call_params(call_params)
            .unwrap()
            .call()
            .await
    }

    pub async fn get_user_collateral(
        market: &MarketContract<WalletUnlocked>,
        address: Address,
        asset_id: Bits256,
    ) -> u64 {
        let res = market
            .methods()
            .get_user_collateral(address, asset_id)
            .simulate()
            .await;
        res.unwrap().value
    }

    pub async fn available_to_borrow(
        market: &MarketContract<WalletUnlocked>,
        contract_ids: &[&dyn SettableContract],
        address: Address,
    ) -> u64 {
        let res = market
            .methods()
            .available_to_borrow(address)
            // .tx_params(TX_PARAMS)
            .with_contracts(contract_ids)
            .simulate()
            .await;
        res.unwrap().value
    }

    pub async fn get_user_supply_borrow(
        market: &MarketContract<WalletUnlocked>,
        address: Address,
    ) -> (u64, u64) {
        market
            .methods()
            .get_user_supply_borrow(address)
            // .tx_params(TX_PARAMS)
            .simulate()
            .await
            .unwrap()
            .value
    }
    pub async fn get_user_basic(
        market: &MarketContract<WalletUnlocked>,
        address: Address,
    ) -> UserBasic {
        let res = market.methods().get_user_basic(address).simulate().await;
        res.unwrap().value
    }
    pub async fn get_market_basics(market: &MarketContract<WalletUnlocked>) -> MarketBasics {
        let res = market.methods().get_market_basics().simulate().await;
        res.unwrap().value
    }
    pub async fn totals_collateral(
        market: &MarketContract<WalletUnlocked>,
        bits256: Bits256,
    ) -> u64 {
        let res = market.methods().totals_collateral(bits256).simulate().await;
        res.unwrap().value
    }
    pub async fn get_utilization(market: &MarketContract<WalletUnlocked>) -> u64 {
        market
            .methods()
            .get_utilization()
            // .tx_params(TX_PARAMS)
            .simulate()
            .await
            .unwrap()
            .value
    }
    pub async fn balance_of(market: &MarketContract<WalletUnlocked>, bits256: Bits256) -> u64 {
        let res = market.methods().balance_of(bits256).simulate().await;
        res.unwrap().value
    }

    pub async fn pause(
        contract: &MarketContract<WalletUnlocked>,
        config: &PauseConfiguration,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        contract.methods().pause(config.clone()).call().await
    }

    pub async fn collateral_value_to_sell(
        market: &MarketContract<WalletUnlocked>,
        contract_ids: &[&dyn SettableContract],
        asset_id: Bits256,
        collateral_amount: u64,
    ) -> u64 {
        market
            .methods()
            .collateral_value_to_sell(asset_id, collateral_amount)
            .tx_params(TxParameters::default().with_gas_price(1))
            .with_contracts(contract_ids)
            .simulate()
            .await
            .unwrap()
            .value
    }

    pub async fn buy_collateral(
        market: &MarketContract<WalletUnlocked>,
        contract_ids: &[&dyn SettableContract],
        base_asset_id: AssetId,
        amount: u64,
        asset_id: Bits256,
        min_amount: u64,
        recipient: Address,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        let call_params = CallParameters::default()
            .with_amount(amount)
            .with_asset_id(base_asset_id);
        market
            .methods()
            .buy_collateral(asset_id, min_amount, recipient)
            .tx_params(TxParameters::default().with_gas_price(1))
            .with_contracts(contract_ids)
            .call_params(call_params)
            .unwrap()
            .append_variable_outputs(2)
            .call()
            .await
    }

    pub async fn absorb(
        market: &MarketContract<WalletUnlocked>,
        contract_ids: &[&dyn SettableContract],
        addresses: Vec<Address>,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        market
            .methods()
            .absorb(addresses)
            .with_contracts(contract_ids)
            // .tx_params(TX_PARAMS)
            .call()
            .await
    }

    pub async fn is_liquidatable(
        market: &MarketContract<WalletUnlocked>,
        contract_ids: &[&dyn SettableContract],
        address: Address,
    ) -> bool {
        let res = market.methods().is_liquidatable(address);
        res.with_contracts(contract_ids)
            // .tx_params(TX_PARAMS)
            .simulate()
            .await
            .unwrap()
            .value
    }

    pub async fn get_collateral_reserves(
        market: &MarketContract<WalletUnlocked>,
        asset_id: Bits256,
    ) -> I64 {
        market
            .methods()
            .get_collateral_reserves(asset_id)
            // .tx_params(TX_PARAMS)
            .simulate()
            .await
            .unwrap()
            .value
    }
    pub async fn get_reserves(market: &MarketContract<WalletUnlocked>) -> I64 {
        market
            .methods()
            .get_reserves()
            // .tx_params(TX_PARAMS)
            .simulate()
            .await
            .unwrap()
            .value
    }
}

pub async fn deploy_market(
    wallet: &WalletUnlocked,
    market_configuration: MarketConfiguration,
    debug_step: Option<u64>, // only for local test
) -> MarketContract<WalletUnlocked> {
    let mut rng = rand::thread_rng();
    let salt = rng.gen::<[u8; 32]>();
    let configurables = MarketContractConfigurables::default()
        .with_MARKET_CONFIGURATION(Option::Some(market_configuration))
        .with_DEBUG_STEP(debug_step);
    let config = LoadConfiguration::default().with_configurables(configurables);
    let id = Contract::load_from("./out/debug/market.bin", config)
        .unwrap()
        .with_salt(salt)
        .deploy(wallet, TxParameters::default().with_gas_price(1))
        .await
        .unwrap();

    MarketContract::new(id, wallet.clone())
}

#[derive(Deserialize)]
struct MarketConfig {
    kink: u64,
    supply_per_second_interest_rate_slope_low: u64,
    supply_per_second_interest_rate_slope_high: u64,
    borrow_per_second_interest_rate_slope_low: u64,
    borrow_per_second_interest_rate_slope_high: u64,
    borrow_per_second_interest_rate_base: u64,
    store_front_price_factor: u64,
    base_tracking_supply_speed: u64,
    base_tracking_borrow_speed: u64,
    base_min_for_rewards: u64,
    base_borrow_min: u64,
    target_reserves: u64,
}

pub fn get_market_config(
    governor: Address,
    pause_guardian: Address,
    base_token_bits256: Bits256,
    base_token_decimals: u64,
    price_feed: ContractId,
    reward_token_bits256: Bits256,
) -> MarketConfiguration {
    let config_json_str = std::fs::read_to_string("tests/artefacts/config.json").unwrap();
    let config: MarketConfig = serde_json::from_str(&config_json_str).unwrap();

    MarketConfiguration {
        governor,
        pause_guardian,
        base_token: base_token_bits256,
        base_token_decimals,
        base_token_price_feed: price_feed,
        kink: config.kink, // decimals: 18
        supply_per_second_interest_rate_slope_low: config.supply_per_second_interest_rate_slope_low, // decimals: 18
        supply_per_second_interest_rate_slope_high: config
            .supply_per_second_interest_rate_slope_high, // decimals: 18
        borrow_per_second_interest_rate_slope_low: config.borrow_per_second_interest_rate_slope_low, // decimals: 18
        borrow_per_second_interest_rate_slope_high: config
            .borrow_per_second_interest_rate_slope_high, // decimals: 18
        borrow_per_second_interest_rate_base: config.borrow_per_second_interest_rate_base, // decimals: 18
        store_front_price_factor: config.store_front_price_factor, // decimals: 4
        base_tracking_supply_speed: config.base_tracking_supply_speed, // decimals 18
        base_tracking_borrow_speed: config.base_tracking_borrow_speed, // decimals 18
        base_min_for_rewards: config.base_min_for_rewards,         // decimals base_token_decimals
        base_borrow_min: config.base_borrow_min,                   // decimals: base_token_decimals
        target_reserves: config.target_reserves,                   // decimals: base_token_decimals
        reward_token: reward_token_bits256,
    }
}
