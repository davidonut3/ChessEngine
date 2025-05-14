use crate::fen::*;
use crate::parsing;

pub fn perft(max_depth: usize, fen_str: &str, per_move: bool) {
    // https://www.chessprogramming.org/Perft

    let fen: Fen = Fen::from_str(fen_str);

    let possible_moves: Vec<[u64; 3]> = fen.get_all_possible_moves();

    if max_depth < 2 {
        if per_move {
            for move1 in &possible_moves {
                println!("Move {}", parsing::move_to_lan(&move1))
            }
        }
        
        println!("Checked a total of {:?} moves", possible_moves.len());
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

        println!("Checked a total of {:?} moves", total);
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