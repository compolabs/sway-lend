use std::str::FromStr;

use dotenv::dotenv;
use fuels::{
    prelude::{AssetId, Provider, WalletUnlocked},
    types::ContractId,
};

use crate::utils::contracts_utils::{
    market_utils::{
        abigen_bindings::market_contract_mod, deploy_market, get_market_config, market_abi_calls,
    },
    token_utils::load_tokens,
};

const RPC: &str = "beta-4.fuel.network";
const ORACLE_ADDRESS: &str = "0xb19e156a8a6cc6d7fc2831c31c65f6bc10b8a4a80f42cbdbeb46c23f3851105e";

#[tokio::test]
async fn deploy() {
    //--------------- WALLET ---------------
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let oracle_id = ContractId::from_str(ORACLE_ADDRESS).unwrap();

    //--------------- Tokens ---------------
    let (assets, asset_configs) = load_tokens("tests/artefacts/tokens.json", oracle_id).await;
    let usdc = assets.get("USDC").unwrap();

    //--------------- MARKET ---------------
    let market_config = get_market_config(
        wallet.address().into(),
        wallet.address().into(),
        usdc.bits256,
        usdc.decimals,
        oracle_id,
        // assets.get("SWAY").unwrap().bits256,
    );
    let market = deploy_market(&wallet, market_config, Option::None).await;
    let sway_bits256 = market_abi_calls::get_reward_token_asset_id(&market).await;
    println!("SWAY Address = {:?}", AssetId::from(sway_bits256.0));

    //--------------- SETUP COLLATERALS ---------------
    for config in &asset_configs {
        let mut config = config.clone();
        // replace swaylend token into reward token
        if config.asset_id.value == assets.get("SWAY").unwrap().bits256 {
            config.asset_id = market_contract_mod::AssetId {
                value: sway_bits256,
            }
        }

        market_abi_calls::add_collateral_asset(&market, &config)
            .await
            .unwrap();
    }

    println!("Market contract = {}", market.contract_id().hash());
    println!("Market contract = {}", market.contract_id().to_string());
}
