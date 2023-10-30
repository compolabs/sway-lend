library;

pub struct I64 {
    value: u64,
    negative: bool,
}

impl From<u64> for I64 {
    fn from(value: u64) -> Self {
        Self {value, negative: false}
    }

    fn into(self) -> u64 {
        if !self.negative {
            self.value
        } else {
            revert(0)
        }
    }
}

impl core::ops::Eq for I64 {
    fn eq(self, other: Self) -> bool {
        self.value == other.value && self.negative == other.negative
    }
}

impl core::ops::Ord for I64 {
    fn gt(self, other: Self) -> bool {
        if (!self.negative && !other.negative) {
            self.value > other.value
        } else if (!self.negative && other.negative) {
            true
        } else if (self.negative && !other.negative) {
            false
        } else if (self.negative && other.negative) {
            self.value < other.value
        } else {
            revert(0)
        }
    }

    fn lt(self, other: Self) -> bool {
        if (!self.negative && !other.negative) {
            self.value < other.value
        } else if (!self.negative && other.negative) {
            false
        } else if (self.negative && !other.negative) {
            true
        } else if (self.negative && other.negative) {
            self.value > other.value
        } else {
            revert(0)
        }
    }
}

impl I64 {
    pub fn ge(self, other: Self) -> bool {
        self > other || self == other
    }

    pub fn le(self, other: Self) -> bool {
        self < other || self == other
    }

    /// The size of this type in bits.
    pub fn bits() -> u32 {
        64
    }

    /// Helper function to get a signed number from with an underlying
    pub fn from_uint(value: u64) -> Self {
        Self {
            value,
            negative: false,
        }
    }

    /// The largest value that can be represented by this integer type,
    pub fn max() -> Self {
        Self {
            value: u64::max(),
            negative: false,
        }
    }

    /// The smallest value that can be represented by this integer type.
    pub fn min() -> Self {
        Self {
            value: u64::min(),
            negative: true,
        }
    }

    /// Helper function to get a negative value of an unsigned number
    pub fn neg_from(value: u64) -> Self {
        Self {
            value,
            negative: if value == 0 { false } else { true },
        }
    }

    /// Initializes a new, zeroed I64.
    pub fn new() -> Self {
        Self {
            value: 0,
            negative: false,
        }
    }

}

impl core::ops::Add for I64 {
    /// Add a I64 to a I64. Panics on overflow.
    fn add(self, other: Self) -> Self {
        if !self.negative && !other.negative {
            Self::from(self.value + other.value)
        } else if self.negative && other.negative {
            Self::neg_from(self.value + other.value)
        } else if (self.value > other.value) {
            Self {
                negative: self.negative,
                value: self.value - other.value,
            }
        } else if (self.value < other.value) {
            Self {
                negative: other.negative,
                value: other.value - self.value,
            }
        } else if (self.value == other.value) {
            Self::new()
        } else{
            revert(0)
        }
    }
}

impl core::ops::Subtract for I64 {
    /// Subtract a I64 from a I64. Panics of overflow.
    fn subtract(self, other: Self) -> Self {
        if self == other {Self::new()}
        else if !self.negative && !other.negative && self.value > other.value {
            Self::from(self.value - other.value)
        } else if !self.negative && !other.negative && self.value < other.value  {
            Self::neg_from(other.value - self.value)
        } else if self.negative && other.negative && self.value > other.value {
            Self::neg_from(self.value - other.value)
        } else if self.negative && other.negative && self.value < other.value  {
            Self::from(other.value - self.value)
        } else if !self.negative && other.negative{
            Self::from(self.value + other.value)
        } else if self.negative && !other.negative {
            Self::neg_from(self.value + other.value)
        }  else{
            revert(0)
        }
    }
}

impl core::ops::Multiply for I64 {
    /// Multiply a I64 with a I64. Panics of overflow.
    fn multiply(self, other: Self) -> Self {
        if self.value == 0 || other.value == 0{
            Self::new()    
        }else if !self.negative == !other.negative {
            Self::from(self.value * other.value)
        }else if !self.negative != !other.negative{
            Self::neg_from(self.value * other.value)
        } else{
            revert(0)
        }
    }
}

impl core::ops::Divide for I64 {
    /// Divide a I64 by a I64. Panics if divisor is zero.
    fn divide(self, divisor: Self) -> Self {
        require(divisor != Self::new(), "ZeroDivisor");
        if self.value == 0{
            Self::new()    
        }else if !self.negative == !divisor.negative {
            Self::from(self.value / divisor.value)
        }else if !self.negative != !divisor.negative{
            Self::neg_from(self.value * divisor.value)
        } else{
            revert(0)
        }
    }
}

impl I64 {
    pub fn flip(self) -> Self {
        self * Self::neg_from(1)
    }
}
