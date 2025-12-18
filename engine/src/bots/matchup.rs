use std::time::Duration;
use std::time::Instant;
use rayon::prelude::*;

use crate::utils::*;
use crate::parsing;
use crate::fenlib::fen::Fen;

// If two engines reach this number of plies, we stop the game,
// so that each game takes only MAX_NUMBER_OF_PLIES * time_per_move time.
const MAX_NUMBER_OF_PLIES: i32 = 500;

pub struct GameResult {
    pub white_is_engine1: bool,
    pub start_fen: String,
    pub end_fen: String,
    pub moves: Vec<Move>,
    pub outcome: GameOutcome,
    pub time_per_move: Duration,
}

impl GameResult {
    pub fn print_result(&self) {
        let lan_vec: Vec<String> = parsing::moves_to_lan_list(&self.moves);
        let lan_string: String = lan_vec.join(" ");

        println!(
            "StartFen: {}\nEndFen: {}\nOutcome: {}\nWhiteIsEngine1: {}\nTime: {:?}\nNumberOfMoves: {}\nMoves: {}\n----------",
            self.start_fen,
            self.end_fen,
            self.outcome.to_string(),
            self.white_is_engine1,
            self.time_per_move,
            lan_vec.len(),
            lan_string
        )
    }
}

pub struct MatchResult {
    pub game_count: usize,
    pub game_results: Vec<GameResult>,
    pub engine1: String,
    pub engine2: String,
    pub engine1_wins: i32,
    pub engine2_wins: i32,
    pub draws: i32,
    pub other: i32,
    pub time_taken: Duration,
}

impl MatchResult {
    pub fn print_match(&self, print_matches: bool) {
        let mut moves_per_game: Vec<usize> = Vec::new();

        for result in &self.game_results {
            moves_per_game.push(result.moves.len());

            if print_matches {
                result.print_result();
            }
        }

        let average_moves: f32 = moves_per_game.iter().sum::<usize>() as f32 / self.game_results.len() as f32;

        moves_per_game.sort();
        let middle: usize = self.game_results.len() / 2 as usize;

        let median_moves: usize = moves_per_game[middle];
        
        let match_result: String = format!(
            "Total: {}, {} wins: {}, {} wins: {}, Draws: {}, Others: {}\nAverage moves: {}, Median moves: {}, Time taken: {:?}",
            self.game_count,
            self.engine1,
            self.engine1_wins,
            self.engine2,
            self.engine2_wins,
            self.draws,
            self.other,
            average_moves,
            median_moves,
            self.time_taken,
        );

        println!("{}", match_result)
    }
}

pub trait Engine {
    fn new_game(fen: &str) -> Self where Self: Sized;

    fn select_move(&mut self, time_per_move: Duration) -> Move;

    fn apply_move(&mut self, move1: Move);

    fn name(&self) -> String;
}

pub fn run_game<E1: Engine, E2: Engine>(fen_str: &str, time_per_move: Duration, engine1_constr: fn(&str) -> E1, engine2_constr: fn(&str) -> E2, white_is_engine1: bool) -> GameResult {
    let mut engine1 = engine1_constr(fen_str);
    let mut engine2 = engine2_constr(fen_str);

    let mut fen: Fen = Fen::from_str(fen_str);
    let mut moves: Vec<Move> = Vec::new();
    let mut outcome: GameOutcome = GameOutcome::Ongoing;

    for _ply in 0..MAX_NUMBER_OF_PLIES {
        let move1: Move = match fen.white_to_move() {
            true    => { if white_is_engine1 { engine1.select_move(time_per_move) } else { engine2.select_move(time_per_move) } }
            false   => { if white_is_engine1 { engine2.select_move(time_per_move) } else { engine1.select_move(time_per_move) } }
        };

        engine1.apply_move(move1);
        engine2.apply_move(move1);

        // We update our fen and add the move to the list of moves
        fen.move_to_fen(move1);
        moves.push(move1);

        // We check the game state to make sure the game is still going
        // We do not check repetition, since the bots will not offer draws
        outcome = fen.game_outcome(Some(MAX_NUMBER_OF_PLIES));

        // We break the loop if the game is over
        if outcome != GameOutcome::Ongoing { break }
    }

    GameResult {
        white_is_engine1: white_is_engine1,
        start_fen: fen_str.to_string(),
        end_fen: fen.to_string(),
        moves: moves,
        outcome: outcome,
        time_per_move: time_per_move
    }
}

pub fn run_games<E1: Engine, E2: Engine>(fen_strs: &[String], time_per_move: Duration, engine1_constr: fn(&str) -> E1, engine2_constr: fn(&str) -> E2) -> MatchResult {

    let start_time: Instant = Instant::now();
    
    // Each bot plays each fen as both black and white, so we have twice as many games as fen_strs
    let mut all_games: Vec<(&str, bool)> = Vec::new();
    for fen_str in fen_strs {
        all_games.push((fen_str, true));
        all_games.push((fen_str, false));
    }

    // We run all games in parallel using Rayon's .par_iter()
    let game_results: Vec<GameResult> = all_games
        .par_iter()
        .map(|&(fen_str, white_is_engine1)| {
            run_game(fen_str, time_per_move, engine1_constr, engine2_constr, white_is_engine1)
        })
        .collect();

    let engine1: String = engine1_constr(DEFAULT).name();
    let engine2: String = engine2_constr(DEFAULT).name();
    
    let mut engine1_wins: i32 = 0;
    let mut engine2_wins: i32 = 0;
    let mut draws: i32 = 0;
    let mut other: i32 = 0;

    for r in &game_results {
        match r.outcome {
            GameOutcome::WhiteWins => if r.white_is_engine1 { engine1_wins += 1 } else { engine2_wins += 1},
            GameOutcome::BlackWins => if r.white_is_engine1 { engine2_wins += 1 } else { engine1_wins += 1},
            GameOutcome::Draw => draws += 1,
            _ => other += 1,
        }
    }

    MatchResult {
        game_count: game_results.len(),
        game_results,
        engine1,
        engine2,
        engine1_wins,
        engine2_wins,
        draws,
        other,
        time_taken: start_time.elapsed(),
    }
}