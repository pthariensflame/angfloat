use super::control_and_status::*;
use core::cmp::Ordering;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use core::str::FromStr;
use num_traits::{FloatConst, PrimInt};

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct AngFloat<
    Repr: PrimInt,
    const ANGLE_BITS: usize,
    const EXPONENT_BITS: usize,
    const EXPONENT_BIAS: u128,
    const MANTISSA_BITS: usize,
> {
    pub repr: Repr,
}

#[cfg(test)]
mod tests {
    use super::*;
}
