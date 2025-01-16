#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Mantissa7 {
    // invariant: only low 7 bits may be unset; top bit must be set
    bits: u8,
}

impl Mantissa7 {
    #[inline]
    pub const fn from_bits(bits: u8) -> Self {
        Self {
            bits: bits | 0b10000000,
        }
    }

    #[inline]
    pub const fn into_bits(self) -> u8 {
        self.bits & 0b1111111
    }
}
