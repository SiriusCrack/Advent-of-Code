use std::{io, str::FromStr};

#[derive(PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
#[derive(PartialEq)]
enum Response {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

struct Round { opponent: Move, player: Response, score: Option<u32> }

fn main() {
    let rounds = populate_moves();
    let rounds = calculate_scores(rounds);
    println!("Total score: {}", rounds.iter().map(|t| t.score.unwrap()).sum::<u32>());
}

fn populate_moves() -> Vec<Round> {
    let mut moves = Vec::new();
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let input_lenth = input_line.trim().len();
        if input_lenth == 0 {
            break;
        }
        let opponent_move: Move = input_line[0..1].parse().unwrap();
        let player_move: Response = input_line[2..3].parse().unwrap();
        moves.push(Round{opponent: opponent_move, player: player_move, score: None});
    }
    moves
}

fn calculate_scores(mut rounds: Vec<Round>) -> Vec<Round> {
    for round in rounds.iter_mut() {
        round.score = Some(0);
        match round.opponent {
            Move::Rock 
                if round.player == Response::Lose => 
                    round.score = Some(
                        round.score.unwrap() +
                        Response::Lose as u32 +
                        Move::Scissors as u32
                    ),
            Move::Rock 
                if round.player == Response::Draw => 
                    round.score = Some(
                        round.score.unwrap() +
                        Response::Draw as u32 +
                        Move::Rock as u32
                    ),
            Move::Rock 
                if round.player == Response::Win => 
                    round.score = Some(
                        round.score.unwrap() +
                        Response::Win  as u32+
                        Move::Paper as u32
                    ),
            Move::Paper 
                if round.player == Response::Lose => 
                    round.score = Some(
                        round.score.unwrap() +
                        Response::Lose as u32 +
                        Move::Rock as u32
                    ),
            Move::Paper 
                if round.player == Response::Draw => 
                    round.score = Some(
                        round.score.unwrap() +
                        Response::Draw as u32 +
                        Move::Paper as u32
                    ),
            Move::Paper 
                if round.player == Response::Win => 
                    round.score = Some(
                        round.score.unwrap() +
                        Response::Win  as u32+
                        Move::Scissors as u32
                    ),
            Move::Scissors 
                if round.player == Response::Lose => 
                    round.score = Some(
                        round.score.unwrap() +
                        Response::Lose as u32 +
                        Move::Paper as u32
                    ),
            Move::Scissors 
                if round.player == Response::Draw => 
                    round.score = Some(
                        round.score.unwrap() +
                        Response::Draw as u32 +
                        Move::Scissors as u32
                    ),
            Move::Scissors 
                if round.player == Response::Win => 
                    round.score = Some(
                        round.score.unwrap() +
                        Response::Win  as u32+
                        Move::Rock as u32
                    ),
            _ => (),
        }
    }
    rounds
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
impl FromStr for Response {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Response::Lose),
            "Y" => Ok(Response::Draw),
            "Z" => Ok(Response::Win),
            _ => Err(()),
        }
    }
}