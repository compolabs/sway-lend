use crate::utils::local_tests_utils::market::{self};

fn toggle(mut num: u16, index: u16) -> u16 {
    num ^= 1 << index;
    println!("num = {num:#b}");
    num
}
fn set_bit(mut assets_in: u16, asset_offset: u16) -> u16 {
    assets_in = assets_in | 1 << asset_offset;
    println!("assets_in = {assets_in:#b}");
    assets_in
}
fn clear_bit(mut assets_in: u16, asset_offset: u16) -> u16 {
    assets_in = assets_in & !(1 << asset_offset);
    println!("assets_in = {assets_in:#b}");
    assets_in
}

fn is_in_asset(assets_in: u16, asset_offset: u8) -> bool {
    assets_in & (1 << asset_offset) != 0
}

// fn update_assets_in(
//     address account,
//     AssetInfo memory assetInfo,
//     uint128 initialUserBalance,
//     uint128 finalUserBalance
// ) internal {
//     if (initialUserBalance == 0 && finalUserBalance != 0) {
//         // set bit for asset
//         userBasic[account].assetsIn |= (uint16(1) << assetInfo.offset);
//     } else if (initialUserBalance != 0 && finalUserBalance == 0) {
//         // clear bit for asset
//         userBasic[account].assetsIn &= ~(uint16(1) << assetInfo.offset);
//     }
// }

#[tokio::test]
async fn test() {
    let empty = 0;
    println!("empty = {empty:#b}");

    let len = 5;

    let assets_in = set_bit(empty, 4);
    let assets_in = set_bit(assets_in, 0);
    let assets_in = set_bit(assets_in, 1);
    let assets_in = set_bit(assets_in, 2);
    let assets_in = set_bit(assets_in, 3);

    let assets_in = clear_bit(assets_in, 3);
    // let assets_in = clear_bit(assets_in, 2);
    let assets_in = clear_bit(assets_in, 1);

    let mut i = 0;
    while i < len {
        if is_in_asset(assets_in, i) {
            println!(" {assets_in:#b}[{i}] = 1 ");
            //todo
            i += 1;
        } else {
            println!(" {assets_in:#b}[{i}] = 0 ");
            i += 1;
            continue;
        }
    }

    // let mut i = 0;
}

/*

i = 0
asset = 0000000000000000000000000000000000000000000000000000000000000000
i = 1
asset = 5e5c9d81b62bb0f2528fa523efa3cdf9bcaee9b5c465fe657b3eff9436c083e2
i = 2
asset = 3a919adf9fd85c0fff8413bc77304a2c53275f7c0068b91f0c9ef09164909ae7
i = 3
asset = 1ff3492f350d82f73642b539ef7e3aa3a5c64e8862606101c439fc3b9159ae33
i = 4
asset = c9ae82d0fab16fd5f203f8deb1bb454db046f31b81e8a96c1f695d746e71f75c

*/
