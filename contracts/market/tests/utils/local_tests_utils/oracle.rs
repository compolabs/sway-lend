use fuels::{
    prelude::{abigen, Contract, SettableContract, StorageConfiguration, TxParameters},
    signers::WalletUnlocked,
};

abigen!(Contract(
    name = "OracleContract",
    abi = "tests/artefacts/oracle/oracle-abi.json"
));

pub mod oracle_abi_calls {
    use std::collections::HashMap;

    use fuels::{
        programs::call_response::FuelCallResponse,
        tx::{Address, ContractId},
    };

    use crate::utils::local_tests_utils::Asset;

    use super::*;

    pub fn get_as_settable_contract(contract: &OracleContract) -> [&dyn SettableContract; 1] {
        [contract]
    }

    pub async fn initialize(contract: &OracleContract, owner: Address) -> FuelCallResponse<()> {
        contract.methods().initialize(owner).call().await.unwrap()
    }

    pub async fn owner(contract: &OracleContract) -> Address {
        contract.methods().owner().simulate().await.unwrap().value
    }

    pub async fn get_price(contract: &OracleContract, asset_id: ContractId) -> Price {
        contract
            .methods()
            .get_price(asset_id)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn _sync_prices(contract: &OracleContract, assets: &HashMap<String, Asset>) {
        let client = reqwest::Client::new();
        let req = "https://api.coingecko.com/api/v3/simple/price?ids=compound-governance-token%2Cbinancecoin%2Cbitcoin%2Cbinance-usd%2Cusd-coin%2Ctether%2Cuniswap%2Cethereum%2Cchainlink&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false&precision=9";
        let body = client.get(req).send().await.unwrap().text().await.unwrap();
        let responce: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
        for (_, asset) in assets.iter() {
            let price = match responce[asset.coingeco_id.as_str()]["usd"].as_f64() {
                Some(p) => (p * 10f64.powf(9f64)).round() as u64,
                _ => asset.default_price,
            };
            set_price(contract, asset.contract_id, price).await;
        }
    }

    pub async fn set_price(
        contract: &OracleContract,
        asset_id: ContractId,
        new_price: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .set_price(asset_id, new_price)
            .call()
            .await
            .unwrap()
    }
}

pub async fn get_oracle_contract_instance(wallet: &WalletUnlocked) -> OracleContract {
    let id = Contract::deploy(
        "./tests/artefacts/oracle/oracle.bin",
        &wallet,
        TxParameters::new(Some(0), Some(100_000_000), Some(0)),
        StorageConfiguration::default(),
    )
    .await
    .unwrap();

    OracleContract::new(id, wallet.clone())
}
