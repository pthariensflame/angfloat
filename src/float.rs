use super::control_and_status::*;
use core::cmp::Ordering;
use core::fmt;
use core::num::NonZeroU8;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use core::str::FromStr;

#[derive(Copy, Clone)]
pub struct Float<
    const ANGLE_BITS: u8,
    const EXPONENT_BITS: u8,
    const EXPONENT_BIAS: u128,
    const MANTISSA_BITS: u8,
>
{
}

#[cfg(test)]
mod tests {
    use super::*;
}
