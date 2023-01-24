use fuels_types::Address;

use crate::utils::local_tests_utils::market::{self, get_market_config, market_abi_calls};

#[tokio::test]
async fn initialize() {
    let (wallets, assets, market, _oracle) = market::setup_market().await;
    let address = Address::from(wallets[0].address());
    let contract_id = assets.get("USDC").unwrap().contract_id;
    let conf = get_market_config(address, address, contract_id, 9, contract_id, contract_id);
    assert!(
        market_abi_calls::initialize(&market, &conf, &vec![], Option::Some(1u64))
            .await
            .is_err()
    );
}
