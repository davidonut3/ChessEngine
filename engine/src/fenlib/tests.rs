use rand::Rng;

use crate::fenlib::fen::Fen;
use crate::parsing;
use crate::games;
use crate::utils::*;

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
    let games: [Fen; 1000] = games::get_random_fens();
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

pub fn check_function_perft() {
    println!("Starting performance test for check function");
    let games: [Fen; 1000] = games::get_random_fens();
    let count = 100000000;

    let mut rng = rand::rng();

    let time: Instant = Instant::now();

    for _ in 0..count {
        let index = rng.random_range(0..1000);
        let game = &games[index];
        game.player_in_check(game.white_to_move());
    }

    let duration = time.elapsed();

    println!("Calling function on {:?} fens took {:?}", count, duration)
}

pub fn validate_move_gen() {
    // https://www.chessprogramming.org/Perft_Results

    // TODO: Add the number of captures, castles, promotions etc to the check

    let position1: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position2: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let position3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    let position4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    let position5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    let position6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

    let fen1 = Fen::from_str(position1);
    let fen2 = Fen::from_str(position2);
    let fen3 = Fen::from_str(position3);
    let fen4 = Fen::from_str(position4);
    let fen5 = Fen::from_str(position5);
    let fen6 = Fen::from_str(position6);

    let fens: Vec<Fen> = vec![fen1, fen2, fen3, fen4, fen5, fen6];

    let nodes1: Vec<u64> = vec![20, 400, 8902, 197281, 4865609, 119060324, 3195901860];
    let nodes2: Vec<u64> = vec![48, 2039, 97862, 4085603, 193690690, 8031647685];
    let nodes3: Vec<u64> = vec![14, 191, 2812, 43238, 674624, 11030083, 178633661, 3009794393];
    let nodes4: Vec<u64> = vec![6, 264, 9467, 422333, 15833292, 706045033];
    let nodes5: Vec<u64> = vec![44, 1486, 62379, 2103487, 89941194];
    let nodes6: Vec<u64> = vec![46, 2079, 89890, 3894594, 164075551, 6923051137];

    let nodes: Vec<Vec<u64>> = vec![nodes1, nodes2, nodes3, nodes4, nodes5, nodes6];

    println!("Starting validation");
    for i in 0..fens.len() {
        let fen = &fens[i];
        let node = &nodes[i];

        for j in 0..node.len() {
            let node_count = recursive_perft_check(fen, j + 1) as u64;
            assert_eq!(node_count, node[j]);
        }

        println!("Validation of position {:?} complete", i + 1);
    }
}