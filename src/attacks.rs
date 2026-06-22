mod hq;

use crate::bitboard::Bitboard;
use crate::core::{Color, Piece, PieceType, Square};

const PAWN_ATTACKS: [[Bitboard; Square::COUNT]; Color::COUNT] = {
    let mut attacks = [[Bitboard::EMPTY; _]; _];

    let mut sq_idx = 0;
    while sq_idx < Square::COUNT {
        let sq = Square::from_raw(sq_idx as u8).unwrap();
        let bit = sq.bb();

        let white_bb = bit.shift_up_left();
        let white_bb = white_bb.or(bit.shift_up_right());

        let black_bb = bit.shift_down_left();
        let black_bb = black_bb.or(bit.shift_down_right());

        attacks[Color::White.idx()][sq_idx] = white_bb;
        attacks[Color::Black.idx()][sq_idx] = black_bb;

        sq_idx += 1;
    }

    attacks
};

const ALFIL_ATTACKS: [Bitboard; Square::COUNT] = {
    let mut attacks = [Bitboard::EMPTY; _];

    let mut sq_idx = 0;
    while sq_idx < Square::COUNT {
        let sq = Square::from_raw(sq_idx as u8).unwrap();
        let bit = sq.bb();

        let bb = bit.shift_up_left().shift_up_left();
        let bb = bb.or(bit.shift_up_right().shift_up_right());
        let bb = bb.or(bit.shift_down_left().shift_down_left());
        let bb = bb.or(bit.shift_down_right().shift_down_right());

        attacks[sq_idx] = bb;
        sq_idx += 1;
    }

    attacks
};

const FERZ_ATTACKS: [Bitboard; Square::COUNT] = {
    let mut attacks = [Bitboard::EMPTY; _];

    let mut sq_idx = 0;
    while sq_idx < Square::COUNT {
        let sq = Square::from_raw(sq_idx as u8).unwrap();
        let bit = sq.bb();

        let bb = bit.shift_up_left();
        let bb = bb.or(bit.shift_up_right());
        let bb = bb.or(bit.shift_down_left());
        let bb = bb.or(bit.shift_down_right());

        attacks[sq_idx] = bb;
        sq_idx += 1;
    }

    attacks
};

const KNIGHT_ATTACKS: [Bitboard; Square::COUNT] = {
    let mut attacks = [Bitboard::EMPTY; _];

    let mut sq_idx = 0;
    while sq_idx < Square::COUNT {
        let sq = Square::from_raw(sq_idx as u8).unwrap();
        let bit = sq.bb();

        let bb = bit.shift_up().shift_up_left();
        let bb = bb.or(bit.shift_up().shift_up_right());

        let bb = bb.or(bit.shift_down().shift_down_left());
        let bb = bb.or(bit.shift_down().shift_down_right());

        let bb = bb.or(bit.shift_up_left().shift_left());
        let bb = bb.or(bit.shift_down_left().shift_left());

        let bb = bb.or(bit.shift_up_right().shift_right());
        let bb = bb.or(bit.shift_down_right().shift_right());

        attacks[sq_idx] = bb;
        sq_idx += 1;
    }

    attacks
};

const KING_ATTACKS: [Bitboard; Square::COUNT] = {
    let mut attacks = [Bitboard::EMPTY; _];

    let mut sq_idx = 0;
    while sq_idx < Square::COUNT {
        let sq = Square::from_raw(sq_idx as u8).unwrap();
        let bit = sq.bb();

        let bb = bit.shift_up();
        let bb = bb.or(bit.shift_down());
        let bb = bb.or(bit.shift_left());
        let bb = bb.or(bit.shift_right());

        let bb = bb.or(bit.shift_up().shift_left());
        let bb = bb.or(bit.shift_up().shift_right());
        let bb = bb.or(bit.shift_down().shift_left());
        let bb = bb.or(bit.shift_down().shift_right());

        attacks[sq_idx] = bb;
        sq_idx += 1;
    }

    attacks
};

