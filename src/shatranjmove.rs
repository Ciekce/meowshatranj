use crate::core::{Square, SquareStrError};
use std::fmt::{Display, Formatter};
use std::num::NonZeroU16;
use std::str::FromStr;

// Compatible with oranjformat

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Move {
    raw: NonZeroU16,
}

impl Move {
    const SQUARE_BITS: usize = 6;
    const FLAG_BITS: usize = 1;

    const TOTAL_BITS: usize = Self::SQUARE_BITS * 2 + Self::FLAG_BITS;

    const SQUARE_MASK: u16 = (1 << Self::SQUARE_BITS) - 1;
    const FLAG_MASK: u16 = (1 << Self::FLAG_BITS) - 1;

    const VALID_MASK: u16 = (1 << Self::TOTAL_BITS) - 1;

    // Make a1a1 representable by always setting the msb internally
    const PRESENT_BIT: u16 = 1 << 15;

    const FROM_SHIFT: usize = 0;
    const TO_SHIFT: usize = 6;
    const PROMO_SHIFT: usize = 12;

    #[must_use]
    pub const fn new(from: Square, to: Square) -> Self {
        let mut value = Self::PRESENT_BIT;

        value |= (from.raw() as u16) << Self::FROM_SHIFT;
        value |= (to.raw() as u16) << Self::TO_SHIFT;

        Self {
            // SAFETY: `value` always has `PRESENT_BIT` set, and thus is nonzero
            raw: unsafe { NonZeroU16::new_unchecked(value) },
        }
    }

    #[must_use]
    pub const fn new_promo(from: Square, to: Square) -> Self {
        let mut value = Self::PRESENT_BIT;

        value |= (from.raw() as u16) << Self::FROM_SHIFT;
        value |= (to.raw() as u16) << Self::TO_SHIFT;
        value |= 1 << Self::PROMO_SHIFT;

        Self {
            // SAFETY: `value` always has `PRESENT_BIT` and the promo flag set, and thus is nonzero
            raw: unsafe { NonZeroU16::new_unchecked(value) },
        }
    }

    #[must_use]
    pub const unsafe fn from_raw_unchecked(raw: u16) -> Self {
        debug_assert!(raw & !Self::VALID_MASK == 0);
        Self {
            raw: unsafe { NonZeroU16::new_unchecked(raw | Self::PRESENT_BIT) },
        }
    }

    #[must_use]
    pub const fn from_raw(raw: u16) -> Option<Self> {
        if raw & !Self::VALID_MASK != 0 {
            None
        } else {
            // SAFETY: we just checked that the pattern is valid
            Some(unsafe { Self::from_raw_unchecked(raw) })
        }
    }

    #[must_use]
    pub const fn from_sq(self) -> Square {
        let raw = (self.raw.get() >> Self::FROM_SHIFT) & Self::SQUARE_MASK;
        // SAFETY: any subset of SQUARE_MASK is a valid square index
        unsafe { Square::from_raw_unchecked(raw as u8) }
    }

    #[must_use]
    pub const fn to_sq(self) -> Square {
        let raw = (self.raw.get() >> Self::TO_SHIFT) & Self::SQUARE_MASK;
        // SAFETY: any subset of SQUARE_MASK is a valid square index
        unsafe { Square::from_raw_unchecked(raw as u8) }
    }

    #[must_use]
    pub const fn is_promo(self) -> bool {
        let raw = (self.raw.get() >> Self::PROMO_SHIFT) & Self::FLAG_MASK;
        raw != 0
    }

    #[must_use]
    pub const fn raw(self) -> u16 {
        self.raw.get() ^ Self::PRESENT_BIT
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from_sq(), self.to_sq())?;
        if self.is_promo() {
            write!(f, "q")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MoveStrError {
    NonAsciiString,
    TooShort,
    TooLong,
    InvalidFromSquare(SquareStrError),
    InvalidToSquare(SquareStrError),
    InvalidPromoPiece,
}

impl FromStr for Move {
    type Err = MoveStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(MoveStrError::NonAsciiString);
        }

        if s.len() < 4 {
            return Err(MoveStrError::TooShort);
        }

        if s.len() > 5 {
            return Err(MoveStrError::TooLong);
        }

        let from_sq = s[0..2]
            .parse::<Square>()
            .map_err(MoveStrError::InvalidFromSquare)?;
        let to_sq = s[2..4]
            .parse::<Square>()
            .map_err(MoveStrError::InvalidToSquare)?;

        if s.len() == 5 {
            if s.as_bytes()[4] != b'q' {
                return Err(MoveStrError::InvalidPromoPiece);
            }

            Ok(Move::new_promo(from_sq, to_sq))
        } else {
            Ok(Move::new(from_sq, to_sq))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Move;
    use crate::core::Square;

    #[test]
    fn standard() {
        let mv = Move::new(Square::E2, Square::E3);
        assert_eq!(mv.from_sq(), Square::E2);
        assert_eq!(mv.to_sq(), Square::E3);
        assert!(!mv.is_promo());
    }

    #[test]
    fn promo() {
        let mv = Move::new_promo(Square::D7, Square::D8);
        assert_eq!(mv.from_sq(), Square::D7);
        assert_eq!(mv.to_sq(), Square::D8);
        assert!(mv.is_promo());
    }

    #[test]
    fn from_raw() {
        assert!(Move::from_raw(0b1110_0000_0000_0000).is_none());
        assert_eq!(
            Move::from_raw(0b0000_0101_0000_1100).unwrap(),
            Move::new(Square::E2, Square::E3)
        );
        assert_eq!(
            Move::from_raw(0b0001_1110_1111_0011).unwrap(),
            Move::new_promo(Square::D7, Square::D8)
        );
    }
}
