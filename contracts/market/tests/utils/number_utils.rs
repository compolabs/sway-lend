// TODO: Добавить возможность принимать флоаты
pub fn parse_units(num: u64, decimals: u8) -> u64 {
    num * 10u64.pow(decimals as u32)
}

pub fn format_units(num: u64, decimals: u8) -> f64 {
    num as f64 / 10u64.pow(decimals as u32) as f64
}
