contract;

use std::address::Address;
use std::auth::{AuthError,msg_sender};
use std::block::timestamp;
use std::constants::ZERO_B256;
use std::identity::Identity;
use std::logging::log;
use std::result::Result;
use std::revert::require;
use std::hash::Hash;

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

struct Price {
    asset_id: AssetId,
    price: u64,
    last_update: u64,
}

configurable {
    ADMIN: Address = Address::from(ZERO_B256),
}

storage {
    prices: StorageMap<AssetId, Price> = StorageMap {},
}

pub fn get_msg_sender_address_or_panic() -> Address {
    let sender: Result<Identity, AuthError> = msg_sender();
    if let Identity::Address(address) = sender.unwrap() {
        address
    } else {
        revert(0);
    }
}

#[storage(read)]
fn validate_owner() {
    let sender = get_msg_sender_address_or_panic();
    require(ADMIN == sender, "Access denied");
}

impl Oracle for Contract {   
    #[storage(read)]
    fn owner() -> Identity {
        Identity::Address(ADMIN)
    }

    #[storage(read, write)]
    fn set_price(asset_id: AssetId, price: u64) {
        validate_owner();
        storage.prices.insert(asset_id, Price {
            price,
            asset_id,
            last_update: timestamp(),
        });
    }
 
    #[storage(read, write)]
    fn set_prices(prices: Vec<(AssetId, u64)>) {
        validate_owner();
        let mut i = 0;
        while i < prices.len() {
            let (asset_id, price) = prices.get(i).unwrap();
            storage.prices.insert(asset_id, Price {
                price,
                asset_id,
                last_update: timestamp(),
            });
            i += 1;
        }
    }
    
    #[storage(read)]
    fn get_price(asset_id: AssetId) -> Price {
        storage.prices.get(asset_id).read()
    }
}
