use std::str::FromStr;

use fuels::{
    prelude::{Bech32ContractId, TxParameters},
    tx::{Address, ContractId},
};

use crate::utils::{
    local_tests_utils::OracleContract, number_utils::format_units,
    testnet_tests_utils::setup_wallet,
};

const ORACLE_ADDRESS: &str = "0xde764394c83bb3c8a3aec5f75f383ff86e64728964fab4469df5910ca01b1a59";
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
            symbol: "SWAY",
            default_price: 50 * 10u64.pow(9),
            asset_id: "0x99075448d291a8f8f69e5f3d25a309c38ad38def9f709a69ae4a2aeaed1701fe",
            coingeco_id: "compound-governance-token",
            // decimals: 9,
        },
        AssetConfig {
            symbol: "BTC",
            default_price: 19000 * 10u64.pow(9),
            asset_id: "0xdd17dda6eeee55f6d327020e6d61b9fa7b3c2ab205c46cdca690a46966f4e1c7",
            coingeco_id: "bitcoin",
            // decimals: 8,
        },
        AssetConfig {
            symbol: "USDC",
            default_price: 1 * 10u64.pow(9),
            asset_id: "0xd7d5e5c1220872e6f42b38f85ae80c6072b1b4723e7a7218bbf6717aca962536",
            coingeco_id: "usd-coin",
            // decimals: 6,
        },
        AssetConfig {
            symbol: "UNI",
            default_price: 5 * 10u64.pow(9),
            asset_id: "0x76c4fda9074c4509eaf2652f82bace86e2c7a21bf9faff7bf6228034ebc0f8a2",
            coingeco_id: "uniswap",
            // decimals: 9,
        },
        AssetConfig {
            symbol: "LINK",
            default_price: 5 * 10u64.pow(9),
            asset_id: "0x71be783354a9bccfa9de0e7edf291797775e4a730d0922a9675258dbb47f557b",
            coingeco_id: "chainlink",
            // decimals: 9,
        },
        AssetConfig {
            symbol: "ETH",
            default_price: 1200 * 10u64.pow(9),
            asset_id: "0x0000000000000000000000000000000000000000000000000000000000000000",
            coingeco_id: "ethereum",
            // decimals: 9,
        },
    ];

    let (wallet, _provider) = setup_wallet().await;
    let tx_params = TxParameters::new(Some(1), Some(1000000), None);
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

    for asset in assets {
        let contract_id = ContractId::from_str(asset.asset_id)
            .expect("failed to create ContractId address from string");
        let bech32_address = Bech32ContractId::from(contract_id);

        let asset_id = ContractId::from(bech32_address);
        let symbol = asset.symbol;
        let last_price = methods.get_price(asset_id).simulate().await.unwrap();

        let price = match responce[asset.coingeco_id]["usd"].as_f64() {
            Some(p) => (p * 10f64.powf(9f64)).round() as u64,
            _ => asset.default_price,
        };

        let _res = methods
            .set_price(asset_id, price)
            .tx_params(tx_params)
            .call()
            .await;
        let new_price = methods.get_price(asset_id).simulate().await.unwrap();
        println!("{} Set price", if _res.is_ok() { "✅" } else { "❌" },);
        println!(
            "{symbol} price was changed {} {symbol} ({}) -> {} {symbol} ({})",
            format_units(last_price.value.price, 9),
            last_price.value.price,
            format_units(new_price.value.price, 9),
            new_price.value.price
        );
    }
}