const ROOK_PSEUDO_ATTACKS: [Bitboard; Square::COUNT] = {
    let mut attacks = [Bitboard::EMPTY; _];

    let mut sq_idx = 0;
    while sq_idx < Square::COUNT {
        let sq = Square::from_raw(sq_idx as u8).unwrap();

        let rank = Bitboard::rank_bb(sq.rank());
        let file = Bitboard::file_bb(sq.file());

        attacks[sq_idx] = rank.xor(file);
        sq_idx += 1;
    }

    attacks
};

#[must_use]
pub const fn pawn_attacks(sq: Square, color: Color) -> Bitboard {
    PAWN_ATTACKS[color.idx()][sq.idx()]
}

#[must_use]
pub const fn alfil_attacks(sq: Square) -> Bitboard {
    ALFIL_ATTACKS[sq.idx()]
}

#[must_use]
pub const fn ferz_attacks(sq: Square) -> Bitboard {
    FERZ_ATTACKS[sq.idx()]
}

#[must_use]
pub const fn knight_attacks(sq: Square) -> Bitboard {
    KNIGHT_ATTACKS[sq.idx()]
}

#[must_use]
pub const fn king_attacks(sq: Square) -> Bitboard {
    KING_ATTACKS[sq.idx()]
}

#[must_use]
pub const fn rook_pseudo_attacks(sq: Square) -> Bitboard {
    ROOK_PSEUDO_ATTACKS[sq.idx()]
}

#[must_use]
pub const fn rook_attacks(sq: Square, occ: Bitboard) -> Bitboard {
    use hq::*;

    let flip = occ
        .shr(sq.file())
        .and(Bitboard::FILE_A)
        .raw()
        .wrapping_mul(DIAG);
    let file_sq = (flip >> 57) & 0x3f;
    let file_attacks = FILE_ATTACKS[sq.idx()][file_sq as usize];

    let rank_sq = occ.shr(RANK_SHIFTS[sq.idx()]).raw() & 0x3f;
    let rank_attacks = RANK_ATTACKS[sq.idx()][rank_sq as usize];

    file_attacks.or(rank_attacks)
}

#[must_use]
pub const fn attacks(piece: Piece, sq: Square, occ: Bitboard) -> Bitboard {
    match piece.piece_type() {
        PieceType::Pawn => pawn_attacks(sq, piece.color()),
        PieceType::Alfil => alfil_attacks(sq),
        PieceType::Ferz => ferz_attacks(sq),
        PieceType::Knight => knight_attacks(sq),
        PieceType::Rook => rook_attacks(sq, occ),
        PieceType::King => king_attacks(sq),
    }
}

#[must_use]
pub const fn pseudo_attacks(piece: Piece, sq: Square) -> Bitboard {
    match piece.piece_type() {
        PieceType::Pawn => pawn_attacks(sq, piece.color()),
        PieceType::Alfil => alfil_attacks(sq),
        PieceType::Ferz => ferz_attacks(sq),
        PieceType::Knight => knight_attacks(sq),
        PieceType::Rook => rook_pseudo_attacks(sq),
        PieceType::King => king_attacks(sq),
    }
}

#[must_use]
pub const fn white_pawn_attacks_setwise(pawns: Bitboard) -> Bitboard {
    let attacks = pawns.shift_up_left();
    let attacks = attacks.or(pawns.shift_up_right());
    attacks
}

#[must_use]
pub const fn black_pawn_attacks_setwise(pawns: Bitboard) -> Bitboard {
    let attacks = pawns.shift_down_left();
    let attacks = attacks.or(pawns.shift_down_right());
    attacks
}

#[must_use]
pub const fn pawn_attacks_setwise(pawns: Bitboard, color: Color) -> Bitboard {
    match color {
        Color::White => white_pawn_attacks_setwise(pawns),
        Color::Black => black_pawn_attacks_setwise(pawns),
    }
}
