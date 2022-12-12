use std::{io, str::FromStr};

#[derive(PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

struct Turn { opponent: Move, player: Move, score: Option<u32> }

fn main() {
    let turns = populate_moves();
    let turns = calculate_scores(turns);
    println!("Total score: {}", turns.iter().map(|t| t.score.unwrap()).sum::<u32>());
}

fn populate_moves() -> Vec<Turn> {
    let mut moves = Vec::new();
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let input_lenth = input_line.trim().len();
        if input_lenth == 0 {
            break;
        }
        let opponent_move: Move = input_line[0..1].parse().unwrap();
        let player_move: Move = input_line[2..3].parse().unwrap();
        moves.push(Turn{opponent: opponent_move, player: player_move, score: None});
    }
    moves
}

fn calculate_scores(mut turns: Vec<Turn>) -> Vec<Turn> {
    for turn in turns.iter_mut() {
        turn.score = Some(0);
        match turn.opponent {
            Move::Rock 
                if turn.player == Move::Rock => 
                    turn.score = Some(turn.score.unwrap() + 3 + Move::Rock as u32),
            Move::Rock 
                if turn.player == Move::Paper => 
                    turn.score = Some(turn.score.unwrap() + 6 + Move::Paper as u32),
            Move::Rock 
                if turn.player == Move::Scissors => 
                    turn.score = Some(turn.score.unwrap() + 0 + Move::Scissors as u32),
            Move::Paper 
                if turn.player == Move::Rock => 
                    turn.score = Some(turn.score.unwrap() + 0 + Move::Rock as u32),
            Move::Paper 
                if turn.player == Move::Paper => 
                    turn.score = Some(turn.score.unwrap() + 3 + Move::Paper as u32),
            Move::Paper 
                if turn.player == Move::Scissors => 
                    turn.score = Some(turn.score.unwrap() + 6 + Move::Scissors as u32),
            Move::Scissors 
                if turn.player == Move::Rock => 
                    turn.score = Some(turn.score.unwrap() + 6 + Move::Rock as u32),
            Move::Scissors 
                if turn.player == Move::Paper => 
                    turn.score = Some(turn.score.unwrap() + 0 + Move::Paper as u32),
            Move::Scissors 
                if turn.player == Move::Scissors => 
                    turn.score = Some(turn.score.unwrap() + 3 + Move::Scissors as u32),
            _ => (),
        }
    }
    turns
}

impl FromStr for Move {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}