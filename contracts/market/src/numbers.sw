// SPDX-License-Identifier: BUSL-1.1
library;
use std::u128::U128;

impl U128 {
    pub fn from_u64(value: u64) -> U128 {
        U128::from((0, value))
    }
}

impl U128{
    pub fn ge(self, other: Self) -> bool {
        self > other || self == other
    }

    pub fn le(self, other: Self) -> bool {
        self < other || self == other
    }
}

// impl I128 {
//     pub fn from_u64(value: u64) -> I128 {
//         Self {
//             underlying: U128::from((0, value))
//         }
//     }

//     pub fn as_u64(self) -> u64 {
//         if self.underlying < Self::indent() {revert(0)} else {self.underlying.as_u64().unwrap()}
//     }

//     pub fn flip(self) -> Self {
//         self * Self {
//             underlying: Self::indent() - self.underlying,
//         }
//     }
// }

// impl I128{
//     pub fn ge(self, other: Self) -> bool {
//         self > other || self == other
//     }

//     pub fn le(self, other: Self) -> bool {
//         self < other || self == other
//     }

//     pub fn zero() -> I128 {
//         Self::from_u64(0)
//     }
// }