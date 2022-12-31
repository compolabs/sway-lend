// SPDX-License-Identifier: BUSL-1.1
contract;

/**
 *
 * @title Swaylend's Market Contract
 * @notice An efficient monolithic money market protocol
 * @author SWAY GANG
 */
dep structs;
dep int;
dep math;

use structs::*;
use int::*;
use oracle_abi::*;

use std::{
    address::*,
    auth::{
        AuthError,
        msg_sender,
    },
    block::timestamp,
    call_frames::{
        contract_id,
        msg_asset_id,
    },
    constants::ZERO_B256,
    context::{
        balance_of,
        msg_amount,
    },
    hash::sha256,
    revert::require,
    storage::*,
    token::*,
};
use sway_libs::i128::I128;

abi Market {
    #[storage(write)]
    fn initialize(config: MarketConfiguration);

    #[storage(write, read)]
    fn pause(config: PauseConfiguration);

    #[storage(read)]
    fn get_configuration() -> MarketConfiguration;

    #[storage(read)]
    fn get_oracle_price(asset: ContractId) -> u64;
    //-------------------------------------------------
    #[storage(read)]
    fn get_utilization() -> u64;

    #[storage(read)]
    fn get_supply_rate(utilization: u64) -> u64;

    #[storage(read)]
    fn get_borrow_rate(utilization: u64) -> u64;
   
    #[storage(read)]
    fn is_liquidatable(account: Address) -> bool;
    
    #[storage(read)]
    fn get_collateral_reserves(asset: ContractId) -> u64;

    #[storage(read)]
    fn get_reserves() -> I128;
   
    #[storage(read)]
    fn withdraw_reserves(to: Address, amount: u64); 
   
    #[storage(read)]
    fn quote_collateral(asset: ContractId, base_amount: u64) -> u64;

    #[storage(read, write)]
    fn absorb(absorber: Address, accounts: Vec<Address>);

    #[storage(read)]
    fn buy_collateral(asset: ContractId, min_amount: u64, base_amount: u64, recipient: Address);

    #[storage(read, write)]
    fn supply_collateral(from: Address, dst: Address, asset: ContractId, amount: u64);

    #[storage(read, write)]
    fn withdraw_collateral(asset: ContractId, amount: u64);

    #[storage(read, write)]
    fn supply_base();

    #[storage(read, write)]
    fn withdraw_base(amount: u64);
    
    #[storage(read)]
    fn withdraw_reward_token(to: Address, amount: u64);

    #[storage(read, write)]
    fn get_reward_owed(account: Address) -> u64;

    #[storage(read, write)]
    fn claim();
}

storage {
    config: Option<MarketConfiguration> = Option::None,
    pause_config: Option<PauseConfiguration> = Option::None,
    totals_collateral: StorageMap<ContractId, u64> = StorageMap {},
    user_collateral: StorageMap<(Address, ContractId), u64> = StorageMap {}, 
    user_basic: StorageMap<Address, UserBasic> = StorageMap {},
    market_basic: MarketBasics = MarketBasics {
        base_supply_index: 0,
        base_borrow_index: 0,
        tracking_supply_index: 0,
        tracking_borrow_index: 0,
        total_supply_base: 0,
        total_borrow_base: 0,
        last_accrual_time: 0,
    },
}

//=======================================================
const BASE_INDEX_SCALE: u64 = 1000000000000000000; //1e18
const FACTOR_SCALE: u64 = 1000000000000000000; // 1e18
//=======================================================

#[storage(read)]
fn is_absorb_paused() -> bool {
    match storage.pause_config {
        Option::Some(config) => config.absorb_paused,
        Option::None(_) => false,
    }
}

#[storage(read)]
fn is_buy_paused() -> bool {
    match storage.pause_config {
        Option::Some(config) => config.buy_pause,
        Option::None(_) => false,
    }
}
#[storage(read)]
fn is_supply_paused() -> bool {
    match storage.pause_config {
        Option::Some(config) => config.supply_paused,
        Option::None(_) => false,
    }
}
#[storage(read)]
fn is_withdraw_paused() -> bool {
    match storage.pause_config {
        Option::Some(config) => config.withdraw_paused,
        Option::None(_) => false,
    }
}

#[storage(read)]
fn is_claim_paused() -> bool {
    match storage.pause_config {
        Option::Some(config) => config.claim_paused,
        Option::None(_) => false,
    }
}

fn is_liquidatable() -> bool{
    //TODO: inplement
    true
}

