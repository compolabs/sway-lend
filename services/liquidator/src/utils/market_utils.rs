use self::abigen_bindings::market_contract_mod;
use fuels::programs::call_utils::TxDependencyExtension;

use fuels::prelude::{abigen, TxParameters, WalletUnlocked};
use fuels::programs::call_response::FuelCallResponse;
use fuels::types::Address;

abigen!(Contract(
    name = "MarketContract",
    abi = "src/artefacts/market-abi.json"
),);

pub mod market_abi_calls {

    use fuels::{
        prelude::{CallParameters, SettableContract},
        types::{AssetId, Bits256},
    };

    use super::*;

    pub async fn collateral_value_to_sell(
        market: &MarketContract<WalletUnlocked>,
        contract_ids: &[&dyn SettableContract],
        asset_id: Bits256,
        collateral_amount: u64,
    ) -> u64 {
        let asset_id = market_contract_mod::AssetId { value: asset_id };
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
        let asset_id = market_contract_mod::AssetId { value: asset_id };
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
            .tx_params(TxParameters::default().with_gas_price(1))
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
        let asset_id = market_contract_mod::AssetId { value: asset_id };
        market
            .methods()
            .get_collateral_reserves(asset_id)
            .simulate()
            .await
            .unwrap()
            .value
    }
}
