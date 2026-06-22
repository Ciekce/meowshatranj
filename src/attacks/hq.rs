use crate::bitboard::Bitboard;
use crate::core::Square;

const WEST: [u64; Square::COUNT] = {
    let mut bbs = [0; _];

    let mut sq_idx = 0;
    while sq_idx < Square::COUNT {
        let sq = Square::from_raw(sq_idx as u8).unwrap();
        let bit = sq.bb().raw();

        let bb = (bit - 1) & (0xFF << (sq_idx & 0b111000));

        bbs[sq_idx] = bb;
        sq_idx += 1;
    }

    bbs
};

const EAST: [u64; Square::COUNT] = {
    let mut bbs = [0; _];

    let mut sq_idx = 0;
    while sq_idx < Square::COUNT {
        let sq = Square::from_raw(sq_idx as u8).unwrap();
        let bit = sq.bb().raw();

        let bb = bit ^ WEST[sq_idx] ^ (0xFF << (sq_idx & 0b111000));

        bbs[sq_idx] = bb;
        sq_idx += 1;
    }

    bbs
};

pub(super) const DIAG: u64 = 0x8040201008040201;

pub(super) const RANK_SHIFTS: [usize; Square::COUNT] = {
    let mut shifts = [0; _];

    let mut sq_idx = 0;
    while sq_idx < Square::COUNT {
        shifts[sq_idx] = (sq_idx & 0b111000) + 1;
        sq_idx += 1;
    }

    shifts
};

pub(super) const RANK_ATTACKS: [[Bitboard; Square::COUNT]; Square::COUNT] = {
    let mut attacks = [[Bitboard::EMPTY; _]; _];

    let mut sq_idx = 0;
    while sq_idx < Square::COUNT {
        let sq = Square::from_raw(sq_idx as u8).unwrap();

        let mut rank_sq_idx = 0;
        while rank_sq_idx < Square::COUNT {
            let occ = (rank_sq_idx << 1) as u64;

            let east = EAST[sq.file()];
            let east = east ^ EAST[((east & occ) | (1 << 63)).trailing_zeros() as usize];

            let west = WEST[sq.file()];
            let west = west ^ WEST[(((west & occ) | 1).leading_zeros() ^ 63) as usize];

            attacks[sq_idx][rank_sq_idx] = Bitboard::from_raw((east | west) << (sq_idx & 0b111000));

            rank_sq_idx += 1;
        }

        sq_idx += 1;
    }

    attacks
};

pub(super) const FILE_ATTACKS: [[Bitboard; Square::COUNT]; Square::COUNT] = {
    let mut attacks = [[Bitboard::EMPTY; _]; _];

    let mut sq_idx = 0;
    while sq_idx < Square::COUNT {
        let sq = Square::from_raw(sq_idx as u8).unwrap();

        let mut file_sq_idx = 0;
        while file_sq_idx < Square::COUNT {
            let rank_attacks = RANK_ATTACKS[7 - sq.rank()][file_sq_idx].raw();

            attacks[sq_idx][file_sq_idx] = Bitboard::from_raw(
                (rank_attacks.wrapping_mul(DIAG) & Bitboard::FILE_H.raw()) >> (7 - sq.file()),
            );

            file_sq_idx += 1;
        }

        sq_idx += 1;
    }

    attacks
};
