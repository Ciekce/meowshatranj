use crate::bitboard::Bitboard;
use std::fmt::{Display, Formatter, Write};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub const COUNT: usize = 2;

    #[must_use]
    pub const unsafe fn from_raw_unchecked(raw: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Self>(raw) }
    }

    #[must_use]
    pub const fn from_raw(raw: u8) -> Option<Self> {
        if (raw as usize) < Self::COUNT {
            // SAFETY: we just bounds checked the value
            Some(unsafe { Self::from_raw_unchecked(raw) })
        } else {
            None
        }
    }

    #[must_use]
    pub const fn raw(self) -> u8 {
        self as u8
    }

    #[must_use]
    pub const fn idx(self) -> usize {
        self as usize
    }

    #[must_use]
    pub const fn flip(self) -> Self {
        Self::from_raw(self as u8 ^ 0x1).unwrap()
    }

    #[must_use]
    pub fn all() -> ColorIterator {
        ColorIterator { raw: 0 }
    }
}

impl<T> Index<Color> for [T; Color::COUNT] {
    type Output = T;

    fn index(&self, index: Color) -> &Self::Output {
        // SAFETY: this is only implemented for arrays of the correct length
        unsafe { self.get_unchecked(index.idx()) }
    }
}

impl<T> IndexMut<Color> for [T; Color::COUNT] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        // SAFETY: this is only implemented for arrays of the correct length
        unsafe { self.get_unchecked_mut(index.idx()) }
    }
}

pub struct ColorIterator {
    raw: u8,
}

impl Iterator for ColorIterator {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        let c = Color::from_raw(self.raw);
        self.raw += 1;
        c
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum PieceType {
    Pawn,
    Alfil,
    Ferz,
    Knight,
    Rook,
    King,
}

impl PieceType {
    pub const COUNT: usize = 6;

    #[must_use]
    pub const unsafe fn from_raw_unchecked(raw: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Self>(raw) }
    }

    #[must_use]
    pub const fn from_raw(raw: u8) -> Option<Self> {
        if (raw as usize) < Self::COUNT {
            // SAFETY: we just bounds checked the value
            Some(unsafe { Self::from_raw_unchecked(raw) })
        } else {
            None
        }
    }

    #[must_use]
    pub const fn raw(self) -> u8 {
        self as u8
    }

    #[must_use]
    pub const fn idx(self) -> usize {
        self as usize
    }

    #[must_use]
    pub const fn with_color(self, color: Color) -> Piece {
        Piece::from_raw((self.raw() << 1) | color.raw()).unwrap()
    }

    #[must_use]
    pub fn all() -> PieceTypeIterator {
        PieceTypeIterator { raw: 0 }
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = ['p', 'b', 'q', 'n', 'r', 'k'][self.idx()];
        write!(f, "{}", c)
    }
}

impl<T> Index<PieceType> for [T; PieceType::COUNT] {
    type Output = T;

    fn index(&self, index: PieceType) -> &Self::Output {
        // SAFETY: this is only implemented for arrays of the correct length
        unsafe { self.get_unchecked(index.idx()) }
    }
}

impl<T> IndexMut<PieceType> for [T; PieceType::COUNT] {
    fn index_mut(&mut self, index: PieceType) -> &mut Self::Output {
        // SAFETY: this is only implemented for arrays of the correct length
        unsafe { self.get_unchecked_mut(index.idx()) }
    }
}

pub struct PieceTypeIterator {
    raw: u8,
}

impl Iterator for PieceTypeIterator {
    type Item = PieceType;

    fn next(&mut self) -> Option<Self::Item> {
        let pt = PieceType::from_raw(self.raw);
        self.raw += 1;
        pt
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteAlfil,
    BlackAlfil,
    WhiteFerz,
    BlackFerz,
    WhiteKnight,
    BlackKnight,
    WhiteRook,
    BlackRook,
    WhiteKing,
    BlackKing,
}

impl Piece {
    pub const COUNT: usize = 12;

    #[must_use]
    pub const unsafe fn from_raw_unchecked(raw: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Self>(raw) }
    }

