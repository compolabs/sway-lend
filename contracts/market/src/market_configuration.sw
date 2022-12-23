library market_configuration;

pub struct MarketConfiguration {
    governor: Address, // admin
    pause_guardian: Address, //admin, who can emergency stop the protocol
    base_token: ContractId, // borrow token
    base_token_price_feed: Address, // borrow token price oracle address
    // extensionDelegate: Address,
    supply_kink: u64,
    supply_per_second_interest_rate_slope_low: u64, //оптимальная утилизация
    supply_per_second_interest_rate_slope_high: u64, //коэффициент зависимости ежесекундной ставки от утилизации, если утилизация ниже оптимальной. может стоит поменять на ежеминутную
    supply_per_second_interest_rate_base: u64, //коэффициент зависимости ежесекундной ставки от утилизации, если утилизация выше оптимальной
    borrow_kink: u64,
    borrow_per_second_interest_rate_slope_low: u64,
    borrow_per_second_interest_rate_slope_high: u64,
    borrow_per_second_interest_rate_base: u64, //ежесекундная ставка при утилизации стремящейся к нулю
    store_front_price_factor: u64, //доля ликвидейшн пеналти которую получает ликвидатор
    // trackingIndexScale: u64,
    // baseTrackingSupplySpeed: u64,
    // baseTrackingBorrowSpeed: u64,
    // baseMinForRewards: u64, //TODO: change type to u128
    base_borrow_min: u64, // минимальная сумма займа TODO: change type to u128
    target_reserves: u64, //максимальное количество резервов при которых происходят ликвидации //TODO: change type to u128
    asset_configs: Vec<AssetConfig>,
}

pub struct AssetConfig {
    asset: ContractId,
    price_feed: ContractId,
    decimals: u8,
    borrow_collateral_factor: u64,
    liquidate_collateral_factor: u64,
    liquidation_factor: u64,
    supply_cap: u64, //TODO: change type to u128
}