fn mint_reward_token(_amount: u64){
    //TODO: inplement
}

#[storage(read)]
fn get_config() -> MarketConfiguration {
    match (storage.config) {
        Option::Some(c) => c,
        _ => revert(0),
    }
}

pub fn get_caller() -> Address {
    let sender: Result<Identity, AuthError> = msg_sender();
    if let Identity::Address(address) = sender.unwrap() {
        address
    } else {
        revert(0);
    }
}

//FIXME: after compiler internal error will be fixed
fn get_price(asset: ContractId, _price_feed: ContractId) -> u64 {
    let price = match (asset) {
        ContractId {
            value: 0x6cd466e67547102656267a5f6005113e48d1f53a6846e6819c841a7f3eadafe9,
        } => 250,
        ContractId {
            value: 0x851ec5e04fa3485ba0794b34030bbcd70e96be282cd429da03c58e8de4d46c00,
        } => 19000,
        ContractId {
            value: 0xfcdcc57a0c59be38eecab975ddd03c3cd2cb1852957b622d5613d60ec8f4f2c2,
        } => 1,
        ContractId {
            value: 0xe09c4c702e6a8237dd07f29228c136cc076b79cb9d0e1f891d39c54dc95069ac,
        } => 1,
        ContractId {
            value: 0x7d4b2c57d0c8715be35224b29357ba2444e40f6cd1d9227a96e8d8f4a8f44ba4,
        } => 1,
        ContractId {
            value: 0xcc28b139c7664ac9cddc2c01c00559fbbebd6fa8a879db341adf3a4aafdaa137,
        } => 5,
        ContractId {
            value: 0x579cd9e73d2471fd0ce20156e06e34c09cdf2fd655c993af8d236185305461ee,
        } => 5,
        ContractId {
            value: 0x0000000000000000000000000000000000000000000000000000000000000000,
        } => 1200,
        _ => revert(0),
    };
    price * 1000000000
}
//#[storage(read)]
// fn get_price(asset: ContractId, price_feed: ContractId) -> u64 {
//     let caller = abi(Oracle, price_feed.value);
//     caller.get_price(asset).price
// }
// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
// ----------------------------------------------------------------------

 // ФУНКЦИИ ВСЕГДА ВОЗВРАЩАЮТ ПОЛОЖИТЕЛЬНЫЕ ЧИСЛА
 // тут у return всегда в base_asset_decimal децимале
pub fn present_value_supply(base_supply_index_: u64, principal_value_: u64) -> u64 { // base_asset_decimals
    principal_value_ * base_supply_index_ / BASE_INDEX_SCALE
}

pub fn present_value_borrow(base_borrow_index_: u64, principal_value_: u64) -> u64 { // base_asset_decimals
    principal_value_ * base_borrow_index_ / BASE_INDEX_SCALE
}

pub fn principal_value_supply(base_supply_index_: u64, present_value_: u64) -> u64 { // base_asset_decimals
    present_value_ * BASE_INDEX_SCALE / base_supply_index_
}

pub fn principal_value_borrow(base_borrow_index_: u64, present_value_: u64) -> u64 { // base_asset_decimals
    (present_value_ * BASE_INDEX_SCALE + base_borrow_index_ - 1) / base_borrow_index_
}

 #[storage(read)]
fn present_value(principal_value_: I128) -> I128 { // base_asset_decimals
    let base_supply_index = storage.market_basic.base_supply_index; // decimals 18
    let base_borrow_index = storage.market_basic.base_borrow_index; // decimals 18
  
    if principal_value_ >= I128::zero() {   // principal_value_ >= 0
        let present_value_u64 = present_value_supply(base_supply_index, principal_value_.as_u64());
        I128::from_u64(present_value_u64)
    } else {
        let present_value_u64 = present_value_borrow(base_borrow_index, principal_value_.flip().as_u64());
        I128::from_u64(present_value_u64).flip()
    }
}

#[storage(read)]
fn principal_value(present_value_: I128) -> I128 { // base_asset_decimals
    let base_supply_index = storage.market_basic.base_supply_index; // decimals 18
    let base_borrow_index = storage.market_basic.base_borrow_index; // decimals 18
    if present_value_ >= I128::zero() {   // present_value_ >= 0
        let principal_value_u64 = principal_value_supply(base_supply_index, present_value_.as_u64());
        I128::from_u64(principal_value_u64)
    } else {
        let principal_value_u64 = principal_value_borrow(base_borrow_index, present_value_.flip().as_u64());
        I128::from_u64(principal_value_u64).flip()
    }
}

