/// Decimal struct representing a number.
///
/// If the first bit of high is 0, then the number is:
///   high + med * 2^{-32} + low * 2^{-32*2}
/// If the first bit of high is 1, then the number is:
///   -2^{32} + high + med * 2^{-32} + low * 2^{-32*2}
///
/// The number is 95 bits (first bit indicates positive or negative)
/// and so prec indicates how many bits are precise, including
/// leading zeros.
///
#[derive(Copy, Clone, Debug)]
pub struct Decimal {
    high: u32,
    med: u32,
    low: u32,
    prec: u8,
}

impl Decimal {
    pub fn new(high: u32, med: u32, low: u32, prec: u8) -> Result<Decimal, String> {
        if prec == 0 || prec > 95 {
            Err("invalid prec, must be in (0, 96)".to_owned())
        } else {
            Ok(Decimal {
                high,
                med,
                low,
                prec,
            })
        }
    }

    /// provide a string representation in decimal
    pub fn to_string(&self) -> String {
        todo!()
    }

    /// indicates whether number is positive
    ///
    /// number might not have all zero bits if
    /// it is zero to its level of precision
    pub fn is_positive(&self) -> bool {
        (self.high >> 31) == 0 && !self.is_zero()
    }

    /// indicates whether number is zero taking precision into account
    pub fn is_zero(&self) -> bool {
        if self.prec < 31 {
            ((self.high << 1) >> (self.prec + 1)) == 0
        } else if self.prec < 63 {
            ((self.high << 1) == 0) && ((self.med >> (self.prec - 31)) == 0)
        } else if self.prec < 95 {
            ((self.high << 1) == 0) && (self.med == 0) && ((self.high >> (self.prec - 63)) == 0)
        } else {
            ((self.high << 1) == 0) && (self.med == 0) && (self.high == 0)
        }
    }
}

/// Largest possible value at max precision
pub const MAX: Decimal = Decimal {
    high: 2_147_483_647,
    low: 4_294_967_295,
    med: 4_294_967_295,
    prec: 95,
};

/// Lowest possible value at max precision
pub const MIN: Decimal = Decimal {
    high: 4_294_967_295,
    low: 4_294_967_295,
    med: 4_294_967_295,
    prec: 95,
};

/// Zero at max precision
pub const ZERO: Decimal = Decimal {
    high: 0,
    low: 0,
    med: 0,
    prec: 95,
};
