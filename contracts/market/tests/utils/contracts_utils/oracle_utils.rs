use fuels::prelude::{abigen, Contract, LoadConfiguration, TxParameters, WalletUnlocked};

abigen!(Contract(
    name = "OracleContract",
    abi = "tests/artefacts/oracle/oracle-abi.json"
));

pub mod oracle_abi_calls {
    use std::collections::HashMap;

    use fuels::{programs::call_response::FuelCallResponse, types::Bits256};

    use crate::utils::contracts_utils::token_utils::Asset;

    use super::*;

    pub async fn get_price(contract: &OracleContract<WalletUnlocked>, asset_id: Bits256) -> Price {
        contract
            .methods()
            .get_price(asset_id)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn _sync_prices(
        contract: &OracleContract<WalletUnlocked>,
        assets: &HashMap<String, Asset>,
    ) {
        let client = reqwest::Client::new();
        let req = "https://api.coingecko.com/api/v3/simple/price?ids=compound-governance-token%2Cbinancecoin%2Cbitcoin%2Cbinance-usd%2Cusd-coin%2Ctether%2Cuniswap%2Cethereum%2Cchainlink&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false&precision=9";
        let body = client.get(req).send().await.unwrap().text().await.unwrap();
        let responce: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
        for (_, asset) in assets.iter() {
            let price = match responce[asset.coingeco_id.as_str()]["usd"].as_f64() {
                Some(p) => (p * 10f64.powf(9f64)).round() as u64,
                _ => asset.default_price,
            };
            set_price(contract, asset.bits256, price).await;
        }
    }

    pub async fn set_price(
        contract: &OracleContract<WalletUnlocked>,
        asset_id: Bits256,
        new_price: u64,
    ) -> FuelCallResponse<()> {
        // let bits256 = Bits256::from_hex_str(&asset_id.to_string()).unwrap();
        contract
            .methods()
            .set_price(asset_id, new_price)
            .call()
            .await
            .unwrap()
    }
}

pub async fn deploy_oracle(wallet: &WalletUnlocked) -> OracleContract<WalletUnlocked> {
    let configurables = OracleContractConfigurables::default().with_ADMIN(wallet.address().into());
    let id = Contract::load_from(
        "./tests/artefacts/oracle/oracle.bin",
        LoadConfiguration::default().with_configurables(configurables),
    )
    .unwrap()
    .deploy(wallet, TxParameters::default().with_gas_price(1))
    .await
    .unwrap();

    OracleContract::new(id, wallet.clone())
}
