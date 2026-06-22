use crate::attacks;
use crate::bitboard::Bitboard;
use crate::core::Square;

const BETWEEN_RAYS: [[Bitboard; Square::COUNT]; Square::COUNT] = {
    let mut rays = [[Bitboard::EMPTY; _]; _];

    let mut from_idx = 0;
    while from_idx < Square::COUNT {
        let from = Square::from_raw(from_idx as u8).unwrap();

        let from_mask = attacks::rook_pseudo_attacks(from);

        let mut to_idx = 0;
        while to_idx < Square::COUNT {
            if from_idx == to_idx {
                to_idx += 1;
                continue;
            }

            let to = Square::from_raw(to_idx as u8).unwrap();

            if from_mask.has_sq(to) {
                let from_attacks = attacks::rook_attacks(from, to.bb());
                let to_attacks = attacks::rook_attacks(to, from.bb());
                rays[from_idx][to_idx] = from_attacks.and(to_attacks);
            }

            to_idx += 1;
        }

        from_idx += 1;
    }

    rays
};

const PASSING_RAYS: [[Bitboard; Square::COUNT]; Square::COUNT] = {
    let mut rays = [[Bitboard::EMPTY; _]; _];

    let mut from_idx = 0;
    while from_idx < Square::COUNT {
        let from = Square::from_raw(from_idx as u8).unwrap();

        let from_mask = attacks::rook_pseudo_attacks(from);

        let mut to_idx = 0;
        while to_idx < Square::COUNT {
            if from_idx == to_idx {
                to_idx += 1;
                continue;
            }

            let to = Square::from_raw(to_idx as u8).unwrap();

            if from_mask.has_sq(to) {
                let to_attacks = attacks::rook_attacks(to, from.bb()).or(to.bb());
                rays[from_idx][to_idx] = from_mask.and(to_attacks);
            }

            to_idx += 1;
        }

        from_idx += 1;
    }

    rays
};

pub const fn ray_between(a: Square, b: Square) -> Bitboard {
    BETWEEN_RAYS[a.idx()][b.idx()]
}

pub const fn ray_past(a: Square, b: Square) -> Bitboard {
    PASSING_RAYS[a.idx()][b.idx()]
}
