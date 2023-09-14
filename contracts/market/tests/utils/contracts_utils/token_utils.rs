use fuels::accounts::wallet::WalletUnlocked;
use std::collections::HashMap;

use fuels::types::{AssetId, Bits256, ContractId};
use serde::Deserialize;
use src20_sdk::{deploy_token_factory_contract, token_factory_abi_calls, TokenFactoryContract};

use super::market_utils::CollateralConfiguration;

pub struct Asset {
    pub bits256: Bits256,
    pub asset_id: AssetId,
    pub default_price: u64,
    pub decimals: u64,
    pub symbol: String,
    pub coingeco_id: String,
}

#[derive(Deserialize)]
pub struct TokenConfig {
    pub asset_id: String,
    pub name: String,
    pub symbol: String,
    pub coingeco_id: String,
    pub default_price: u64,
    pub decimals: u64,
    pub borrow_collateral_factor: Option<u64>,
    pub liquidate_collateral_factor: Option<u64>,
    pub liquidation_penalty: Option<u64>,
    pub supply_cap: Option<u64>,
}

pub async fn deploy_tokens(
    admin: &WalletUnlocked,
    price_feed: ContractId,
) -> (
    HashMap<String, Asset>,
    Vec<CollateralConfiguration>,
    TokenFactoryContract<WalletUnlocked>,
) {
    let bin_path = "tests/artefacts/factory/token-factory.bin";
    let factory = deploy_token_factory_contract(admin, &bin_path).await;

    let tokens_json = std::fs::read_to_string("tests/artefacts/tokens.json").unwrap();
    let token_configs: Vec<TokenConfig> = serde_json::from_str(&tokens_json).unwrap();

    let mut assets: HashMap<String, Asset> = HashMap::new();
    let mut asset_configs: Vec<CollateralConfiguration> = vec![];

    for config in token_configs {
        let name = config.name;
        let symbol = config.symbol;
        let decimals = config.decimals;

        token_factory_abi_calls::deploy(&factory, &symbol, &name, decimals)
            .await
            .unwrap();

        let bits256 = token_factory_abi_calls::asset_id(&factory, &symbol)
            .await
            .unwrap()
            .value;

        if symbol != "USDC" {
            asset_configs.push(CollateralConfiguration {
                asset_id: bits256,
                decimals: config.decimals,
                price_feed,
                borrow_collateral_factor: config.borrow_collateral_factor.unwrap(), // decimals: 4
                liquidate_collateral_factor: config.liquidate_collateral_factor.unwrap(), // decimals: 4
                liquidation_penalty: config.liquidation_penalty.unwrap(), // decimals: 4
                supply_cap: config.supply_cap.unwrap(), // decimals: asset decimals
                paused: false,
            });
        }

        assets.insert(
            symbol.clone(),
            Asset {
                bits256,
                asset_id: AssetId::from(bits256.0),
                default_price: config.default_price,
                decimals: config.decimals,
                symbol,
                coingeco_id: config.coingeco_id,
            },
        );
    }
    (assets, asset_configs, factory)
}

pub async fn _load_tokens(tokens_json_path: &str) -> HashMap<String, Asset> {
    let tokens_json = std::fs::read_to_string(tokens_json_path).unwrap();
    let token_configs: Vec<TokenConfig> = serde_json::from_str(&tokens_json).unwrap();

    let mut assets: HashMap<String, Asset> = HashMap::new();

    for config in token_configs {
        let bits256 = Bits256::from_hex_str(&config.asset_id).unwrap();
        assets.insert(
            config.symbol.clone(),
            Asset {
                bits256,
                asset_id: AssetId::from(bits256.0),
                default_price: config.default_price,
                decimals: config.decimals,
                symbol: config.symbol.clone(),
                coingeco_id: config.coingeco_id,
            },
        );
    }
    assets
}