// @Callable get_utilization() -> u64
#[storage(read)]
fn get_utilization_internal() -> u64 { // 18
    let market_basic = storage.market_basic;
    let total_supply_ = present_value_supply(market_basic.base_supply_index, market_basic.total_supply_base);
    let total_borrow_ = present_value_borrow(market_basic.base_borrow_index, market_basic.total_borrow_base);
    if total_supply_ == 0 {
        0
    } else {
        total_borrow_ * FACTOR_SCALE / total_supply_
    }
}

// @Callable get_supply_rate(utilization: u64) -> u64
#[storage(read)]
fn get_supply_rate_internal(utilization: u64) -> u64 { //18
    let config = get_config();
    let kink_ = config.kink; //decimals 18
    let interest_rate_base = 0; //decimals 18, 0 потому что в конфиге нет supply_per_second_interest_rate_base
    let interest_rate_slope_low = config.supply_per_second_interest_rate_slope_low;// decimals 18
    let interest_rate_slope_high = config.supply_per_second_interest_rate_slope_high;// decimals 18
    if (utilization <= kink_) {
        interest_rate_base + interest_rate_slope_low * utilization / FACTOR_SCALE
    } else {
        interest_rate_base + (interest_rate_slope_low * kink_ + interest_rate_slope_high * (utilization - kink_)) / FACTOR_SCALE
    }
}

// @Callable get_borrow_rate(utilization: u64) -> u64
#[storage(read)]
fn get_borrow_rate_internal(utilization: u64) -> u64 { //18
    let config = get_config();
    let kink_ = config.kink; //decimals 18
    let interest_rate_base = config.borrow_per_second_interest_rate_base; //decimals 18
    let interest_rate_slope_low = config.borrow_per_second_interest_rate_slope_low; //decimals 18
    let interest_rate_slope_high = config.borrow_per_second_interest_rate_slope_high; //decimals 18
    if (utilization <= kink_) {
        interest_rate_base + interest_rate_slope_low * utilization / FACTOR_SCALE
    } else {
        interest_rate_base + (interest_rate_slope_low * kink_ + interest_rate_slope_high * (utilization - kink_)) / FACTOR_SCALE
    }
}

// подсчет обновленного значения base_supply/borrow_index
#[storage(read)]
fn accrued_interest_indices(time_elapsed: u64) -> (u64, u64) { // (18, 18)
    let mut base_supply_index_ = storage.market_basic.base_supply_index; // decimals 18
    let mut base_borrow_index_ = storage.market_basic.base_borrow_index; // decimals 18
    if (time_elapsed > 0) {
        let utilization = get_utilization_internal();  // decimals 18
        let supply_rate = get_supply_rate_internal(utilization); // decimals 18
        let borrow_rate = get_borrow_rate_internal(utilization); // decimals 18
        base_supply_index_ += base_supply_index_ * supply_rate * time_elapsed / FACTOR_SCALE;
        base_borrow_index_ += base_supply_index_ * supply_rate * time_elapsed / FACTOR_SCALE;
    }
    return (base_supply_index_, base_borrow_index_);
}

