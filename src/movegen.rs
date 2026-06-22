use crate::attacks::*;
use crate::bitboard::Bitboard;
use crate::core::{Color, PieceType, Square};
use crate::position::Position;
use crate::rays::{ray_between, ray_past};
use crate::shatranjmove::Move;

pub const MAX_MOVES: usize = 128;
pub type MoveList = arrayvec::ArrayVec<Move, MAX_MOVES>;

fn push_standards_with_offset(dst: &mut MoveList, offset: i32, to_squares: Bitboard) {
    for to in to_squares {
        let from = to.offset(-offset).unwrap();
        dst.push(Move::new(from, to));
    }
}

fn push_promos_with_offset(dst: &mut MoveList, offset: i32, to_squares: Bitboard) {
    for to in to_squares {
        let from = to.offset(-offset).unwrap();
        dst.push(Move::new_promo(from, to));
    }
}

fn push_standards_from_square(dst: &mut MoveList, from: Square, to_squares: Bitboard) {
    for to in to_squares {
        dst.push(Move::new(from, to));
    }
}

fn generate_pawns(dst: &mut MoveList, pos: &Position, mask: Bitboard) {
    let promo_rank = match pos.stm() {
        Color::White => Bitboard::RANK_8,
        Color::Black => Bitboard::RANK_1,
    };

    let (forward_offset, left_offset, right_offset) = match pos.stm() {
        Color::White => (8, 7, 9),
        Color::Black => (-8, -9, -7),
    };

    let king_sq = pos.king_sq(pos.stm());

    let pinned = pos.pinned();
    let pawns = pos.piece_bb(PieceType::Pawn.with_color(pos.stm()));

    let forward_pin_mask = Bitboard::file_bb(king_sq.file());
    let pushable_pawns = (pawns & !pinned) | (pawns & forward_pin_mask);

    let pushed = pushable_pawns.shift_up_relative(pos.stm()) & !pos.occ() & mask;
    push_standards_with_offset(dst, forward_offset, pushed & !promo_rank);
    push_promos_with_offset(dst, forward_offset, pushed & promo_rank);

    // pinned pawns cannot capture
    let capture_worthy_pawns = pawns & !pinned;
    let capture_mask = mask & pos.color_bb(pos.stm().flip());

    let left_captures = capture_worthy_pawns.shift_up_left_relative(pos.stm()) & capture_mask;
    push_standards_with_offset(dst, left_offset, left_captures & !promo_rank);
    push_promos_with_offset(dst, left_offset, left_captures & promo_rank);

    let right_captures = capture_worthy_pawns.shift_up_right_relative(pos.stm()) & capture_mask;
    push_standards_with_offset(dst, right_offset, right_captures & !promo_rank);
    push_promos_with_offset(dst, right_offset, right_captures & promo_rank);
}

fn generate_leapers(dst: &mut MoveList, pos: &Position, mask: Bitboard) {
    let pinned = pos.pinned();

    // none of these pieces can move when pinned

    let alfils = pos.piece_bb(PieceType::Alfil.with_color(pos.stm()));
    for alfil in alfils & !pinned {
        let moves = alfil_attacks(alfil) & mask;
        push_standards_from_square(dst, alfil, moves);
    }

    let ferzes = pos.piece_bb(PieceType::Ferz.with_color(pos.stm()));
    for ferz in ferzes & !pinned {
        let moves = ferz_attacks(ferz) & mask;
        push_standards_from_square(dst, ferz, moves);
    }

    let knights = pos.piece_bb(PieceType::Knight.with_color(pos.stm()));
    for knight in knights & !pinned {
        let moves = knight_attacks(knight) & mask;
        push_standards_from_square(dst, knight, moves);
    }
}

fn generate_rooks(dst: &mut MoveList, pos: &Position, mask: Bitboard) {
    let pinned = pos.pinned();
    let rooks = pos.piece_bb(PieceType::Rook.with_color(pos.stm()));

    let king_sq = pos.king_sq(pos.stm());
    let occ = pos.occ();

    for rook in rooks & !pinned {
        let moves = rook_attacks(rook, occ) & mask;
        push_standards_from_square(dst, rook, moves);
    }

    for rook in rooks & pinned {
        let pin_ray = ray_past(king_sq, rook);
        let moves = rook_attacks(rook, occ) & mask & pin_ray;
        push_standards_from_square(dst, rook, moves);
    }
}

fn generate_kings(dst: &mut MoveList, pos: &Position, mask: Bitboard) {
    let king_sq = pos.king_sq(pos.stm());
    let moves = king_attacks(king_sq) & mask & !pos.threats();
    push_standards_from_square(dst, king_sq, moves);
}

pub fn generate_all(dst: &mut MoveList, pos: &Position) {
    let king_mask = !pos.color_bb(pos.stm());
    let mut mask = king_mask;

    if pos.in_check() {
        if pos.checkers().popcount() > 1 {
            generate_kings(dst, pos, king_mask);
            return;
        }

        mask = pos.checkers() | ray_between(pos.king_sq(pos.stm()), pos.checkers().lsb().unwrap());
    }

    generate_pawns(dst, pos, mask);
    generate_leapers(dst, pos, mask);
    generate_rooks(dst, pos, mask);
    generate_kings(dst, pos, king_mask);
}
