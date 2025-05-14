use crate::fen::*;
use crate::parsing;

pub fn perft(max_depth: usize, fen_str: &str) {
    // https://www.chessprogramming.org/Perft

    let fen: Fen = Fen::from_str(fen_str);

    let possible_moves: Vec<[u64; 3]> = fen.get_all_possible_moves();

    for move1 in possible_moves {
        if max_depth < 2 {
            println!("Move {}", parsing::move_to_lan(&move1))
        } else {
            let mut new_fen: Fen = fen.clone();
            new_fen.move_to_fen(&move1);
            println!("Move {} lead to {:?} moves", parsing::move_to_lan(&move1), recursive_perft_check(&new_fen, max_depth - 1))
        }
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