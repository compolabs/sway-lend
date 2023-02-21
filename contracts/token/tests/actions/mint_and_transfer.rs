use std::str::FromStr;

use dotenv::dotenv;
use fuels::{
    prelude::{abigen, Bech32Address, Bech32ContractId,  Provider, TxParameters},
    signers::{Wallet, WalletUnlocked},
    types::{Address, ContractId},
};

use crate::utils::parse_units;

abigen!(Contract(
    name = "TokenContract",
    abi = "out/debug/token_contract-abi.json"
));

const RPC: &str = "node-beta-2.fuel.network";
const AMOUNT: u64 = 10_000;
const TOKEN_ADDRESS: &str = "0xd7d5e5c1220872e6f42b38f85ae80c6072b1b4723e7a7218bbf6717aca962536";
const RECIPIEND_ADDRES: &str = "fuel1mdd5auwxa49xzc4p48rrlkuge8ah9pn76mdjtn6chmxyctykgj7ssre6fv";

#[tokio::test]
async fn transfer() {
    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    dotenv().ok();
    let secret = match std::env::var("SECRET") {
        Ok(s) => s,
        Err(error) => panic!("❌ Cannot find .env file: {:#?}", error),
    };

    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let recipient = Bech32Address::from_str(RECIPIEND_ADDRES).unwrap();
    let recipient = Wallet::from_address(recipient, Some(provider.clone()));

    let token_id = Bech32ContractId::from(ContractId::from_str(TOKEN_ADDRESS).unwrap());

    let instance = TokenContract::new(token_id, wallet.clone());

    let config = instance.methods().config().simulate().await.unwrap().value;

    let amount = parse_units(AMOUNT, config.decimals);
    instance
        .methods()
        .mint_and_transfer(amount, Address::from(recipient.address()))
        .tx_params(TxParameters::new(Some(1), Some(1000000), None))
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

}