// нужно проверить, что сумма доллоровой стоимости залогов пользователя умноженных на borrowCollateralFactor больше чем сумма (планируемого) займа.
#[storage(read)]
fn is_borrow_collateralized(account: Address) -> bool {
    let config = get_config();
    let base_token_price = get_price(config.base_token, config.base_token_price_feed); //decimals 9
    let principal_value_ = storage.user_basic.get(account).principal; // decimals base_asset_decimal
    let present_value_ = present_value(principal_value_.flip()); // decimals base_asset_decimals; flip меняет знак
    let scale = 10.pow(config.base_token_decimals);
    
    let borrow_amount = present_value_ * I128::from_u64(base_token_price) / I128::from_u64(scale); // decimals 9
    
    let mut borrow_limit = 0;
    let mut index = 0;
    while index < config.asset_configs.len() {
        let asset_config = match config.asset_configs.get(index) {
            Option::Some(asset_config) => asset_config,
            Option::None => continue,
        };
        let balance = balance_of(contract_id(), asset_config.asset); // decimals asset_config.decimals
        let price = get_price(asset_config.asset, asset_config.price_feed); // decimals 9
        let collateral_factor = asset_config.borrow_collateral_factor; // decimals 4
        let scale = 10.pow(asset_config.decimals);

        borrow_limit += balance * price * collateral_factor / 10000 / scale; //decimals 9
        index = index + 1;
    }
    let borrow_limit = I128::from_u64(borrow_limit);
    borrow_limit >= borrow_amount  // borrow_limit >= borrow_amount
}
// нужно проверить, что сумма доллоровой стоимости залогов пользователя умноженных на liquidateCollateralFactor меньше чем сумма займа. UPD1230, добавил минус в борроу эмаунт
// @Callable is_liquidatable(account: Address) -> bool
#[storage(read)]
fn is_liquidatable_internal(account: Address) -> bool {
    let config = get_config();
    let base_token_price = get_price(config.base_token, config.base_token_price_feed); //decimals 9
    let principal_value_ = storage.user_basic.get(account).principal; // decimals base_asset_decimal
    let present_value_ = present_value(principal_value_.flip()); // decimals base_asset_decimals; flip меняет знак
    let scale = 10.pow(config.base_token_decimals);
    
    let borrow_amount = present_value_ * I128::from_u64(base_token_price) / I128::from_u64(scale); // decimals 9

    let mut liquidation_treshold = 0;
    let mut index = 0;
    while index < config.asset_configs.len() {
        let asset_config = match config.asset_configs.get(index) {
            Option::Some(asset_config) => asset_config,
            Option::None => continue,
        };
        let balance = balance_of(contract_id(), asset_config.asset); // decimals asset_config.decimals
        let price = get_price(asset_config.asset, asset_config.price_feed); // decimals 9
        let collateral_factor = asset_config.liquidate_collateral_factor; // decimals 4
        let scale = 10.pow(asset_config.decimals);

        liquidation_treshold += balance * price * collateral_factor / 10000 / scale; //decimals 9
        index = index + 1;
    }
    
    let liquidation_treshold = I128::from_u64(liquidation_treshold);
    liquidation_treshold < borrow_amount  // liquidation_treshold < borrow_amount
}

// децимал коллатерала. Может возвращать отрицательные числа
// @Callable get_collateral_reserves(asset: ContractId) -> u64
#[storage(read)]
fn get_collateral_reserves_internal(asset: ContractId) -> u64 { // base_token_decimals
    balance_of(contract_id(), asset) - storage.totals_collateral.get(asset)
}

// @Callable get_reserves_internal() -> I128
// TODO: discuss u64 -> I128 
#[storage(read)]
fn get_reserves_internal() -> I128 {  // base_token_decimals
    let config = get_config();
    let last_accrual_time = storage.market_basic.last_accrual_time;
    let (base_supply_index_, base_borrow_index_) = accrued_interest_indices(timestamp() - last_accrual_time); // decimals (18, 18)
    let balance = balance_of(contract_id(), config.base_token); // base_token_decimals
    let total_supply = present_value_supply(base_supply_index_, storage.market_basic.total_supply_base); // base_token_decimals
    let total_borrow = present_value_borrow(base_borrow_index_, storage.market_basic.total_borrow_base); // base_token_decimals
    return I128::from_u64(balance) - I128::from_u64(total_supply) + I128::from_u64(total_borrow); 
}

// все эти переменные из маркет конфига. нужно для обновления процентов и ревордов
#[storage(read, write)]
fn accrue_internal() { // TODO: Проверить работает ли обновление стораджа нормально
    let config = get_config();
    let mut market_basic = storage.market_basic;
    let now = timestamp();
    let time_elapsed = now - market_basic.last_accrual_time;
    if (time_elapsed > 0) {
        let (base_supply_index, base_borrow_index) = accrued_interest_indices(time_elapsed);
        let total_supply_base = market_basic.total_supply_base; // base_asset_decimal
        let total_borrow_base = market_basic.total_borrow_base; // base_asset_decimal
        let tracking_supply_speed = config.base_tracking_supply_speed; // decimals 18
        let tracking_borrow_speed = config.base_tracking_borrow_speed; // decimals 18
        let min_for_rewards = config.base_min_for_rewards; // decimals 6
        let scale = 10.pow(config.base_token_decimals);
    
        if (total_supply_base >= min_for_rewards) {
            market_basic.tracking_supply_index += tracking_supply_speed * time_elapsed / total_supply_base / scale; // 18 
        }
        if (total_borrow_base >= min_for_rewards) {
            market_basic.tracking_borrow_index += tracking_borrow_speed * time_elapsed / total_borrow_base / scale;  // 18 
        }
        market_basic.last_accrual_time = now;
        storage.market_basic = market_basic;
    }
}