    #[must_use]
    pub const fn from_raw(raw: u8) -> Option<Self> {
        if (raw as usize) < Self::COUNT {
            // SAFETY: we just bounds checked the value
            Some(unsafe { Self::from_raw_unchecked(raw) })
        } else {
            None
        }
    }

    #[must_use]
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            'P' => Some(Self::WhitePawn),
            'p' => Some(Self::BlackPawn),
            'B' => Some(Self::WhiteAlfil),
            'b' => Some(Self::BlackAlfil),
            'Q' => Some(Self::WhiteFerz),
            'q' => Some(Self::BlackFerz),
            'N' => Some(Self::WhiteKnight),
            'n' => Some(Self::BlackKnight),
            'R' => Some(Self::WhiteRook),
            'r' => Some(Self::BlackRook),
            'K' => Some(Self::WhiteKing),
            'k' => Some(Self::BlackKing),
            _ => None,
        }
    }

    #[must_use]
    pub const fn raw(self) -> u8 {
        self as u8
    }

    #[must_use]
    pub const fn idx(self) -> usize {
        self as usize
    }

    #[must_use]
    pub const fn color(self) -> Color {
        Color::from_raw(self.raw() & 0x1).unwrap()
    }

    #[must_use]
    pub const fn piece_type(self) -> PieceType {
        PieceType::from_raw(self.raw() >> 1).unwrap()
    }

    #[must_use]
    pub fn all() -> PieceIterator {
        PieceIterator { raw: 0 }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = ['P', 'p', 'B', 'b', 'Q', 'q', 'N', 'n', 'R', 'r', 'K', 'k'][self.idx()];
        write!(f, "{}", c)
    }
}

impl<T> Index<Piece> for [T; Piece::COUNT] {
    type Output = T;

    fn index(&self, index: Piece) -> &Self::Output {
        // SAFETY: this is only implemented for arrays of the correct length
        unsafe { self.get_unchecked(index.idx()) }
    }
}

impl<T> IndexMut<Piece> for [T; Piece::COUNT] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        // SAFETY: this is only implemented for arrays of the correct length
        unsafe { self.get_unchecked_mut(index.idx()) }
    }
}

pub struct PieceIterator {
    raw: u8,
}

impl Iterator for PieceIterator {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        let piece = Piece::from_raw(self.raw);
        self.raw += 1;
        piece
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const COUNT: usize = 4;

    #[must_use]
    pub const unsafe fn from_raw_unchecked(raw: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Self>(raw) }
    }

    #[must_use]
    pub const fn from_raw(raw: u8) -> Option<Self> {
        if (raw as usize) < Self::COUNT {
            // SAFETY: we just bounds checked the value
            Some(unsafe { Self::from_raw_unchecked(raw) })
        } else {
            None
        }
    }

    #[must_use]
    pub const fn raw(self) -> u8 {
        self as u8
    }

    #[must_use]
    pub const fn idx(self) -> usize {
        self as usize
    }

    #[must_use]
    pub const fn offset(self) -> i8 {
        [8, -8, -1, 1][self.idx()]
    }

    #[must_use]
    pub fn all() -> DirectionIterator {
        DirectionIterator { raw: 0 }
    }
}

impl<T> Index<Direction> for [T; Direction::COUNT] {
    type Output = T;

    fn index(&self, index: Direction) -> &Self::Output {
        // SAFETY: this is only implemented for arrays of the correct length
        unsafe { self.get_unchecked(index.idx()) }
    }
}

impl<T> IndexMut<Direction> for [T; Direction::COUNT] {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        // SAFETY: this is only implemented for arrays of the correct length
        unsafe { self.get_unchecked_mut(index.idx()) }
    }
}

pub struct DirectionIterator {
    raw: u8,
}

impl Iterator for DirectionIterator {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let dir = Direction::from_raw(self.raw);
        self.raw += 1;
        dir
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub const COUNT: usize = 64;

