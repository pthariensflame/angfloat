use bitflags::bitflags;

bitflags! {
    #[must_use]
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    #[repr(transparent)]
    pub struct Status: u8 {
        const OK = 0x00;
        const INVALID_OP = 0x01;
        const DIV_BY_ZERO = 0x02;
        const OVERFLOW = 0x04;
        const UNDERFLOW = 0x08;
        const INEXACT = 0x10;
    }
}

#[must_use]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct StatusAnd<T> {
    pub status: Status,
    pub value: T,
}

impl<T> StatusAnd<T> {
    pub fn map_value<F: FnOnce(T) -> U, U>(self, f: F) -> StatusAnd<U> {
        StatusAnd {
            status: self.status,
            value: f(self.value),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Category {
    Infinity,
    NaN,
    Normal,
    Zero,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Round {
    Odd,
    Nearest {
        ties_to: RoundTiesDirection,
    },
    Directed {
        polarity: RoundDirectionPolarity,
        direction: RoundDirection,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum RoundTiesDirection {
    Even,
    Directed {
        polarity: RoundDirectionPolarity,
        direction: RoundDirection,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum RoundDirectionPolarity {
    Towards,
    Away,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum RoundDirection {
    Infinity,
    Zero,
}

#[cfg(test)]
mod tests {
    use super::*;
}
