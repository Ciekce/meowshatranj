use crate::attacks::*;
use crate::bitboard::Bitboard;
use crate::core::*;
use crate::keys::{psq_key, stm_key};
use crate::movegen::{MoveList, generate_all};
use crate::rays::{ray_between, ray_past};
use crate::shatranjmove::Move;
use std::io::Write;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FenError {
    TooFewParts,
    TooManyParts,
    TooFewRanks,
    TooManyRanks,
    TooFewFiles(usize),
    TooManyFiles(usize),
    InvalidPiece(char),
    MissingWhiteKing,
    MultipleWhiteKings,
    MissingBlackKing,
    MultipleBlackKings,
    TooManyPieces,
    InvalidStm,
    InvalidHalfmove,
    HalfmoveTooHigh,
    InvalidFullmove,
    OpponentInCheck,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DecisiveType {
    Mate,
    BareKing,
}

// Note: threefold repetition is *not* handled
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DrawType {
    SeventyMoveRule,
    InsufficientMaterial,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameOutcome {
    Win(DecisiveType),
    Loss(DecisiveType),
    Draw(DrawType),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Position {
    mailbox: [Option<Piece>; Square::COUNT],
    colors: [Bitboard; Color::COUNT],
    pieces: [Bitboard; PieceType::COUNT],
    kings: [Square; Color::COUNT],
    stm: Color,
    checkers: Bitboard,
    pinned: Bitboard,
    threats: Bitboard,
    key: u64,
    halfmove: u16,
    fullmove: u32,
}

impl Position {
    #[must_use]
    fn empty() -> Self {
        Self {
            mailbox: [None; _],
            colors: [Bitboard::EMPTY; _],
            pieces: [Bitboard::EMPTY; _],
            kings: [Square::A1; _],
            stm: Color::White,
            checkers: Bitboard::EMPTY,
            pinned: Bitboard::EMPTY,
            threats: Bitboard::EMPTY,
            key: 0,
            halfmove: 0,
            fullmove: 1,
        }
    }

    #[must_use]
    pub fn startpos() -> Self {
        let mut pos = Self::empty();

        pos.colors[Color::White] = Bitboard::from_raw(0x000000000000FFFF);
        pos.colors[Color::Black] = Bitboard::from_raw(0xFFFF000000000000);

        pos.pieces[PieceType::Pawn] = Bitboard::from_raw(0x00FF00000000FF00);
        pos.pieces[PieceType::Alfil] = Bitboard::from_raw(0x2400000000000024);
        pos.pieces[PieceType::Ferz] = Bitboard::from_raw(0x1000000000000010);
        pos.pieces[PieceType::Knight] = Bitboard::from_raw(0x4200000000000042);
        pos.pieces[PieceType::Rook] = Bitboard::from_raw(0x8100000000000081);
        pos.pieces[PieceType::King] = Bitboard::from_raw(0x0800000000000008);

        pos.regen();

        pos
    }

    pub fn from_fen_parts(parts: &[&str]) -> Result<Self, FenError> {
        if parts.len() < 2 {
            return Err(FenError::TooFewParts);
        }

        if parts.len() > 6 {
            return Err(FenError::TooManyParts);
        }

        let ranks: Vec<&str> = parts[0].split('/').collect();

        if ranks.len() < 8 {
            return Err(FenError::TooFewRanks);
        }

        if ranks.len() > 8 {
            return Err(FenError::TooManyRanks);
        }

        let mut pos = Self::empty();

        let mut set_piece = |sq: Square, piece: Piece| {
            pos.colors[piece.color()].set_sq(sq);
            pos.pieces[piece.piece_type()].set_sq(sq);
        };

        for (rank_idx, &rank) in ranks.iter().enumerate() {
            let mut file_idx = 0;

            for c in rank.chars() {
                if file_idx >= 8 {
                    return Err(FenError::TooManyFiles(rank_idx));
                }

                if c.is_ascii_digit() {
                    let empty_files = c.to_digit(10).unwrap() as usize;
                    file_idx += empty_files;
                    continue;
                }

                if let Some(piece) = Piece::from_char(c) {
                    let sq = Square::from_file_rank(file_idx, 7 - rank_idx).unwrap();
                    set_piece(sq, piece);
                } else {
                    return Err(FenError::InvalidPiece(c));
                }

                file_idx += 1;
            }

            // last character was a digit
            if file_idx > 8 {
                return Err(FenError::TooManyFiles(rank_idx));
            }

            if file_idx < 8 {
                return Err(FenError::TooFewFiles(rank_idx));
            }
        }

        match pos.piece_bb(Piece::WhiteKing).popcount() {
            0 => return Err(FenError::MissingWhiteKing),
            1 => {}
            _ => return Err(FenError::MultipleWhiteKings),
        }

        match pos.piece_bb(Piece::BlackKing).popcount() {
            0 => return Err(FenError::MissingBlackKing),
            1 => {}
            _ => return Err(FenError::MultipleBlackKings),
        }

        if pos.occ().popcount() > 32 {
            return Err(FenError::TooManyPieces);
        }

        match parts[1] {
            "w" => pos.stm = Color::White,
            "b" => pos.stm = Color::Black,
            _ => return Err(FenError::InvalidStm),
        }

        if parts.len() >= 5 {
            if let Ok(halfmove) = parts[4].parse() {
                if halfmove > 140 {
                    return Err(FenError::HalfmoveTooHigh);
                }
                pos.halfmove = halfmove;
            } else {
                return Err(FenError::InvalidHalfmove);
            }
        }

        if parts.len() >= 6 {
            if let Ok(fullmove) = parts[5].parse() {
                pos.fullmove = fullmove;
                pos.fullmove = pos.fullmove.max(1);
            } else {
                return Err(FenError::InvalidFullmove);
            }
        }

        pos.regen();

        if pos.is_attacked(pos.king_sq(pos.stm.flip()), pos.stm) {
            return Err(FenError::OpponentInCheck);
        }

        Ok(pos)
    }

    #[must_use]
    pub fn stm(&self) -> Color {
        self.stm
    }

    #[must_use]
    pub fn piece_on(&self, sq: Square) -> Option<Piece> {
        self.mailbox[sq]
    }

    #[must_use]
    pub fn pt_bb(&self, pt: PieceType) -> Bitboard {
        self.pieces[pt]
    }

    #[must_use]
    pub fn color_bb(&self, color: Color) -> Bitboard {
        self.colors[color]
    }

    #[must_use]
    pub fn piece_bb(&self, piece: Piece) -> Bitboard {
        self.pieces[piece.piece_type()] & self.colors[piece.color()]
    }

    #[must_use]
    pub fn occ(&self) -> Bitboard {
        self.colors[0] | self.colors[1]
    }

    #[must_use]
    pub fn king_sq(&self, color: Color) -> Square {
        self.kings[color]
    }

    #[must_use]
    pub fn in_check(&self) -> bool {
        !self.checkers.is_empty()
    }

    #[must_use]
    pub fn checkers(&self) -> Bitboard {
        self.checkers
    }

    #[must_use]
    pub fn pinned(&self) -> Bitboard {
        self.pinned
    }

    #[must_use]
    pub fn threats(&self) -> Bitboard {
        self.threats
    }

    #[must_use]
    pub fn key(&self) -> u64 {
        self.key
    }

    #[must_use]
    pub fn halfmove(&self) -> u16 {
        self.halfmove
    }

    #[must_use]
    pub fn fullmove(&self) -> u32 {
        self.fullmove
    }

    #[must_use]
    pub fn apply_move(&self, mv: Move) -> Self {
        debug_assert!(self.is_legal(mv));

        let mut new_pos = *self;

        let from = mv.from_sq();
        let to = mv.to_sq();

        let moving = self.piece_on(from).unwrap();
        let target = if mv.is_promo() {
            PieceType::Ferz.with_color(self.stm)
        } else {
            moving
        };

        new_pos.colors[self.stm] ^= from.bb() ^ to.bb();

        new_pos.pieces[moving.piece_type()] ^= from.bb();
        new_pos.pieces[target.piece_type()] ^= to.bb();

        new_pos.key ^= psq_key(moving, from);
        new_pos.key ^= psq_key(target, to);

        let captured = self.piece_on(to);
        if let Some(captured) = captured {
            new_pos.colors[self.stm.flip()] ^= to.bb();
            new_pos.pieces[captured.piece_type()] ^= to.bb();
            new_pos.key ^= psq_key(captured, to);
        }

        new_pos.mailbox[from] = None;
        new_pos.mailbox[to] = Some(target);

        if moving.piece_type() == PieceType::King {
            new_pos.kings[self.stm] = to;
        }

        new_pos.stm = new_pos.stm.flip();
        new_pos.key ^= stm_key();

        if captured.is_some() || moving.piece_type() == PieceType::Pawn {
            new_pos.halfmove = 0;
        } else {
            new_pos.halfmove += 1;
        }

        if self.stm == Color::Black {
            new_pos.fullmove += 1;
        }

        new_pos.update_attacks();

        new_pos
    }

    #[must_use]
    pub fn apply_nullmove(&self) -> Self {
        assert!(!self.in_check());

        let mut new_pos = *self;

        new_pos.stm = new_pos.stm.flip();
        new_pos.key ^= stm_key();

        new_pos.halfmove += 1;

        if self.stm == Color::Black {
            new_pos.fullmove += 1;
        }

        new_pos.update_attacks();

        new_pos
    }

    #[must_use]
    pub fn is_legal(&self, mv: Move) -> bool {
        let from = mv.from_sq();
        let to = mv.to_sq();

        if from == to {
            return false;
        }

        let king_sq = self.king_sq(self.stm);

        let moving = if let Some(moving) = self.piece_on(from) {
            moving
        } else {
            return false;
        };

        let captured = self.piece_on(to);

        if moving.color() != self.stm {
            return false;
        }

        if self.in_check() && moving.piece_type() != PieceType::King {
            // multiple checks may only be evaded with a king move
            if self.checkers.popcount() > 1 {
                return false;
            }

            let checker = self.checkers.lsb().unwrap();
            let check_mask = ray_between(king_sq, checker) | checker.bb();

            if !check_mask.has_sq(to) {
                return false;
            }
        }

        if self.pinned.has_sq(from) && !ray_past(king_sq, from).has_sq(to) {
            return false;
        }

        if let Some(captured) = captured
            && captured.color() == self.stm
        {
            // we do not test for king captures, because positions where they are
            //   pseudolegal cannot be constructed without making an illegal move
            return false;
        }

        if moving.piece_type() == PieceType::Pawn {
            let from_rank = from.rank();
            let to_rank = to.rank();

            if match self.stm {
                Color::White => to_rank <= from_rank,
                Color::Black => to_rank >= from_rank,
            } {
                return false;
            }

            if from.file() != to.file() {
                let captures = pawn_attacks(from, self.stm) & self.color_bb(self.stm.flip());
                if !captures.has_sq(to) {
                    return false;
                }
            } else {
                if from_rank.abs_diff(to_rank) != 1 {
                    return false;
                }

                if captured.is_some() {
                    return false;
                }
            }

            let promo_rank = relative_rank(self.stm, RANK_8);
            if mv.is_promo() != (to.rank() == promo_rank) {
                return false;
            }
        } else {
            if mv.is_promo() {
                return false;
            }

            let moves = match moving.piece_type() {
                PieceType::Alfil => alfil_attacks(from),
                PieceType::Ferz => ferz_attacks(from),
                PieceType::Knight => knight_attacks(from),
                PieceType::Rook => rook_attacks(from, self.occ()),
                PieceType::King => king_attacks(from) & !self.threats,
                _ => unreachable!(),
            };

            if !moves.has_sq(to) {
                return false;
            }
        }

        true
    }

    #[must_use]
    pub fn is_attacked(&self, sq: Square, attacker: Color) -> bool {
        let pawns = self.piece_bb(PieceType::Pawn.with_color(attacker));
        if pawn_attacks(sq, attacker.flip()).intersects(pawns) {
            return true;
        }

        let alfils = self.piece_bb(PieceType::Alfil.with_color(attacker));
        if alfil_attacks(sq).intersects(alfils) {
            return true;
        }

        let ferzes = self.piece_bb(PieceType::Ferz.with_color(attacker));
        if ferz_attacks(sq).intersects(ferzes) {
            return true;
        }

        let knights = self.piece_bb(PieceType::Knight.with_color(attacker));
        if knight_attacks(sq).intersects(knights) {
            return true;
        }

        let kings = self.piece_bb(PieceType::King.with_color(attacker));
        if king_attacks(sq).intersects(kings) {
            return true;
        }

        let rooks = self.piece_bb(PieceType::Rook.with_color(attacker));
        if rook_attacks(sq, self.occ()).intersects(rooks) {
            return true;
        }

        false
    }

    #[must_use]
    pub fn non_slider_attackers_to(&self, sq: Square, attacker: Color) -> Bitboard {
        let mut attackers = Bitboard::EMPTY;

        let pawns = self.piece_bb(PieceType::Pawn.with_color(attacker));
        attackers |= pawns & pawn_attacks(sq, attacker.flip());

        let alfils = self.piece_bb(PieceType::Alfil.with_color(attacker));
        attackers |= alfils & alfil_attacks(sq);

        let ferzes = self.piece_bb(PieceType::Ferz.with_color(attacker));
        attackers |= ferzes & ferz_attacks(sq);

        let knights = self.piece_bb(PieceType::Knight.with_color(attacker));
        attackers |= knights & knight_attacks(sq);

        let kings = self.piece_bb(PieceType::King.with_color(attacker));
        attackers |= kings & king_attacks(sq);

        attackers
    }

    #[must_use]
    pub fn all_attackers_to(&self, sq: Square, attacker: Color) -> Bitboard {
        let mut attackers = self.non_slider_attackers_to(sq, attacker);

        let rooks = self.piece_bb(PieceType::Rook.with_color(attacker));
        attackers |= rooks & rook_attacks(sq, self.occ());

        attackers
    }

    fn update_attacks(&mut self) {
        let king_sq = self.king_sq(self.stm);

        self.checkers = self.non_slider_attackers_to(king_sq, self.stm.flip());
        self.pinned = Bitboard::EMPTY;

        let our_occ = self.color_bb(self.stm);
        let their_occ = self.color_bb(self.stm.flip());

        let their_rooks = self.piece_bb(PieceType::Rook.with_color(self.stm.flip()));
        let potential_attackers = their_rooks & rook_attacks(king_sq, their_occ);

        for potential_attacker in potential_attackers {
            let maybe_pinned = our_occ & ray_between(potential_attacker, king_sq);
            if maybe_pinned.is_empty() {
                self.checkers.set_sq(potential_attacker);
            } else if maybe_pinned.popcount() == 1 {
                self.pinned |= maybe_pinned;
            }
        }

        self.threats = Bitboard::EMPTY;

        let them = self.stm.flip();
        let kingless_occ = self.occ() ^ king_sq.bb();

        let their_pawns = self.piece_bb(PieceType::Pawn.with_color(them));
        self.threats |= pawn_attacks_setwise(their_pawns, them);

        let their_alfils = self.piece_bb(PieceType::Alfil.with_color(them));
        for alfil in their_alfils {
            self.threats |= alfil_attacks(alfil);
        }

        let their_ferzes = self.piece_bb(PieceType::Ferz.with_color(them));
        for ferz in their_ferzes {
            self.threats |= ferz_attacks(ferz);
        }

        let their_knights = self.piece_bb(PieceType::Knight.with_color(them));
        for knight in their_knights {
            self.threats |= knight_attacks(knight);
        }

        let their_rooks = self.piece_bb(PieceType::Rook.with_color(them));
        for rook in their_rooks {
            self.threats |= rook_attacks(rook, kingless_occ);
        }

        let their_king = self.king_sq(them);
        self.threats |= king_attacks(their_king);
    }

    fn regen(&mut self) {
        assert_eq!(
            self.colors[Color::White] | self.colors[Color::Black],
            self.pieces[PieceType::Pawn]
                | self.pieces[PieceType::Alfil]
                | self.pieces[PieceType::Ferz]
                | self.pieces[PieceType::Knight]
                | self.pieces[PieceType::Rook]
                | self.pieces[PieceType::King]
        );

        self.mailbox.fill(None);
        self.key = 0;

        let mut seen_king = [false; Color::COUNT];

        for piece in Piece::all() {
            for sq in self.piece_bb(piece) {
                assert!(self.piece_on(sq).is_none());

                self.mailbox[sq] = Some(piece);
                self.key ^= psq_key(piece, sq);

                if piece.piece_type() == PieceType::King {
                    assert!(!seen_king[piece.color()]);
                    seen_king[piece.color()] = true;
                    self.kings[piece.color()] = sq;
                }
            }
        }

        assert!(seen_king[0] && seen_king[1]);

        if self.stm == Color::Black {
            self.key ^= stm_key();
        }

        self.update_attacks();
    }

    #[must_use]
    pub fn is_insufficient_material(&self) -> bool {
        if self.piece_bb(Piece::WhiteKing) == self.color_bb(Color::White)
            && self.piece_bb(Piece::BlackKing) == self.color_bb(Color::Black)
        {
            return true;
        }

        //TODO

        false
    }

    #[must_use]
    pub fn has_bare_king(&self, color: Color) -> bool {
        let our_king = self.piece_bb(PieceType::King.with_color(color));
        let our_pieces = self.color_bb(color);

        if our_pieces != our_king {
            return false;
        }

        let their_king = self.piece_bb(PieceType::King.with_color(color));
        let their_non_kings = self.color_bb(color) ^ their_king;

        if their_non_kings.is_empty() {
            return false;
        }

        if their_non_kings.popcount() > 1 {
            return true;
        }

        let our_king_attacks = king_attacks(self.king_sq(color));
        let their_king_attacks = king_attacks(self.king_sq(color.flip()));

        let our_king_attacks = our_king_attacks & !their_king_attacks;

        !(their_non_kings & !our_king_attacks).is_empty()
    }

    // **Will** return a mate if one occurs on the 70th move.
    #[must_use]
    pub fn non_mate_outcome(&self) -> Option<GameOutcome> {
        if self.halfmove >= 140 {
            let mut moves = MoveList::new();
            generate_all(&mut moves, self);

            return if moves.is_empty() {
                Some(GameOutcome::Loss(DecisiveType::Mate))
            } else {
                Some(GameOutcome::Draw(DrawType::SeventyMoveRule))
            };
        }

        if self.is_insufficient_material() {
            return Some(GameOutcome::Draw(DrawType::InsufficientMaterial));
        }

        if self.has_bare_king(self.stm) {
            return Some(GameOutcome::Loss(DecisiveType::BareKing));
        }

        if self.has_bare_king(self.stm.flip()) {
            return Some(GameOutcome::Win(DecisiveType::BareKing));
        }

        None
    }

    #[must_use]
    pub fn outcome(&self) -> Option<GameOutcome> {
        if let Some(outcome) = self.non_mate_outcome() {
            return Some(outcome);
        }

        let mut moves = MoveList::new();
        generate_all(&mut moves, self);

        if moves.is_empty() {
            return Some(GameOutcome::Loss(DecisiveType::Mate));
        }

        None
    }

    pub fn fen(&self) -> String {
        let mut buf = Vec::with_capacity(82);

        for rank in (0..8).rev() {
            let mut file = 0;

            while file < 8 {
                let sq = Square::from_file_rank(file, rank).unwrap();

                if let Some(piece) = self.piece_on(sq) {
                    write!(buf, "{}", piece).unwrap();
                } else {
                    let mut empty_squares = 1;

                    while file < 7
                        && self
                            .piece_on(Square::from_file_rank(file + 1, rank).unwrap())
                            .is_none()
                    {
                        empty_squares += 1;
                        file += 1;
                    }

                    write!(buf, "{}", empty_squares).unwrap();
                }

                file += 1;
            }

            if rank > 0 {
                write!(buf, "/").unwrap();
            }
        }

        match self.stm {
            Color::White => write!(buf, " w").unwrap(),
            Color::Black => write!(buf, " b").unwrap(),
        }

        write!(buf, " - -").unwrap();
        write!(buf, " {} {}", self.halfmove, self.fullmove).unwrap();

        String::from_utf8(buf).unwrap()
    }
}

impl FromStr for Position {
    type Err = FenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        Self::from_fen_parts(&parts)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PositionBuilderError {
    MissingKing(Color),
    TooManyKings,
    PieceAlreadyOnSquare,
    HalfmoveTooHigh,
}

#[derive(Clone)]
pub struct PositionBuilder {
    pos: Position,
    has_king: [bool; Color::COUNT],
}

impl PositionBuilder {
    pub fn build(mut self, stm: Color) -> Result<Position, PositionBuilderError> {
        for color in Color::all() {
            if !self.has_king[color] {
                return Err(PositionBuilderError::MissingKing(color));
            }
        }

        self.pos.stm = stm;
        self.pos.regen();

        Ok(self.pos)
    }

    pub fn set_piece(&mut self, piece: Piece, sq: Square) -> Result<(), PositionBuilderError> {
        if piece.piece_type() == PieceType::King {
            if self.has_king[piece.color()] {
                return Err(PositionBuilderError::TooManyKings);
            }
            self.has_king[piece.color()] = true;
        }

        if self.pos.colors[piece.color()].has_sq(sq)
            || self.pos.pieces[piece.piece_type()].has_sq(sq)
        {
            return Err(PositionBuilderError::PieceAlreadyOnSquare);
        }

        self.pos.colors[piece.color()].set_sq(sq);
        self.pos.pieces[piece.piece_type()].set_sq(sq);

        Ok(())
    }

    pub fn set_halfmove(&mut self, halfmove: u16) -> Result<(), PositionBuilderError> {
        if halfmove > 140 {
            return Err(PositionBuilderError::HalfmoveTooHigh);
        }

        self.pos.halfmove = halfmove;

        Ok(())
    }

    pub fn set_fullmove(&mut self, fullmove: u32) {
        self.pos.fullmove = fullmove.max(1);
    }
}

impl Default for PositionBuilder {
    fn default() -> Self {
        Self {
            pos: Position::empty(),
            has_king: [false; _],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bitboard::Bitboard;
    use crate::position::Position;

    #[test]
    fn white_threats() {
        let pos = "3k4/8/8/5N2/8/1P6/8/K1Q1RB2 b - - 0 1"
            .parse::<Position>()
            .unwrap();
        assert_eq!(pos.threats(), Bitboard::from_raw(0x1050_9810_9dd8_1b2e));
    }

    #[test]
    fn black_threats() {
        let pos = "2br1q1k/8/6p1/8/2n5/8/8/4K3 w - - 0 1"
            .parse::<Position>()
            .unwrap();
        assert_eq!(pos.threats(), Bitboard::from_raw(0x74d8_1bb9_0819_0a08));
    }
}