    #[must_use]
    pub const unsafe fn from_raw_unchecked(raw: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Self>(raw) }
    }

    #[must_use]
    pub const fn from_raw(raw: u8) -> Option<Self> {
        if (raw as usize) < Self::COUNT {
            // SAFETY: we just bounds checked the value
            Some(unsafe { Self::from_raw_unchecked(raw) })
        } else {
            None
        }
    }

    #[must_use]
    pub const fn from_file_rank(file: usize, rank: usize) -> Option<Self> {
        if file >= 8 || rank >= 8 {
            None
        } else {
            Some(Self::from_raw((rank as u8 * 8) + file as u8).unwrap())
        }
    }

    #[must_use]
    pub const fn raw(self) -> u8 {
        self as u8
    }

    #[must_use]
    pub const fn idx(self) -> usize {
        self as usize
    }

    #[must_use]
    pub const fn rank(self) -> usize {
        self.raw() as usize / 8
    }

    #[must_use]
    pub const fn file(self) -> usize {
        self.raw() as usize % 8
    }

    #[must_use]
    pub const fn flip_file(self) -> Self {
        let raw = self.raw() ^ 0b000111;
        // SAFETY: any bitpattern this can produce is a valid square index
        unsafe { Self::from_raw_unchecked(raw) }
    }

    #[must_use]
    pub const fn flip_rank(self) -> Self {
        let raw = self.raw() ^ 0b111000;
        // SAFETY: any bitpattern this can produce is a valid square index
        unsafe { Self::from_raw_unchecked(raw) }
    }

    #[must_use]
    pub const fn bb(self) -> Bitboard {
        Bitboard::from_raw(1 << self.idx())
    }

    #[must_use]
    pub const unsafe fn offset_unchecked(self, offset: i32) -> Self {
        let raw = self.raw() as i32 + offset;
        unsafe { Self::from_raw_unchecked(raw as u8) }
    }

    #[must_use]
    pub const fn offset(self, offset: i32) -> Option<Self> {
        let raw = self.raw() as i32;
        if offset == 0
            || (offset < 0 && raw >= offset)
            || (offset > 0 && raw + offset < Self::COUNT as i32)
        {
            // SAFETY: we just bounds checked the value
            Some(unsafe { self.offset_unchecked(offset) })
        } else {
            None
        }
    }

    #[must_use]
    pub const fn shift(self, dir: Direction) -> Option<Self> {
        let shifted = self as i8 + dir.offset();
        if shifted >= 0 && shifted < Self::COUNT as i8 {
            Some(Self::from_raw(shifted as u8).unwrap())
        } else {
            None
        }
    }

    #[must_use]
    pub const fn shift_checked(self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::Left if self.file() == 0 => None,
            Direction::Right if self.file() == 7 => None,
            _ => self.shift(dir),
        }
    }

    #[must_use]
    pub fn all() -> SquareIterator {
        SquareIterator { raw: 0 }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char((b'a' + self.file() as u8) as char)?;
        f.write_char((b'1' + self.rank() as u8) as char)
    }
}

impl<T> Index<Square> for [T; Square::COUNT] {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        // SAFETY: this is only implemented for arrays of the correct length
        unsafe { self.get_unchecked(index.idx()) }
    }
}

impl<T> IndexMut<Square> for [T; Square::COUNT] {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        // SAFETY: this is only implemented for arrays of the correct length
        unsafe { self.get_unchecked_mut(index.idx()) }
    }
}

pub struct SquareIterator {
    raw: u8,
}

impl Iterator for SquareIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let sq = Square::from_raw(self.raw);
        self.raw += 1;
        sq
    }
}

pub const RANK_1: usize = 0;
pub const RANK_2: usize = 1;
pub const RANK_3: usize = 2;
pub const RANK_4: usize = 3;
pub const RANK_5: usize = 4;
pub const RANK_6: usize = 5;
pub const RANK_7: usize = 6;
pub const RANK_8: usize = 7;

pub const FILE_A: usize = 0;
pub const FILE_B: usize = 1;
pub const FILE_C: usize = 2;
pub const FILE_D: usize = 3;
pub const FILE_E: usize = 4;
pub const FILE_F: usize = 5;
pub const FILE_G: usize = 6;
pub const FILE_H: usize = 7;

#[must_use]
pub const fn relative_rank(color: Color, rank: usize) -> usize {
    assert!(rank < 8);
    match color {
        Color::White => rank,
        Color::Black => rank ^ 7,
    }
}
