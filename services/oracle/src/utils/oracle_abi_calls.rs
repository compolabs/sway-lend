pub mod oracle_abi_calls {

    use fuels::{
        prelude::TxParameters, programs::call_response::FuelCallResponse, types::ContractId,
    };

    use crate::OracleContract;

    pub async fn set_prices(
        oracle: &OracleContract,
        prices: Vec<(ContractId, u64)>,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        let tx_params = TxParameters::default().set_gas_price(1);
        oracle
            .methods()
            .set_prices(prices)
            .tx_params(tx_params)
            .call()
            .await
    }
}
