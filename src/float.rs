use super::control_and_status::*;
use core::cmp::Ordering;
use core::fmt;
use core::num::NonZeroU8;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use core::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct FloatSpec {
    pub angle_bits: u8,
    pub exponent_bits: NonZeroU8,
    pub exponent_bias: u128,
    pub mantissa_bits: NonZeroU8,
}

#[cfg(test)]
mod tests {
    use super::*;
}