// функция через которую будет проходить любые изменения баланса. обновляет переменные ревордов по пользователю
#[storage(write, read)]    
fn update_base_principal(account: Address, basic: UserBasic, principal_new: I128) {
    let principal = basic.principal;
    let mut basic = basic;
    basic.principal = principal_new;

    if (principal >= I128::zero()) {
        let index_delta = storage.market_basic.tracking_supply_index - basic.base_tracking_index; // decimals 18
        basic.base_tracking_accrued += principal.as_u64() * index_delta / FACTOR_SCALE; //native_asset_decimal
    } else {
        let index_delta = storage.market_basic.tracking_borrow_index  - basic.base_tracking_index;
        basic.base_tracking_accrued += principal.flip().as_u64() * index_delta / FACTOR_SCALE; //native_asset_decimal
    }
    if (principal_new >= I128::zero()) {
        basic.base_tracking_index = storage.market_basic.tracking_supply_index;
    } else {
        basic.base_tracking_index = storage.market_basic.tracking_borrow_index;
    }
    storage.user_basic.insert(account, basic);
}

fn repay_and_supply_amount(old_principal: I128, new_principal: I128) -> (u64, u64) {
    // If the new principal is less than the old principal, then no amount has been repaid or supplied
    if (new_principal < old_principal) {return (0, 0)};

    if (new_principal <= I128::zero()) {
        return ((new_principal - old_principal).as_u64(), 0);
    } else if (old_principal >= I128::zero()) {
        return (0, (new_principal - old_principal).as_u64());
    } else {
        return (old_principal.flip().as_u64(), new_principal.as_u64());
    }
}

fn withdraw_and_borrow_amount(old_principal: I128, new_principal: I128) -> (u64, u64) {
    // If the new principal is greater than the old principal, then no amount has been withdrawn or borrowed
    if (new_principal > old_principal) {return (0, 0)};

    if (new_principal >= I128::zero()) {
        return ((old_principal - new_principal).as_u64(), 0);
    } else if (old_principal <= I128::zero()) {
        return (0, (old_principal - new_principal).as_u64());
    } else {
        return ((old_principal).as_u64(), (new_principal).flip().as_u64());
    }
}

// @Callable withdraw_reserves(to: Address, amount: u64)
#[storage(read)] 
fn withdraw_reserves_internal(to: Address, amount: u64) {
    let config = get_config();
    let sender = get_caller();

    require(sender == config.governor, Error::Unauthorized);

    let reserves = get_reserves_internal();
    require(reserves >= I128::zero() && amount <= reserves.as_u64(), Error::InsufficientReserves);
    
    transfer_to_address(amount,config.base_token, to);
}

#[storage(read)]
fn get_asset_config_by_asset_id(asset: ContractId) -> AssetConfig {
    let mut out: Option<AssetConfig> = Option::None;
    let config = get_config();
    let mut i = 0;
    while i < config.asset_configs.len() {
        let asset_config = config.asset_configs.get(i).unwrap();
        if asset_config.asset == asset {
            out = Option::Some(asset_config);
        }
        i += 1;
    }
    match out {
        Option::Some(v) => v,
        Option::None(_) => revert(0),
    }
}

// функция которая считает курс покупки залога пользователем у нашего протокола, то есть сколько коллатрала получает пользователь за выкуп одного доллара ликвидируемого долга
// @Callable quote_collateral(asset: ContractId, base_amount: u64) -> u64 
#[storage(read)]
fn quote_collateral_internal(asset: ContractId, base_amount: u64) -> u64 { // asset decimals
    let config = get_config();
    let asset_config = get_asset_config_by_asset_id(asset);
    let asset_price = get_price(asset, asset_config.price_feed); // decimals 9
    
    let base_price = get_price(config.base_token, config.base_token_price_feed); // decimals 9

    let store_front_price_factor = config.store_front_price_factor; //decimals 4
    let liquidate_collateral_factor = asset_config.liquidate_collateral_factor; //decimals 4
    
    // Store front discount is derived from the collateral asset's liquidate_collateral_factor and store_front_price_factor
    // discount = store_front_price_factor * (1e18 - liquidate_collateral_factor)
    let discount_factor = store_front_price_factor - liquidate_collateral_factor; // decimals 4
    let asset_price_discounted = asset_price - discount_factor * 10.pow(9 - 4); // decimals 9
    
    // # of collateral assets
    // = (TotalValueOfBaseAmount / DiscountedPriceOfCollateralAsset) * assetScale
    // = ((basePrice * baseAmount / baseScale) / assetPriceDiscounted) * assetScale
    let base_scale = 10.pow(config.base_token_decimals);
    let asset_scale = 10.pow(asset_config.decimals);
    return base_price * base_amount * asset_scale / asset_price_discounted / base_scale;
}
   
