use crate::core::*;
use std::ops::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Bitboard {
    raw: u64,
}

impl Bitboard {
    pub const EMPTY: Self = Self::from_raw(0);
    pub const ALL: Self = Self::from_raw(0xFFFFFFFFFFFFFFFF);

    pub const RANK_1: Self = Self::from_raw(0x00000000000000FF);
    pub const RANK_2: Self = Self::from_raw(0x000000000000FF00);
    pub const RANK_3: Self = Self::from_raw(0x0000000000FF0000);
    pub const RANK_4: Self = Self::from_raw(0x00000000FF000000);
    pub const RANK_5: Self = Self::from_raw(0x000000FF00000000);
    pub const RANK_6: Self = Self::from_raw(0x0000FF0000000000);
    pub const RANK_7: Self = Self::from_raw(0x00FF000000000000);
    pub const RANK_8: Self = Self::from_raw(0xFF00000000000000);

    pub const FILE_A: Self = Self::from_raw(0x0101010101010101);
    pub const FILE_B: Self = Self::from_raw(0x0202020202020202);
    pub const FILE_C: Self = Self::from_raw(0x0404040404040404);
    pub const FILE_D: Self = Self::from_raw(0x0808080808080808);
    pub const FILE_E: Self = Self::from_raw(0x1010101010101010);
    pub const FILE_F: Self = Self::from_raw(0x2020202020202020);
    pub const FILE_G: Self = Self::from_raw(0x4040404040404040);
    pub const FILE_H: Self = Self::from_raw(0x8080808080808080);

    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        Self { raw }
    }

    #[must_use]
    pub const fn raw(self) -> u64 {
        self.raw
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.raw == 0
    }

    #[must_use]
    pub const fn has_sq(self, sq: Square) -> bool {
        (self.raw & sq.bb().raw) != 0
    }

    #[must_use]
    pub const fn with_sq(self, sq: Square) -> Self {
        Self {
            raw: self.raw | sq.bb().raw,
        }
    }

    #[must_use]
    pub const fn without_sq(self, sq: Square) -> Self {
        Self {
            raw: self.raw & !sq.bb().raw,
        }
    }

    #[must_use]
    pub const fn with_sq_toggled(self, sq: Square) -> Self {
        Self {
            raw: self.raw ^ sq.bb().raw,
        }
    }

    pub const fn set_sq(&mut self, sq: Square) {
        self.raw |= sq.bb().raw;
    }

    pub const fn clear_sq(&mut self, sq: Square) {
        self.raw &= !sq.bb().raw;
    }

    pub const fn toggle_sq(&mut self, sq: Square) {
        self.raw ^= sq.bb().raw;
    }

    #[must_use]
    pub const fn cmpl(self) -> Self {
        Self { raw: !self.raw }
    }

    #[must_use]
    pub const fn and(self, other: Self) -> Self {
        Self {
            raw: self.raw & other.raw,
        }
    }

    #[must_use]
    pub const fn or(self, other: Self) -> Self {
        Self {
            raw: self.raw | other.raw,
        }
    }

    #[must_use]
    pub const fn xor(self, other: Self) -> Self {
        Self {
            raw: self.raw ^ other.raw,
        }
    }

    #[must_use]
    pub const fn shr(self, count: usize) -> Self {
        Self {
            raw: self.raw >> count,
        }
    }

    #[must_use]
    pub const fn shl(self, count: usize) -> Self {
        Self {
            raw: self.raw << count,
        }
    }

    #[must_use]
    pub const fn lsb(self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            Some(Square::from_raw(self.raw.trailing_zeros() as u8).unwrap())
        }
    }

    pub fn pop_lsb(&mut self) -> Option<Square> {
        let sq = self.lsb()?;
        self.raw &= self.raw - 1;
        Some(sq)
    }

    #[must_use]
    pub const fn popcount(self) -> u32 {
        self.raw.count_ones()
    }

    #[must_use]
    pub const fn shift_up(self) -> Self {
        self.shl(8)
    }

    #[must_use]
    pub const fn shift_down(self) -> Self {
        self.shr(8)
    }

    #[must_use]
    pub const fn shift_left(self) -> Self {
        self.and(Self::FILE_A.cmpl()).shr(1)
    }

    #[must_use]
    pub const fn shift_right(self) -> Self {
        self.and(Self::FILE_H.cmpl()).shl(1)
    }

    #[must_use]
    pub const fn shift_up_left(self) -> Self {
        self.and(Self::FILE_A.cmpl()).shl(7)
    }

    #[must_use]
    pub const fn shift_up_right(self) -> Self {
        self.and(Self::FILE_H.cmpl()).shl(9)
    }

    #[must_use]
    pub const fn shift_down_left(self) -> Self {
        self.and(Self::FILE_A.cmpl()).shr(9)
    }

    #[must_use]
    pub const fn shift_down_right(self) -> Self {
        self.and(Self::FILE_H.cmpl()).shr(7)
    }

    #[must_use]
    pub const fn shift_up_relative(self, color: Color) -> Self {
        match color {
            Color::White => self.shift_up(),
            Color::Black => self.shift_down(),
        }
    }

    #[must_use]
    pub const fn shift_down_relative(self, color: Color) -> Self {
        match color {
            Color::White => self.shift_down(),
            Color::Black => self.shift_up(),
        }
    }

    #[must_use]
    pub const fn shift_up_left_relative(self, color: Color) -> Self {
        match color {
            Color::White => self.shift_up_left(),
            Color::Black => self.shift_down_left(),
        }
    }

    #[must_use]
    pub const fn shift_up_right_relative(self, color: Color) -> Self {
        match color {
            Color::White => self.shift_up_right(),
            Color::Black => self.shift_down_right(),
        }
    }

    #[must_use]
    pub const fn shift_down_left_relative(self, color: Color) -> Self {
        match color {
            Color::White => self.shift_down_left(),
            Color::Black => self.shift_up_left(),
        }
    }

    #[must_use]
    pub const fn shift_down_right_relative(self, color: Color) -> Self {
        match color {
            Color::White => self.shift_down_right(),
            Color::Black => self.shift_up_right(),
        }
    }

    #[must_use]
    pub const fn shift(self, dir: Direction) -> Self {
        match dir {
            Direction::Up => self.shift_up(),
            Direction::Down => self.shift_down(),
            Direction::Left => self.shift_left(),
            Direction::Right => self.shift_right(),
        }
    }

    #[must_use]
    pub const fn rank_bb(rank: usize) -> Bitboard {
        assert!(rank < 8);
        Self::RANK_1.shl(rank * 8)
    }

    #[must_use]
    pub const fn file_bb(file: usize) -> Bitboard {
        assert!(file < 8);
        Self::FILE_A.shl(file)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.cmpl()
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

impl Shr<usize> for Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: usize) -> Self::Output {
        self.shr(rhs)
    }
}

impl Shl<usize> for Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: usize) -> Self::Output {
        self.shl(rhs)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

impl ShrAssign<usize> for Bitboard {
    fn shr_assign(&mut self, rhs: usize) {
        *self = *self >> rhs;
    }
}

impl ShlAssign<usize> for Bitboard {
    fn shl_assign(&mut self, rhs: usize) {
        *self = *self << rhs;
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = Biterator;

    fn into_iter(self) -> Self::IntoIter {
        Biterator { board: self }
    }
}

pub struct Biterator {
    board: Bitboard,
}

impl Iterator for Biterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        self.board.pop_lsb()
    }
}
