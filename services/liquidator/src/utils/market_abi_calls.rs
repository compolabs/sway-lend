pub mod market_abi_calls {

    use fuels::{prelude::TxParameters, programs::call_response::FuelCallResponse, types::Address};

    use crate::MarketContract;

    pub async fn absorb(
        market: &MarketContract,
        // contract_ids: &[&dyn SettableContract],
        addresses: Vec<Address>,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        market
            .methods()
            .absorb(addresses)
            // .set_contracts(contract_ids)
            .tx_params(TxParameters::new(Some(1), Some(100_000_000), None))
            .call()
            .await
    }

    pub async fn is_liquidatable(
        market: &MarketContract,
        // contract_ids: &[&dyn SettableContract],
        address: Address,
    ) -> bool {
        let tx_params = TxParameters::new(Some(1), Some(100_000_000), Some(0));
        let res = market.methods().is_liquidatable(address);
        res //.set_contracts(contract_ids)
            .tx_params(tx_params)
            .simulate()
            .await
            .unwrap()
            .value
    }
}
