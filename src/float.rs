use super::control_and_status::*;
use bytemuck::Pod;
use core::cmp::Ordering;
use core::fmt;
use core::mem;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use core::str::FromStr;
use num_traits::{
    ConstOne, ConstZero, Euclid, FloatConst, FromBytes, Inv, MulAdd, MulAddAssign, One, Pow,
    ToBytes, Unsigned, Zero,
};

#[derive(Copy, Clone)]
pub struct AngFloat<
    Repr: ?Sized,
    const ANGLE_BITS: usize,
    const EXPONENT_BITS: usize,
    const EXPONENT_BIAS: u128,
    const MANTISSA_BITS: usize,
> {
    repr: Repr,
}

impl<
        Repr: Pod,
        const ANGLE_BITS: usize,
        const EXPONENT_BITS: usize,
        const EXPONENT_BIAS: u128,
        const MANTISSA_BITS: usize,
    > AngFloat<Repr, ANGLE_BITS, EXPONENT_BITS, EXPONENT_BIAS, MANTISSA_BITS>
{
    pub const fn from_repr(repr: Repr) -> Self {
        const {
            assert!(MANTISSA_BITS >= 1);
            assert!(EXPONENT_BITS >= 1);
            let repr_size: usize = mem::size_of::<Repr>();
            let needed_bits: usize = ANGLE_BITS + EXPONENT_BITS + MANTISSA_BITS;
            assert!(ANGLE_BITS.div_ceil(8) <= repr_size);
            assert!(EXPONENT_BITS.div_ceil(8) <= repr_size);
            assert!(MANTISSA_BITS.div_ceil(8) <= repr_size);
            assert!(needed_bits.div_ceil(8) <= repr_size);
        }

        AngFloat { repr }
    }

    pub const fn into_repr(self) -> Repr {
        self.repr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
