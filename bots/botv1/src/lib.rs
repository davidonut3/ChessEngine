use fenlib::fen::*;
use std::time::Instant;
use std::time::Duration;

/*
BotV1 implements:

- minimax algorithm to find the best move
- eval function based on material score
- a dynamic tree structure for the moves tree
- time constraint for searching tree

*/

// Values from https://www.chessprogramming.org/Simplified_Evaluation_Function
const PAWN_VAL: u32 = 100;
const KNIGHT_VAL: u32 = 320;
const BISHOP_VAL: u32 = 330;
const ROOK_VAL: u32 = 500;
const QUEEN_VAL: u32 = 900;
const KING_VAL: u32 = 20000;

const EQUAL: u32 = 0x80000000;

const INFINITY: u32 = u32::max_value();
const MAX_TIME_MILI: u64 = 0x3E8; //1000 miliseconds per move

pub fn eval(boards: &[u64; 12]) -> u32 {
    let mut score: u32 = EQUAL;

    score += PAWN_VAL * boards[0].count_ones();
    score -= PAWN_VAL * boards[6].count_ones();

    score += KNIGHT_VAL * boards[1].count_ones();
    score -= KNIGHT_VAL * boards[7].count_ones();

    score += BISHOP_VAL * boards[2].count_ones();
    score -= BISHOP_VAL * boards[8].count_ones();

    score += ROOK_VAL * boards[3].count_ones();
    score -= ROOK_VAL * boards[9].count_ones();

    score += QUEEN_VAL * boards[4].count_ones();
    score -= QUEEN_VAL * boards[10].count_ones();

    score += KING_VAL * boards[5].count_ones();
    score -= KING_VAL * boards[11].count_ones();

    score
}

pub fn get_worst_eval(white_to_move: bool) -> u32 {
    match white_to_move {
        true => 0x0,
        false => INFINITY
    }
}

pub fn get_better_move(white_to_move: bool, old: u32, new: u32) -> u32 {
    let new_is_greater: bool;
    if new > old {
        new_is_greater = true
    } else {
        new_is_greater = false
    }

    if (white_to_move && new_is_greater) || (!white_to_move && !new_is_greater) {
        return new;
    } else {
        return old;
    }
}

pub struct Move {
    move1: [u64; 3],
    fen: Fen,
    eval: u32,
    index: usize,
    new_moves: Vec<[u64; 3]>,
    children: Vec<Box<Move>>,
    child_count: usize,
}

impl Move {
    pub fn new(move1: [u64; 3], old_fen: &Fen) -> Self {

        // cloning fen seems to take 0-100 ns, this is a built-in Rust function, so there is little we can do to optimise this
        let mut fen: Fen = old_fen.clone();

        // move to fen seems to take 100-200 ns, we could speed this up even furthur, but it is quite optimised already
        fen.move_to_fen(&move1);
        
        // getting possible moves seems to take 45-55 us, we could speed this up by precomputing attacked and defended squares, and perhaps magic bitboards
        let new_moves: Vec<[u64; 3]> = fen.get_all_possible_moves();

        // getting the eval seems to take 0-100 ns, though this is still a very primitive function so time could increase in future versions
        let eval: u32 = eval(&fen.boards);

        let child_count: usize = new_moves.len();

        Self {
            move1,
            fen,
            eval,
            index: 0,
            new_moves,
            children: Vec::new(),
            child_count,
        }
    }

    pub fn add_child(&mut self) {
        let new_child: Box<Move> = Box::new(Self::new(self.new_moves[self.index], &self.fen));
        self.children.push(new_child);
        self.index += 1;
    }
}

#[derive(Debug, Clone)]
pub struct BotV1 {
    fen: Fen,
}

impl BotV1 {
    pub fn new() -> Self {
        let fen: Fen = Fen::new();

        Self { fen, }
    }

    pub fn from_fen(fen_str: &str) -> Self {
        let fen: Fen = Fen::from_str(fen_str);

        Self { fen, }
    }

    pub fn minimax(&self, move1: &mut Move, depth: u32, start_time: Instant, max_time: Duration) -> Option<u32> {

        if start_time.elapsed() >= max_time {
            return None
        }
        
        if depth == 0 {
            return Some(move1.eval)
        }

        let white_to_move: bool = move1.fen.white_to_move;
        let mut value: u32 = get_worst_eval(white_to_move);

        for i in 0..move1.child_count {

            if i == move1.index {
                move1.add_child()
            }

            if let Some(new_value) = self.minimax(&mut move1.children[i], depth - 1, start_time, max_time) {
                value = get_better_move(white_to_move, value, new_value);
            } else {
                return None
            }
        }
        return Some(value)
    }

    pub fn get_move(&mut self) -> String {
        let start_time: Instant = Instant::now();
        let max_time: Duration = Duration::from_millis(MAX_TIME_MILI);
        let white_to_move: bool = self.fen.white_to_move;

        let moves: Vec<[u64; 3]> = self.fen.get_all_possible_moves();
        let mut possible_moves: Vec<Move> = Vec::new();
        for move1 in moves {
            let new_move = Move::new(move1, &self.fen);
            possible_moves.push(new_move);
        }

        let mut best_move: [u64; 3] = possible_moves[0].move1;
        let mut best_score: u32 = get_worst_eval(white_to_move);
        let mut depth: u32 = 1;

        loop {
            println!("We are currently looking at depth: {:?}", depth);
            let best_move_prev_iter: [u64; 3] = best_move;
            for move1 in &mut possible_moves {
                if let Some(new_score) = self.minimax(move1, depth, start_time, max_time) {
                    if self.fen.white_to_move && new_score > best_score {
                        best_score = new_score;
                        best_move = move1.move1;
                    } else if new_score < best_score {
                        best_score = new_score;
                        best_move = move1.move1;
                    }
                } else {
                    self.fen.move_to_fen(&best_move_prev_iter);
                    println!("Calculating move took {:?}", start_time.elapsed());
                    return fenlib::parsing::move_to_lan(&best_move_prev_iter);
                }
            }
            depth += 1
        }

    }

    pub fn receive_move(&mut self, lan: &str) {
        self.fen.lan_to_fen(lan);
    }
}