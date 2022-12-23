use fuels::tx::{Address, ContractId};

use crate::utils::local_tests_utils::{marketcontract_mod::AssetConfig, *};

#[tokio::test]
async fn initialize() {
    //--------------- WALLET ---------------
    let wallet = init_wallet().await;
    let address = Address::from(wallet.address());
    println!("Wallet address {address}\n");

    //--------------- TOKENS ---------------

    let bnb_config = DeployTokenConfig {
        name: String::from("BNB"),
        symbol: String::from("BNB"),
        decimals: 8,
        mint_amount: 5,
    };

    let bnb_instance = get_token_contract_instance(&wallet, &bnb_config).await;

    //--------------- MARKET ---------------
    let market_instance = get_market_contract_instance(&wallet).await;
    let marketConfig = marketcontract_mod::MarketConfiguration {
        governor: Address::from(wallet.address()),
        base_token: ContractId::from(bnb_instance.get_contract_id()),
        asset_configs: vec![
            AssetConfig {
                asset: ContractId::from(bnb_instance.get_contract_id()),
                price_feed: ContractId::from(bnb_instance.get_contract_id()),
                decimals: bnb_config.decimals,
                borrow_collateral_factor: 1,
                liquidate_collateral_factor: 1,
                liquidation_factor: 1,
                supply_cap: 1,
            },
            AssetConfig {
                asset: ContractId::from(bnb_instance.get_contract_id()),
                price_feed: ContractId::from(bnb_instance.get_contract_id()),
                decimals: bnb_config.decimals,
                borrow_collateral_factor: 1,
                liquidate_collateral_factor: 1,
                liquidation_factor: 1,
                supply_cap: 1,
            },
        ],
    };
    market_abi_calls::initialize(&market_instance, marketConfig)
        .await
        .expect("❌ Cannot initialize market");

    // let usdt_methods = usdt_instance.methods();
    // let usdt_asset_id = AssetId::from(*usdt_instance.get_contract_id().hash());
    // let usdt_symbol = usdt_methods.symbol().simulate().await.unwrap().value;
    // let usdt_decimals = usdt_methods.decimals().simulate().await.unwrap().value;

    // let usdc_methods = usdc_instance.methods();
    // let usdc_asset_id = AssetId::from(*usdc_instance.get_contract_id().hash());
    // let usdc_symbol = usdc_methods.symbol().simulate().await.unwrap().value;
    // let usdc_decimals = usdc_methods.decimals().simulate().await.unwrap().value;

    // println!("Asset1\n id: {usdt_asset_id}\n symbol {usdt_symbol}\n decimals {usdt_decimals}\n");
    // println!("Asset2\n id: {usdc_asset_id}\n symbol {usdc_symbol}\n decimals {usdt_decimals}\n");

    // print_balances(&wallet).await;

    // //--------------- LIMIT ORDERS ---------
    // let limit_orders_instance = get_limit_orders_contract_instance(&wallet).await;
    // let dapp_methods = limit_orders_instance.methods();

    // let _res = dapp_methods
    //     .create_order(
    //         ContractId::from(usdc_instance.get_contract_id()),
    //         parse_units(10, usdc_decimals),
    //     )
    //     .call_params(CallParameters::new(
    //         Some(parse_units(10, usdt_decimals)),
    //         Some(usdt_asset_id),
    //         None,
    //     ))
    //     .append_variable_outputs(1)
    //     .call()
    //     .await;
    // println!("\n{} Create Order", if _res.is_ok() { "✅" } else { "❌" });

    // let balance = wallet.get_asset_balance(&usdt_asset_id).await.unwrap();
    // assert!(balance == parse_units(10000 - 10, usdt_decimals));

    // let _res = dapp_methods
    //     .fulfill_order(1)
    //     .call_params(CallParameters::new(
    //         Some(parse_units(10, usdc_decimals)),
    //         Some(usdc_asset_id),
    //         None,
    //     ))
    //     .estimate_tx_dependencies(Option::None)
    //     .await
    //     .unwrap()
    //     .call()
    //     .await;
    // println!("\n{} Fulfill Order", if _res.is_ok() { "✅" } else { "❌" });

    // let balance = wallet.get_asset_balance(&usdt_asset_id).await.unwrap();
    // assert!(balance == parse_units(10000, usdt_decimals));

    // let balance = wallet.get_asset_balance(&usdc_asset_id).await.unwrap();
    // assert!(balance == parse_units(10000, usdc_decimals));

    // let order = dapp_methods.order_by_id(1).simulate().await.unwrap().value;
    // assert!(order.status == Status::Completed());
}
