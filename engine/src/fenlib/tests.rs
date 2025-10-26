use crate::fenlib::fen::Fen;
use crate::fenlib::parsing;
use crate::fenlib::games;
use crate::fenlib::utils::*;

use std::time::Instant;
use std::time::Duration;

pub fn perft(max_depth: usize, fen_str: &str, per_move: bool) -> usize {
    // https://www.chessprogramming.org/Perft

    let fen: Fen = Fen::from_str(fen_str);

    let (legal_moves, move_count) = fen.get_legal_moves_array();

    if max_depth < 2 {
        if per_move {
            for i in 0..move_count {
                println!("Move {}", parsing::move_to_lan(&legal_moves[i]))
            }
        }
        
        return move_count
    } else {
        let mut total: usize = 0;

        for i in 0..move_count {
            let move1: Move = legal_moves[i];

            let mut new_fen: Fen = fen.clone();
            new_fen.move_to_fen(move1);
            let count: usize = recursive_perft_check(&new_fen, max_depth - 1);

            if per_move {
                println!("{}: {:?}", parsing::move_to_lan(&move1), count);
            }
            total += count;
        }

        return total
    }
}

pub fn recursive_perft_check(fen: &Fen, depth: usize) -> usize {
    let (legal_moves, move_count) = fen.get_legal_moves_array();

    if depth == 1 {
        // if we reach a depth of 1, we return the number of legal moves from the current fen
        return move_count
    } else {
        // if we are not at a depth of 1, we recursively call the function to determine the number of legal moves after `depth` moves
        let mut total: usize = 0;
        for i in 0..move_count {
            let move1: Move = legal_moves[i];

            let mut new_fen: Fen = fen.clone();
            new_fen.move_to_fen(move1);
            total += recursive_perft_check(&new_fen, depth - 1)
        }
        
        return total;
    }
}

pub fn move_gen_perft() {
    let global_time: Instant = Instant::now();
    println!("Starting performance test for move generation");
    let games: [Fen; 1000] = games::get_random_games();
    println!("Creating Fens took {:?}", global_time.elapsed());

    // We call the function once before testing since the first one is always significantly slower than the rest
    games[0].get_legal_moves_array();

    let mut durations: [Duration; 1000] = [Duration::from_nanos(0); 1000];
    for i in 0..1000 {
        let time: Instant = Instant::now();
        games[i].get_legal_moves_array();
        durations[i] = time.elapsed();
    }

    let mut total_nanos: u128 = 0;
    let mut min: Duration = durations[0];
    let mut max: Duration = durations[0];

    let mut worst_fen: String = games[0].to_string();
    let mut best_fen: String = games[0].to_string();

    for i in 0..1000 {
        let duration: Duration = durations[i];
        println!("{:?} at {}", duration, games[i].to_string());
        total_nanos += duration.as_nanos();

        if duration < min {
            min = duration;
            best_fen = games[i].to_string();
        }

        if duration > max {
            max = duration;
            worst_fen = games[i].to_string();
        }
    }

    let avg: Duration = Duration::from_nanos((total_nanos / durations.len() as u128) as u64);

    println!("Min duration {:?} at {}", min, best_fen);
    println!("Max duration {:?} at {}", max, worst_fen);
    println!("Average duration {:?}", avg);
}

pub fn moves_per_second_perft() {
    let time: Instant = Instant::now();

    let count = perft(7, DEFAULT, false);
    
    let duration = time.elapsed();

    println!("Getting {:?} moves took {:?}", count, duration)
}