use crate::utils::local_tests_utils::*;
use fuels::tx::{Address, ContractId};

#[tokio::test]
async fn test() {
    let wallet = init_wallet().await;
    let address = Address::from(wallet.address());

    let usdc_config = DeployTokenConfig {
        name: String::from("USD Coin"),
        symbol: String::from("USDC"),
        decimals: 6,
        mint_amount: 10000,
    };
    let usdc_instance = get_token_contract_instance(&wallet, &usdc_config).await;
    let contract_id = ContractId::from(usdc_instance.get_contract_id());
    println!("Address {}", address);
    println!("ContractId {}", contract_id);
}
// #[tokio::test]
// async fn pause() {
//     //--------------- WALLET ---------------
//     let wallet = init_wallet().await;
//     let address = Address::from(wallet.address());
//     println!("Wallet address {address}\n");

//     //--------------- ORACLE ---------------
//     let oracle_instance = get_oracle_contract_instance(&wallet).await;

//     //--------------- TOKENS ---------------
//     let eth_config = DeployTokenConfig {
//         name: String::from("Etherium"),
//         symbol: String::from("ETH"),
//         decimals: 9,
//         mint_amount: 1,
//     };

//     let usdc_config = DeployTokenConfig {
//         name: String::from("USD Coin"),
//         symbol: String::from("USDC"),
//         decimals: 6,
//         mint_amount: 10000,
//     };
//     let usdc_instance = get_token_contract_instance(&wallet, &usdc_config).await;

//     let link_config = DeployTokenConfig {
//         name: String::from("Chainlink"),
//         symbol: String::from("LINK"),
//         decimals: 9,
//         mint_amount: 1000,
//     };
//     let link_instance = get_token_contract_instance(&wallet, &link_config).await;

//     let btc_config = DeployTokenConfig {
//         name: String::from("Bitcoin"),
//         symbol: String::from("BTC"),
//         decimals: 8,
//         mint_amount: 1,
//     };
//     let btc_instance = get_token_contract_instance(&wallet, &btc_config).await;

//     let uni_config = DeployTokenConfig {
//         name: String::from("Uniswap"),
//         symbol: String::from("UNI"),
//         decimals: 9,
//         mint_amount: 1000,
//     };
//     let uni_instance = get_token_contract_instance(&wallet, &uni_config).await;

//     //--------------- MARKET ---------------
//     let market_instance = deploy_market_contract(&wallet).await;

//     let market_config = MarketConfiguration {
//         governor: address,
//         pause_guardian: address,
//         base_token: ContractId::from(usdc_instance.get_contract_id()),
//         base_token_decimals: usdc_config.decimals,
//         base_token_price_feed: ContractId::from(oracle_instance.get_contract_id()),
//         kink: 800000000000000000,                                       // decimals: 18
//         supply_per_second_interest_rate_slope_low: 250000000000000000,  // decimals: 18
//         supply_per_second_interest_rate_slope_high: 750000000000000000, // decimals: 18
//         borrow_per_second_interest_rate_slope_low: 300000000000000000,  // decimals: 18
//         borrow_per_second_interest_rate_slope_high: 800000000000000000, // decimals: 18
//         borrow_per_second_interest_rate_base: 15854895992,              // decimals: 18
//         store_front_price_factor: 6000,                                 // decimals: 4
//         base_tracking_supply_speed: 1868287030000000,                   // decimals 18
//         base_tracking_borrow_speed: 3736574060000000,                   // decimals 18
//         base_min_for_rewards: 20000000, // decimals base_token_decimals
//         base_borrow_min: 10000000,      // decimals: base_token_decimals
//         target_reserves: 1000000000000, // decimals: base_token_decimals
//         asset_configs: vec![
//             AssetConfig {
//                 asset: ContractId::from(link_instance.get_contract_id()),
//                 price_feed: ContractId::from(oracle_instance.get_contract_id()),
//                 decimals: link_config.decimals,
//                 borrow_collateral_factor: 7900,    // decimals: 4
//                 liquidate_collateral_factor: 8500, // decimals: 4
//                 liquidation_penalty: 700,          // decimals: 4
//                 supply_cap: 200000000000000,       // decimals: asset decimals
//             },
//             AssetConfig {
//                 asset: ContractId::from(uni_instance.get_contract_id()),
//                 price_feed: ContractId::from(oracle_instance.get_contract_id()),
//                 decimals: uni_config.decimals,
//                 borrow_collateral_factor: 7500,    // decimals: 4
//                 liquidate_collateral_factor: 8100, // decimals: 4
//                 liquidation_penalty: 700,          // decimals: 4
//                 supply_cap: 200000000000000,       // decimals: asset decimals
//             },
//             AssetConfig {
//                 asset: ContractId::from(btc_instance.get_contract_id()),
//                 price_feed: ContractId::from(oracle_instance.get_contract_id()),
//                 decimals: btc_config.decimals,
//                 borrow_collateral_factor: 7000,    // decimals: 4
//                 liquidate_collateral_factor: 7700, // decimals: 4
//                 liquidation_penalty: 500,          // decimals: 4
//                 supply_cap: 1000000000000,         // decimals: asset decimals
//             },
//             AssetConfig {
//                 asset: ContractId::from_str(BASE_ASSET_ID.to_string().as_str())
//                     .expect("Cannot parse BASE_ASSET_ID to contract id"),
//                 price_feed: ContractId::from(oracle_instance.get_contract_id()),
//                 decimals: eth_config.decimals,
//                 borrow_collateral_factor: 8300,    // decimals: 4
//                 liquidate_collateral_factor: 9000, // decimals: 4
//                 liquidation_penalty: 500,          // decimals: 4
//                 supply_cap: 100000000000000,       // decimals: asset decimals
//             },
//         ],
//     };

//     market_abi_calls::initialize(&market_instance, market_config)
//         .await
//         .expect("❌ Cannot initialize market");

//     market_abi_calls::supply(&market_instance)
//         .await
//         .expect("❌ Cannot supply");

//     let pause_config = PauseConfiguration {
//         supply_paused: true,
//     };
//     market_abi_calls::pause(&market_instance, pause_config)
//         .await
//         .expect("❌ Cannot pause market");

//     assert_eq!(
//         market_abi_calls::supply(&market_instance).await.is_err(),
//         true
//     );

//     let pause_config = PauseConfiguration {
//         supply_paused: false,
//     };
//     market_abi_calls::pause(&market_instance, pause_config)
//         .await
//         .expect("❌ Cannot pause market");

//     assert_eq!(
//         market_abi_calls::supply(&market_instance).await.is_err(),
//         false
//     );
// }
