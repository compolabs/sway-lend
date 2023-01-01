use crate::utils::local_tests_utils::market;

#[tokio::test]
async fn initialize() {
    let (_admin, _assets, _market, _oracle) = market::setup_market().await;
}
