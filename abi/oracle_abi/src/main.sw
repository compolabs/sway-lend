library ;

abi Oracle {
    #[storage(read)]
    fn owner() -> Identity;
    #[storage(read, write)]
    fn set_price(asset_id: AssetId, price_value: u64);
    #[storage(read, write)]
    fn set_prices(prices: Vec<(AssetId, u64)>);
    #[storage(read)]
    fn get_price(asset_id: AssetId) -> Price;
}

pub struct Price {
    asset_id: AssetId,
    price: u64,
    last_update: u64,
}