use std::str::FromStr;

use fuels::{
    prelude::{Bech32ContractId, TxParameters},
    tx::{Address, ContractId},
};

use crate::utils::{
    local_tests_utils::OracleContract, number_utils::format_units,
    testnet_tests_utils::setup_wallet,
};

const ORACLE_ADDRESS: &str = "0xcff9283e360854a2f4523c6e5a569a9032a222b8ea6d91cdd1506f0375e5afb5";
#[derive(Debug)]
struct AssetConfig<'a> {
    symbol: &'a str,
    asset_id: &'a str,
    default_price: u64,
    coingeco_id: &'a str,
}

#[tokio::test]
async fn main_test() {
    let assets = vec![
        AssetConfig {
            symbol: "COMP",
            default_price: 50 * 10u64.pow(9),
            asset_id: "0x6c0a715375b510e1ef562bb5b3a7afb2c9b4a7380251e3f295e3225410b96488",
            coingeco_id: "compound-governance-token",
        },
        AssetConfig {
            symbol: "SWAY",
            default_price: 50 * 10u64.pow(9),
            asset_id: "0x89eac25d412c5c1b63d212deacc109dcff804eff70101fe0fc72167bc7884aa2",
            coingeco_id: "compound-governance-token",
        },
        AssetConfig {
            symbol: "BTC",
            default_price: 19000 * 10u64.pow(9),
            asset_id: "0xf7d6d3344dd36493d7e6b02b16a679778ad24539e2698af02558868a6f2feb81",
            coingeco_id: "bitcoin",
        },
        AssetConfig {
            symbol: "USDC",
            default_price: 1 * 10u64.pow(9),
            asset_id: "0x56fb8789a590ea9c12af6fe6dc2b43f347700b049d4f823fd4476c6f366af201",
            coingeco_id: "usd-coin",
        },
        AssetConfig {
            symbol: "UNI",
            default_price: 5 * 10u64.pow(9),
            asset_id: "0x5381bbd1cff41519062c8531ec30e8ea1a2d752e59e4ac884068d3821e9f0093",
            coingeco_id: "uniswap",
        },
        AssetConfig {
            symbol: "LINK",
            default_price: 5 * 10u64.pow(9),
            asset_id: "0x2018850b249a9c531a51d52465290d7bfc9f18838a5c4c6f476bff9553a8f7e9",
            coingeco_id: "chainlink",
        },
        AssetConfig {
            symbol: "ETH",
            default_price: 1200 * 10u64.pow(9),
            asset_id: "0x0000000000000000000000000000000000000000000000000000000000000000",
            coingeco_id: "ethereum",
        },
    ];

    let (wallet, _provider) = setup_wallet().await;
    let tx_params = TxParameters::default().set_gas_price(1);
    let oracle_dapp_id = Bech32ContractId::from(ContractId::from_str(ORACLE_ADDRESS).unwrap());
    let oracle = OracleContract::new(oracle_dapp_id, wallet.clone());
    let methods = oracle.methods();
    let _res = methods
        .initialize(Address::from(wallet.address()))
        .tx_params(tx_params)
        .call()
        .await;
    println!("{} Initialize\n", if _res.is_ok() { "✅" } else { "❌" });

    let client = reqwest::Client::new();
    let req = "https://api.coingecko.com/api/v3/simple/price?ids=compound-governance-token%2Cbinancecoin%2Cbitcoin%2Cbinance-usd%2Cusd-coin%2Ctether%2Cuniswap%2Cethereum%2Cchainlink&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false&precision=9";
    let body = client.get(req).send().await.unwrap().text().await.unwrap();
    let responce: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();

    let mut prices: Vec<(ContractId, u64)> = vec![];
    let mut message = String::from("💰 Price oracle uppdate\n");
    for asset in assets {
        let contract_id = ContractId::from_str(asset.asset_id)
            .expect("failed to create ContractId address from string");
        let bech32_address = Bech32ContractId::from(contract_id);

        let asset_id = ContractId::from(bech32_address);
        let symbol = asset.symbol;

        let price = match responce[asset.coingeco_id]["usd"].as_f64() {
            Some(p) => (p * 10f64.powf(9f64)).round() as u64,
            _ => asset.default_price,
        };
        prices.push((asset_id, price));

        message += format!("1 {symbol} = ${} ({})\n", format_units(price, 9), price).as_str();
    }
    let _res = methods
        .set_prices(prices)
        .tx_params(tx_params)
        .call()
        .await
        .unwrap();
    println!("{message}");
}