// функця переводит залог в собствнность протокола и закрывает долг пользователя
// @Callable absorb(absorber: Address, accounts: Vec<Address>)
#[storage(read, write)]
// FIXME: Разробраться что тут приходит в пементе
fn absorb_internal(absorber: Address, account: Address) {
    require(is_liquidatable(), Error::NotLiquidatable);

    let account_user = storage.user_basic.get(account);
    let old_principal = account_user.principal;
    let old_balance = present_value(old_principal); // base_asset_decimals
    // let assetsIn = get_user_assets(account);

    let config = get_config();
    let base_price = get_price(config.base_token, config.base_token_price_feed); //decimals 9
    let base_scale = 10.pow(config.base_token_decimals);
    let mut delta_value = 0; // decimals 9

    let mut i = 0;
    while i < config.asset_configs.len() {
        let asset_config = config.asset_configs.get(i).unwrap();
        let asset = asset_config.asset;
        let seize_amount = storage.user_collateral.get((account, asset)); // asset decimals
        if seize_amount == 0 {continue;}
        storage.user_collateral.insert((account, asset), 0);

        let total_collateral = storage.totals_collateral.get(asset); // asset decimals
        storage.totals_collateral.insert(asset, total_collateral - seize_amount);
        
        let price = get_price(asset, asset_config.price_feed); // decimals 9
        let liquidation_penalty = asset_config.liquidation_penalty; // decimals 4
        let asset_scale = 10.pow(asset_config.decimals);
        let panalty_scale = 10.pow(4); 
        delta_value += seize_amount * price * liquidation_penalty / asset_scale / panalty_scale; // decimals 9

        i += 1;
    }
    let delta_balance = delta_value * base_scale / base_price; // base_asset_decimals
    let mut new_balance = old_balance + I128::from_u64(delta_balance); // base_asset_decimals
    if (new_balance < I128::zero()) {
        new_balance = I128::zero();
    }

    let new_principal = principal_value(new_balance);
    update_base_principal(account, account_user, new_principal);

    let (repay_amount, supply_amount) = repay_and_supply_amount(old_principal, new_principal);

    // Reserves are decreased by increasing total supply and decreasing borrows
    //  the amount of debt repaid by reserves is `newBalance - oldBalance`
    let mut market_basic = storage.market_basic;
    market_basic.total_supply_base += supply_amount;
    market_basic.total_borrow_base -= repay_amount;
    storage.market_basic = market_basic;

    // TODO: Сейчас supply_amount это u64, это значит что оно всегда больше нуля!!!
    // если supply_amount > 0, выпускаем LP токен в количестве равном supply_amount и отправляем его пользователю
    if supply_amount > 0 {
        mint_to_address(supply_amount, account); //TODO: Разобоаться кому отправлять LP токены
    }
}

// функция для покупки залога ликвидированного пользователя. 
// @Callable buy_collateral(asset: ContractId, min_amount: u64, base_amount: u64, recipient: Address) 
#[storage(read)]
// FIXME: Разробраться что тут приходит в пементе
fn buy_collateral_internal(asset: ContractId, min_amount: u64, base_amount: u64, recipient: Address) {
    require(!is_buy_paused(), Error::Paused);

    let config = get_config();
    let reserves = get_reserves_internal();
    require(reserves < I128::zero() || reserves.as_u64() < config.target_reserves, Error::NotForSale);

    // Note: Re-entrancy can skip the reserves check above on a second buyCollateral call.

    let collateral_amount = quote_collateral_internal(asset, base_amount);
    require(collateral_amount >= min_amount, Error::TooMuchSlippage);
    require(collateral_amount <= get_collateral_reserves_internal(asset), Error::InsufficientReserves);

    // Note: Pre-transfer hook can re-enter buyCollateral with a stale collateral ERC20 balance.
    //  Assets should not be listed which allow re-entry from pre-transfer now, as too much collateral could be bought.
    //  This is also a problem if quoteCollateral derives its discount from the collateral ERC20 balance.
    transfer_to_address(collateral_amount, asset, recipient);
}

