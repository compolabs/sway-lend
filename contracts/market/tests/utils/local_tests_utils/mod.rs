pub mod market;

use crate::utils::number_utils::parse_units;
use fuels::contract::call_response::FuelCallResponse;
use fuels::prelude::*;
use rand::prelude::Rng;

abigen!(OracleContract, "tests/artefacts/oracle/oracle-abi.json");
abigen!(
    TokenContract,
    "tests/artefacts/token/token_contract-abi.json"
);

pub mod oracle_abi_calls {
    use std::collections::HashMap;

    use super::*;

    pub async fn initialize(contract: &OracleContract, owner: Address) -> FuelCallResponse<()> {
        contract.methods().initialize(owner).call().await.unwrap()
    }

    // pub async fn owner(contract: &OracleContract) -> Identity {
    //     contract.methods().owner().call().await.unwrap().value
    // }

    // pub async fn get_price(contract: &OracleContract, asset_id: ContractId) -> Price {
    //     contract
    //         .methods()
    //         .get_price(asset_id)
    //         .call()
    //         .await
    //         .unwrap()
    //         .value
    // }

    pub async fn sync_prices(contract: &OracleContract, assets: &HashMap<String, Asset>) {
        let client = reqwest::Client::new();
        let req = "https://api.coingecko.com/api/v3/simple/price?ids=binancecoin%2Cbitcoin%2Cbinance-usd%2Cusd-coin%2Ctether%2Cuniswap%2Cethereum%2Cchainlink&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false&precision=9";
        let body = client.get(req).send().await.unwrap().text().await.unwrap();
        let responce: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
        for (_, asset) in assets.iter() {
            let price = match responce[asset.coingeco_id.as_str()]["usd"].as_f64() {
                Some(p) => (p * 10f64.powf(9f64)).round() as u64,
                _ => asset.default_price,
            };
            set_price(contract, asset.asset_id, price).await;
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

pub struct DeployTokenConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub mint_amount: u64,
}

pub struct Asset {
    pub config: DeployTokenConfig,
    pub asset_id: ContractId,
    pub coingeco_id: String,
    pub instance: Option<TokenContract>,
    pub default_price: u64,
}

pub async fn get_oracle_contract_instance(wallet: &WalletUnlocked) -> OracleContract {
    let id = Contract::deploy(
        "./tests/artefacts/oracle/oracle.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::default(),
    )
    .await
    .unwrap();

    OracleContract::new(id, wallet.clone())
}

pub async fn get_token_contract_instance(
    wallet: &WalletUnlocked,
    deploy_config: &DeployTokenConfig,
) -> TokenContract {
    let mut name = deploy_config.name.clone();
    let mut symbol = deploy_config.symbol.clone();
    let decimals = deploy_config.decimals;

    let mut rng = rand::thread_rng();
    let salt = rng.gen::<[u8; 32]>();

    let id = Contract::deploy_with_parameters(
        "./tests/artefacts/token/token_contract.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::default(),
        Salt::from(salt),
    )
    .await
    .unwrap();

    let instance = TokenContract::new(id, wallet.clone());
    let methods = instance.methods();

    let mint_amount = parse_units(deploy_config.mint_amount, decimals);
    name.push_str(" ".repeat(32 - deploy_config.name.len()).as_str());
    symbol.push_str(" ".repeat(8 - deploy_config.symbol.len()).as_str());

    let config: token_contract_mod::Config = token_contract_mod::Config {
        name: fuels::core::types::SizedAsciiString::<32>::new(name).unwrap(),
        symbol: fuels::core::types::SizedAsciiString::<8>::new(symbol).unwrap(),
        decimals,
    };

    let _res = methods
        .initialize(config, mint_amount, Address::from(wallet.address()))
        .call()
        .await;
    let _res = methods.mint().append_variable_outputs(1).call().await;

    instance
}

pub async fn print_balances(wallet: &WalletUnlocked) {
    let balances = wallet.get_balances().await.unwrap();
    println!("{:#?}\n", balances);
}
