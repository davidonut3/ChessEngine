use std::time::Duration;

use crate::{parsing, utils::*};
use crate::fenlib::fen::Fen;

// If two engines reach this number of plies, we stop the game,
// so that each game takes only MAX_NUMBER_OF_PLIES * time_per_move time.
const MAX_NUMBER_OF_PLIES: i32 = 500;

pub enum Side {
    White,
    Black,
}

pub struct GameResult {
    pub white_is_engine1: bool,
    pub start_fen: String,
    pub moves: Vec<Move>,
    pub outcome: GameOutcome,
    pub time_per_move: Duration,
}

impl GameResult {
    pub fn to_string(&self) -> String {
        let lan_vec: Vec<String> = parsing::moves_to_lan_list(&self.moves);
        let lan_string: String = lan_vec.join(" ");

        format!(
            "Fen: {}, Outcome: {}, WhiteIsEngine1: {:?}, Moves: {}, Time: {:?}",
            self.start_fen,
            self.outcome.to_string(),
            self.white_is_engine1,
            lan_string,
            self.time_per_move,
        )
    }
}

pub struct MatchResult {
    pub game_count: usize,
    pub game_results: Vec<GameResult>,
    pub engine1_wins: i32,
    pub engine2_wins: i32,
    pub draws: i32,
    pub other: i32,
}

impl MatchResult {
    pub fn print_match(&self, print_matches: bool) {
        if print_matches {
            for result in &self.game_results {
                println!("{}", result.to_string())
            }
        }
        
        let match_result: String = format!(
            "Games: {}, Engine1 wins: {}, Engine2 wins: {}, Draws: {}, Others: {}",
            self.game_count,
            self.engine1_wins,
            self.engine2_wins,
            self.draws,
            self.other,
        );

        println!("{}", match_result)
    }
}

pub trait Engine {
    fn new_game(fen: &str) -> Self where Self: Sized;

    fn select_move(&mut self, time_per_move: Duration) -> Move;

    fn apply_move(&mut self, move1: Move);
}

pub fn run_game<E: Engine>(fen_str: &str, time_per_move: Duration, white_engine: fn(&str) -> E, black_engine: fn(&str) -> E, white_is_engine1: bool) -> GameResult {
    let mut white: E = white_engine(fen_str);
    let mut black: E = black_engine(fen_str);
    let mut fen: Fen = Fen::from_str(fen_str);
    let mut moves: Vec<Move> = Vec::new();
    let mut outcome: GameOutcome = GameOutcome::Ongoing;
    let mut side: Side = Side::White;

    for _ply in 0..MAX_NUMBER_OF_PLIES {
        let move1: Move = match side {
            Side::White => {
                let move1: Move = white.select_move(time_per_move);
                black.apply_move(move1);
                move1
            }
            Side::Black => {
                let move1: Move = black.select_move(time_per_move);
                white.apply_move(move1);
                move1
            }
        };

        // We update our fen and add the move to the list of moves
        fen.move_to_fen(move1);
        moves.push(move1);

        // We check the game state to make sure the game is still going
        // We do not check repetition, since the bots will not offer draws
        outcome = fen.game_outcome(Some(MAX_NUMBER_OF_PLIES));

        // We break the loop if the game is over
        if outcome != GameOutcome::Ongoing { break }

        side = match side { Side::White => Side::Black, Side::Black => Side::White };
    }

    GameResult {
        white_is_engine1: white_is_engine1,
        start_fen: fen_str.to_string(),
        moves: moves,
        outcome: outcome,
        time_per_move: time_per_move
    }
}

pub fn run_games<E: Engine>(fen_strs: Vec<&str>, time_per_move: Duration, engine1: fn(&str) -> E, engine2: fn(&str) -> E) -> MatchResult {
    
    // Each bot plays each fen as both black and white, so we have twice as many games as fen_strs
    // I have no idea if/how the multithreading works :)

    // We create a list of jobs
    #[derive(Clone)]
    struct Job<'a> { fen: &'a str, flip: bool }
    let jobs: Vec<Job> = fen_strs.iter()
        .flat_map(|fen| vec![ Job{fen,flip:false}, Job{fen,flip:true} ])
        .collect();

    // We run the games in parallel
    let game_results: Vec<GameResult> = jobs.iter()
        .map(|job| {
            if !job.flip {
                run_game(job.fen, time_per_move, engine1, engine2, true)
            } else {
                run_game(job.fen, time_per_move, engine2, engine1, false)
            }
        })
        .collect();

    // We collect the results
    let mut engine1_wins: i32 = 0;
    let mut engine2_wins: i32 = 0;
    let mut draws: i32 = 0;
    let mut other: i32 = 0;

    for r in &game_results {
        match r.outcome {
            GameOutcome::WhiteWins if r.white_is_engine1 => engine1_wins += 1,
            GameOutcome::BlackWins if r.white_is_engine1 => engine2_wins += 1,
            GameOutcome::WhiteWins => engine2_wins += 1,
            GameOutcome::BlackWins => engine1_wins += 1,
            GameOutcome::Draw => draws += 1,
            _ => other += 1,
        }
    }

    MatchResult {
        game_count: game_results.len(),
        game_results,
        engine1_wins,
        engine2_wins,
        draws,
        other,
    }
}