pub mod oracle_abi_calls {

    use fuels::{
        prelude::{TxParameters, WalletUnlocked},
        programs::call_response::FuelCallResponse,
        types::Bits256,
    };

    use crate::OracleContract;

    pub async fn set_prices(
        oracle: &OracleContract<WalletUnlocked>,
        prices: Vec<(Bits256, u64)>,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        let tx_params = TxParameters::default().with_gas_price(1);
        oracle
            .methods()
            .set_prices(prices)
            .tx_params(tx_params)
            .call()
            .await
    }
}
