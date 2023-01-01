script;

use std::logging::log;
use oracle_abi::*;

fn get_price(asset: ContractId, price_feed: ContractId) -> u64 {
    let res = abi(Oracle, price_feed.value).get_price(asset);
    res.price
}



fn main() -> bool {
    let token = ContractId::from(0x851ec5e04fa3485ba0794b34030bbcd70e96be282cd429da03c58e8de4d46c00);
    let price_feed = ContractId::from(0xde764394c83bb3c8a3aec5f75f383ff86e64728964fab4469df5910ca01b1a59);
    let price = get_price(token, price_feed);
    assert(price == 16567367366490);
    true
}
