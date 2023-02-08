use crate::utils::number_utils::parse_units;
use fuels::{
    prelude::{abigen, Contract, StorageConfiguration, TxParameters},
    signers::WalletUnlocked,
    tx::{Address, Salt},
    types::SizedAsciiString,
};
use rand::prelude::Rng;

use super::DeployTokenConfig;

abigen!(Contract(
    name = "TokenContract",
    abi = "tests/artefacts/token/token_contract-abi.json"
));

pub mod token_abi_calls {

    use fuels::{prelude::BASE_ASSET_ID, programs::call_response::FuelCallResponse};

    use super::*;

    pub async fn mint(c: &TokenContract) -> FuelCallResponse<()> {
        let res = c.methods().mint().append_variable_outputs(1).call().await;
        res.unwrap()
    }
    pub async fn mint_and_transfer(
        c: &TokenContract,
        amount: u64,
        recipient: Address,
    ) -> FuelCallResponse<()> {
        let res = c
            .methods()
            .mint_and_transfer(amount, recipient)
            .append_variable_outputs(1);

        res.call().await.unwrap()
    }
    pub async fn initialize(
        c: &TokenContract,
        config: TokenInitializeConfig,
        mint_amount: u64,
        address: Address,
    ) -> FuelCallResponse<()> {
        c.methods()
            .initialize(config, mint_amount, address)
            .call()
            .await
            .expect("❌ Cannot initialize token")
    }
    pub async fn config(c: &TokenContract) -> TokenInitializeConfig {
        if c.contract_id().hash().to_string() == BASE_ASSET_ID.to_string() {
            let mut name = String::from("Etherium");
            let mut symbol = String::from("ETH");
            name.push_str(" ".repeat(32 - name.len()).as_str());
            symbol.push_str(" ".repeat(8 - symbol.len()).as_str());

            TokenInitializeConfig {
                name: SizedAsciiString::<32>::new(name).unwrap(),
                symbol: SizedAsciiString::<8>::new(symbol).unwrap(),
                decimals: 9,
            }
        } else {
            c.methods().config().simulate().await.unwrap().value
        }
    }
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

    let mint_amount = parse_units(deploy_config.mint_amount, decimals);
    name.push_str(" ".repeat(32 - deploy_config.name.len()).as_str());
    symbol.push_str(" ".repeat(8 - deploy_config.symbol.len()).as_str());

    let config: TokenInitializeConfig = TokenInitializeConfig {
        name: SizedAsciiString::<32>::new(name).unwrap(),
        symbol: SizedAsciiString::<8>::new(symbol).unwrap(),
        decimals,
    };

    let address = Address::from(wallet.address());
    token_abi_calls::initialize(&instance, config, mint_amount, address).await;
    token_abi_calls::mint(&instance).await;

    instance
}
