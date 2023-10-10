library ;

abi Oracle {
    fn owner() -> Identity;
    #[storage(read, write)]
    fn set_price(asset_id: b256, price_value: u64);
    #[storage(read, write)]
    fn set_prices(prices: Vec<(b256, u64)>);
    #[storage(read)]
    fn get_price(asset_id: b256) -> Price;
}
struct Price {
    asset_id: b256,
    price: u64,
    last_update: u64,
}