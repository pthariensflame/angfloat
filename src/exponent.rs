#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Exponent5 {
    // invariant: only low 5 bits may be set
    bits: u8,
}

impl Exponent5 {
    #[inline]
    pub const fn from_bits(bits: u8) -> Self {
        Self {
            bits: bits & 0b11111,
        }
    }

    #[inline]
    pub const fn into_bits(self) -> u8 {
        self.bits
    }
}