// @Callable supply_collateral(from: Address, dst: Address, asset: ContractId, amount: u64)
#[storage(read, write)]
fn supply_collateral_internal(from: Address, dst: Address, asset: ContractId, amount: u64) {
    require(!is_supply_paused(), Error::Paused);
    
    let asset_config = get_asset_config_by_asset_id(asset);
    let mut total_supply_asset = storage.totals_collateral.get(asset);
    total_supply_asset += amount;
    require(total_supply_asset <= asset_config.supply_cap, Error::SupplyCapExceeded);

    let dst_collateral = storage.user_collateral.get((dst, asset));
    let dst_collateral_new = dst_collateral + amount;

    storage.totals_collateral.insert(asset, total_supply_asset);
    storage.user_collateral.insert((dst, asset), dst_collateral_new);
}

// @Callable withdraw_collateral(asset: ContractId, amount: u64)
#[storage(read, write)]
fn withdraw_collateral_internal(asset: ContractId, amount: u64) {
    let caller = get_caller();
    let src_collateral = storage.user_collateral.get((caller, asset));
    let src_collateral_new = src_collateral - amount;

    let new_total_supply_asset = storage.totals_collateral.get(asset) - amount;
    storage.totals_collateral.insert(asset, new_total_supply_asset);
    storage.user_collateral.insert((caller, asset), src_collateral_new);

    let asset_config = get_asset_config_by_asset_id(asset);

    // Note: no accrue interest, BorrowCF < LiquidationCF covers small changes
    require(is_borrow_collateralized(caller), Error::NotCollateralized);

    transfer_to_address(amount, asset, caller);
}

// @Callable supply_base()
#[storage(read, write)]
// FIXME: Разробраться что тут приходит в пементе
fn supply_base_internal()  {
    require(!is_supply_paused(), Error::Paused);
    let caller = get_caller();
    let amount = msg_amount();
    // let asset = msg_asset_id();
    accrue_internal();

    let dst_user = storage.user_basic.get(caller);
    let dst_principal = dst_user.principal;
    let dst_balance = present_value(dst_principal) + I128::from_u64(amount);
    let dst_principal_new = principal_value(dst_balance);

    let (repay_amount, supply_amount) = repay_and_supply_amount(dst_principal, dst_principal_new);

    let mut market_basic = storage.market_basic;
    market_basic.total_supply_base += supply_amount;
    market_basic.total_borrow_base -= repay_amount;
    storage.market_basic = market_basic;

    update_base_principal(caller, dst_user, dst_principal_new);

    // TODO: Сейчас supply_amount это u64, это значит что оно всегда больше нуля!!!
    // если supply_amount > 0, выпускаем LP токен в количестве равном supply_amount и отправляем его пользователю
    if supply_amount > 0 {
        mint_to_address(supply_amount, caller); //TODO: Разобоаться кому отправлять LP токены
    }

}

// @Callable withdraw_base(amount: u64)
#[storage(read, write)]
fn withdraw_base_internal(amount: u64)  {    
    require(!is_withdraw_paused(), Error::Paused);

    accrue_internal();

    let config = get_config();
    let caller = get_caller();
    let src_user = storage.user_basic.get(caller);
    let src_principal = src_user.principal;
    let src_balance = present_value(src_principal) - I128::from_u64(amount);
    let src_principal_new = principal_value(src_balance);

    let (withdraw_amount, borrow_amount) = withdraw_and_borrow_amount(src_principal, src_principal_new);

    let mut market_basic = storage.market_basic;
    market_basic.total_supply_base -= withdraw_amount;
    market_basic.total_borrow_base += borrow_amount;
    storage.market_basic = market_basic;

    update_base_principal(caller, src_user, src_principal_new);

    if (src_balance < I128::zero()) {
        require(src_balance.flip().as_u64() >= config.base_borrow_min, Error::BorrowTooSmall);
        require(is_borrow_collateralized(caller), Error::NotCollateralized);
    }

    transfer_to_address(amount, config.base_token, caller);

    // сжигает LP токен в количестве равном withdrawAmount, которые пользователь отправил при снятии
    burn(withdraw_amount);

}

// функция для снятия реворд токенов админом. reward_token задается в конфиге
// @Callable withdraw_reward_token(to: Address, amount: u64)
#[storage(read)]
fn withdraw_reward_token_internal(to: Address, amount: u64) {
    let config = get_config();
    let sender = get_caller();
    require(sender == config.governor, Error::NotPermitted(sender));
    mint_reward_token(amount);
    transfer_to_address(amount, config.reward_token, to);
}

