use crate::{angle::Angle4, exponent::Exponent5, mantissa::Mantissa7};

#[derive(Clone, Copy, Hash, Debug, Default)]
#[repr(transparent)]
pub struct Binary16C {
    // angle:4
    // exponent:5
    // mantissa:7
    bits: u16,
}

impl Binary16C {
    #[inline]
    pub const fn from_bits(bits: u16) -> Self {
        Self { bits }
    }

    #[inline]
    pub const fn into_bits(self) -> u16 {
        self.bits
    }

    #[inline]
    pub const fn angle(self) -> Angle4 {
        Angle4::from_bits((self.bits >> (5 + 7)) as u8)
    }

    #[inline]
    pub const fn exponent(self) -> Exponent5 {
        Exponent5::from_bits((self.bits >> 7) as u8)
    }

    #[inline]
    pub const fn mantissa(self) -> Mantissa7 {
        Mantissa7::from_bits(self.bits as u8)
    }

    pub const fn from_parts(angle: Angle4, exponent: Exponent5, mantissa: Mantissa7) -> Self {
        Self::from_bits(
            ((angle.into_bits() as u16) << (5 + 7))
                | ((exponent.into_bits() as u16) << 7)
                | (mantissa.into_bits() as u16),
        )
    }
}
