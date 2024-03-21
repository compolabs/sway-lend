use crate::INDEXER_URL;

#[derive(serde::Deserialize)]
pub struct IndexerResponce<T> {
    pub data: Vec<Vec<T>>,
}

#[derive(serde::Deserialize)]
pub struct CollateralConfig {
    pub asset_id: String,
    pub borrow_collateral_factor: u64,
    pub decimals: u64,
    pub id: String,
    pub liquidate_collateral_factor: u64,
    pub liquidation_penalty: u64,
    pub paused: bool,
    pub price_feed: String,
    pub supply_cap: u64,
}
#[derive(serde::Deserialize)]
pub struct UserCollateral {
    pub id: String,
    pub address: String,
    pub amount: u64,
    pub asset_id: String,
}
#[derive(serde::Deserialize, Clone)]
pub struct UserBasics {
    pub id: String,
    pub address: String,
    pub base_tracking_accrued: u64,
    pub base_tracking_index: u64,
    pub principal_absolute: u64,
    pub principal_negative: bool,
    pub reward_claimed: u64,
}

pub async fn fetch_collateral_configurations() -> IndexerResponce<CollateralConfig> {
    let reqwest_client = reqwest::Client::new();
    let responce = reqwest_client
    .post(INDEXER_URL)
    .header("Content-Type" , "application/json")
    .body("{\"query\":\"SELECT json_agg(t) FROM (SELECT * FROM composabilitylabs_swaylend_indexer.collateralconfigurationentity) t;\"}")
.send()
.await
.unwrap();
    let collateral_configs: IndexerResponce<CollateralConfig> =
        serde_json::from_str(&responce.text().await.unwrap()).unwrap();
    collateral_configs
}

pub async fn fetch_user_basics() -> IndexerResponce<UserBasics> {
    let reqwest_client = reqwest::Client::new();
    let responce = reqwest_client
        .post(INDEXER_URL)
        .header("Content-Type" , "application/json")
        .body("{\"query\":\"SELECT json_agg(t) FROM (SELECT * FROM composabilitylabs_swaylend_indexer.userbasicentity WHERE principal_negative=TRUE) t;\"}")
        .send()
        .await
        .unwrap();
    let json = responce.text().await.unwrap();

    let user_basics: IndexerResponce<UserBasics> = serde_json::from_str(&json).unwrap();
    user_basics
}