// @Callable get_reward_owed(account: Address) -> u64
#[storage(read, write)]
fn get_reward_owed_internal(account: Address) -> u64 {
    accrue_internal();

    let basic = storage.user_basic.get(account);
    update_base_principal(account, basic, basic.principal);

    let claimed = storage.user_basic.get(account).reward_claimed;
    let accrued = storage.user_basic.get(account).base_tracking_accrued;

    if accrued > claimed {accrued - claimed} else{0}
}

// @Callable claim()
#[storage(read, write)]
fn claim_internal(){
    require(!is_claim_paused(), Error::Paused);
    let caller = get_caller();
    let config = get_config();

    accrue_internal();

    let basic = storage.user_basic.get(caller);
    update_base_principal(caller, basic, basic.principal);

    let claimed = storage.user_basic.get(caller).reward_claimed;
    let accrued = storage.user_basic.get(caller).base_tracking_accrued;

    if (accrued > claimed) {
        let owed = accrued - claimed;
        // rewards_claimed[comet][src] = accrued; // TODO: что за rewards_claimed?
        mint_reward_token(owed);
        transfer_to_address(owed, config.reward_token, caller);
    }
}


// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
impl Market for Contract {
    #[storage(read)]
    fn get_oracle_price(asset: ContractId) -> u64 {
        let base_token_price_feed = get_config().base_token_price_feed;
        return get_price(asset, base_token_price_feed)
    }

    #[storage(write)]
    fn initialize(config: MarketConfiguration) {
        storage.config = Option::Some(config);
    }

    #[storage(write, read)]
    fn pause(pause_config: PauseConfiguration) {
        let sender = get_caller();
        let config = get_config();
        require(sender == config.governor || sender == config.pause_guardian, Error::Unauthorized);
        storage.pause_config = Option::Some(pause_config);
    }

    #[storage(read)]
    fn get_configuration() -> MarketConfiguration {
        get_config()
    }
    //-----------------------------------
    #[storage(read)]
    fn get_utilization() -> u64 {
        get_utilization_internal()
    }
    
    #[storage(read)]
    fn get_supply_rate(utilization: u64) -> u64{
        get_supply_rate_internal(utilization)
    }
    
    #[storage(read)]
    fn get_borrow_rate(utilization: u64) -> u64{
        get_borrow_rate_internal(utilization)
    }
    
   #[storage(read)]
    fn is_liquidatable(account: Address) -> bool{
        is_liquidatable_internal(account)
    }
    
    #[storage(read)]
    fn get_collateral_reserves(asset: ContractId) -> u64{
        get_collateral_reserves_internal(asset)
    }

    #[storage(read)]
    fn get_reserves() -> I128 {
        get_reserves_internal()
    }

    #[storage(read)]
    fn withdraw_reserves(to: Address, amount: u64){
        withdraw_reserves_internal(to, amount)
    }

    #[storage(read)]
    fn quote_collateral(asset: ContractId, base_amount: u64) -> u64{
        quote_collateral_internal(asset, base_amount)
    }

    #[storage(read, write)]
    fn absorb(absorber: Address, accounts: Vec<Address>) {
        require(!is_absorb_paused(), Error::Paused);    
        accrue_internal();
        let mut i = 0;
        while i < accounts.len() {
            absorb_internal(absorber, accounts.get(i).unwrap());
            i += 1;
        }
    }
    #[storage(read)]
    fn buy_collateral(asset: ContractId, min_amount: u64, base_amount: u64, recipient: Address) {
        buy_collateral_internal(asset, min_amount, base_amount, recipient)
    }

    #[storage(read, write)]
    fn supply_collateral(from: Address, dst: Address, asset: ContractId, amount: u64){
        supply_collateral_internal(from, dst, asset, amount)
    }

    #[storage(read, write)]
    fn withdraw_collateral(asset: ContractId, amount: u64){
        withdraw_collateral_internal(asset, amount)
    }

    #[storage(read, write)]
    fn supply_base(){
        supply_base_internal()
    }

    #[storage(read, write)]
    fn withdraw_base(amount: u64){
        withdraw_base_internal(amount)
    }

    #[storage(read)]
    fn withdraw_reward_token(to: Address, amount: u64){
        withdraw_reward_token_internal(to, amount)
    }

    #[storage(read, write)]
    fn get_reward_owed(account: Address) -> u64 {
        get_reward_owed_internal(account)
    }

    #[storage(read, write)]
    fn claim() {
        claim_internal()
    }
}
