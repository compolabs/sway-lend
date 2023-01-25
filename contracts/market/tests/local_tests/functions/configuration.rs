use crate::utils::local_tests_utils::market;

#[tokio::test]
async fn get_configuration() {
    let (_wallets, assets, market, _oracle) = market::setup_market().await;
    let methods = market.methods();
    let is_err = methods.get_configuration().simulate().await.is_err();
    assert!(!is_err);

    let is_err = methods
        .get_asset_config_by_asset_id(assets.get("UNI").unwrap().contract_id)
        .simulate()
        .await
        .is_err();
    assert!(!is_err);

    let is_err = methods
        .get_asset_config_by_asset_id(assets.get("BTC").unwrap().contract_id)
        .simulate()
        .await
        .is_err();
    assert!(!is_err);

    let is_err = methods
        .get_asset_config_by_asset_id(assets.get("SWAY").unwrap().contract_id)
        .simulate()
        .await
        .is_err();
    assert!(!is_err);

    let is_err = methods
        .get_asset_config_by_asset_id(assets.get("LINK").unwrap().contract_id)
        .simulate()
        .await
        .is_err();
    assert!(!is_err);

    let is_err = methods
        .get_asset_config_by_asset_id(assets.get("ETH").unwrap().contract_id)
        .simulate()
        .await
        .is_err();
    assert!(!is_err);
}

