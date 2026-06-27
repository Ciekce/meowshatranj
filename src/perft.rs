use crate::movegen::{MoveList, generate_all};
use crate::position::Position;
use std::time::Instant;

fn do_perft(pos: &Position, depth: i32) -> usize {
    if depth <= 0 {
        return 1;
    }

    let mut moves = MoveList::new();
    generate_all(&mut moves, pos);

    if depth == 1 {
        return moves.len();
    }

    let mut total = 0;

    for mv in moves {
        debug_assert!(pos.is_legal(mv));

        let pos = pos.apply_move(mv);
        total += do_perft(&pos, depth - 1);
    }

    total
}

#[must_use]
pub fn perft(pos: &Position, depth: i32) -> usize {
    do_perft(pos, depth.max(1))
}

pub fn split_perft(pos: &Position, depth: i32) {
    let depth = depth.max(1);

    let start = Instant::now();

    let mut moves = MoveList::new();
    generate_all(&mut moves, pos);

    let mut total = 0;

    for mv in moves {
        debug_assert!(pos.is_legal(mv));

        print!("{:5}  ", mv.to_string());

        let pos = pos.apply_move(mv);
        let value = do_perft(&pos, depth - 1);

        total += value;
        println!("{}", value);
    }

    let nps = (total as f64 / start.elapsed().as_secs_f64()) as usize;

    println!();
    println!("total: {}", total);
    println!("{} nps", nps);
}
