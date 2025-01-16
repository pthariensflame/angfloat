use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Angle4 {
    // invariant: only low 4 bits may be set
    bits: u8,
}

impl Angle4 {
    #[inline]
    pub const fn from_bits(bits: u8) -> Self {
        Self {
            bits: bits & 0b1111,
        }
    }

    #[inline]
    pub const fn into_bits(self) -> u8 {
        self.bits
    }

    #[inline]
    pub fn abs(self) -> Self {
        if (self.into_bits() & 0b1000) == 0 {
            self
        } else {
            -self
        }
    }

    #[inline]
    pub fn neg_abs(self) -> Self {
        -self.abs()
    }
}

impl Add for Angle4 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self::from_bits(self.bits.wrapping_add(other.bits))
    }
}

impl AddAssign for Angle4 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Angle4 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self::from_bits(self.bits.wrapping_sub(other.bits))
    }
}

impl SubAssign for Angle4 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Neg for Angle4 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::from_bits(self.bits.wrapping_neg())
    }
}
