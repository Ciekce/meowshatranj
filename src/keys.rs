use crate::core::*;

struct Sfc64 {
    a: u64,
    b: u64,
    c: u64,
    counter: u64,
}

impl Sfc64 {
    const fn new(seed: u64) -> Self {
        let mut result = Self {
            a: seed,
            b: seed,
            c: seed,
            counter: 1,
        };

        let mut i = 0;
        while i < 12 {
            result.next_u64();
            i += 1;
        }

        result
    }

    const fn next_u64(&mut self) -> u64 {
        let result = self.a.wrapping_add(self.b).wrapping_add(self.counter);
        self.counter = self.counter.wrapping_add(1);
        self.a = self.b ^ (self.b >> 11);
        self.b = self.c.wrapping_add(self.c << 3);
        self.c = self.c.rotate_left(24).wrapping_add(result);
        result
    }

    const fn fill(&mut self, values: &mut [u64]) {
        let mut idx = 0;
        while idx < values.len() {
            values[idx] = self.next_u64();
            idx += 1;
        }
    }
}

const STM_COUNT: usize = 1;
const PSQ_COUNT: usize = Piece::COUNT * Square::COUNT;

const TOTAL_COUNT: usize = STM_COUNT + PSQ_COUNT;

const STM_OFFSET: usize = 0;
const PSQ_OFFSET: usize = STM_OFFSET + STM_COUNT;

#[allow(clippy::large_const_arrays)]
const KEYS: [u64; TOTAL_COUNT] = {
    const SEED: u64 = 0xa6b67fd132b42f39;

    let mut result = [0; TOTAL_COUNT];

    let mut prng = Sfc64::new(SEED);
    prng.fill(&mut result);

    result
};

#[must_use]
pub const fn stm_key() -> u64 {
    KEYS[STM_OFFSET]
}

#[must_use]
pub const fn psq_key(piece: Piece, sq: Square) -> u64 {
    KEYS[PSQ_OFFSET + sq.idx() * Piece::COUNT + piece.idx()]
}
