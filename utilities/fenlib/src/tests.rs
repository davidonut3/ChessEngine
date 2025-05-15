use crate::fen::*;
use crate::parsing;
use crate::games;

use std::time::Instant;
use std::time::Duration;

pub fn perft(max_depth: usize, fen_str: &str, per_move: bool) -> usize {
    // https://www.chessprogramming.org/Perft

    let fen: Fen = Fen::from_str(fen_str);

    let possible_moves: Vec<[u64; 3]> = fen.get_all_possible_moves();

    if max_depth < 2 {
        if per_move {
            for move1 in &possible_moves {
                println!("Move {}", parsing::move_to_lan(&move1))
            }
        }
        
        return possible_moves.len()
    } else {
        let mut total: usize = 0;

        for move1 in possible_moves {
            let mut new_fen: Fen = fen.clone();
            new_fen.move_to_fen(&move1);
            let count: usize = recursive_perft_check(&new_fen, max_depth - 1);

            if per_move {
                println!("Move {} lead to {:?} moves", parsing::move_to_lan(&move1), count);
            }
            total += count;
        }

        return total
    }
}

pub fn recursive_perft_check(fen: &Fen, depth: usize) -> usize {
    let possible_moves: Vec<[u64; 3]> = fen.get_all_possible_moves();

    if depth == 1 {
        // if we reach a depth of 1, we return the number of legal moves from the current fen
        return possible_moves.len()
    } else {
        // if we are not at a depth of 1, we recursively call the function to determine the number of legal moves after <depth> moves
        let mut total: usize = 0;
        for move1 in possible_moves {
            let mut new_fen: Fen = fen.clone();
            new_fen.move_to_fen(&move1);
            total += recursive_perft_check(&new_fen, depth - 1)
        }
        return total;
    }
}

fn analyze_durations(durations: &[Duration; 1000]) -> (Duration, Duration, Duration) {
    let mut total_nanos: u128 = 0;
    let mut min: Duration = durations[0];
    let mut max: Duration = durations[0];

    for &d in durations {
        total_nanos += d.as_nanos();
        if d < min {
            min = d;
        }
        if d > max {
            max = d;
        }
    }

    let avg: Duration = Duration::from_nanos((total_nanos / durations.len() as u128) as u64);
    (min, max, avg)
}

pub fn move_gen_perft() {
    let global_time: Instant = Instant::now();
    println!("Starting performance test for move generation");
    let games: [Fen; 1000] = games::get_random_games();
    println!("Creating Fens took {:?}", global_time.elapsed());

    let mut durations: [Duration; 1000] = [Duration::from_nanos(0); 1000];
    for i in 0..1000 {
        let time: Instant = Instant::now();
        games[i].get_all_possible_moves();
        durations[i] = time.elapsed();
    }

    let info: (Duration, Duration, Duration) = analyze_durations(&durations);
    let min: Duration = info.0;
    let max: Duration = info.1;
    let average: Duration = info.2;

    println!("Min duration {:?}", min);
    println!("Max duration {:?}", max);
    println!("Average duration {:?}", average);
}