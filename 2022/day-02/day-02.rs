use std::fs;

enum Move {
    Rock,
    Paper,
    Scissor,
    X, Y, Z
}

fn main() {
    let list = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    let rounds: Vec<Vec<Move>> = list
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|round| {
            round
                .split(" ")
                .map(|hand| {
                    match hand {
                        "A" => Move::Rock,
                        "B" => Move::Paper,
                        "C" => Move::Scissor,
                        "X" => Move::X,
                        "Y" => Move::Y,
                        "Z" => Move::Z,
                        _ => {
                            dbg!(hand);
                            panic!("Invalid move played")
                        }
                    }
                })
                .collect::<Vec<Move>>()
        })
        .collect();

    let score_one: u32 = rounds
        .iter()
        .map(|round| {
            let self_score = match round[..] {
                [_, Move::X] => 1,
                [_, Move::Y] => 2,
                [_, Move::Z] => 3,
                _ => panic!("Invalid move played")
            };

            let move_score = match round[..] {
                [Move::Rock, Move::Y] => 6,
                [Move::Paper, Move::Z] => 6,
                [Move::Scissor, Move::X] => 6,

                [Move::Rock, Move::X] => 3,
                [Move::Paper, Move::Y] => 3,
                [Move::Scissor, Move::Z] => 3,

                [_, _] => 0,

                _ => panic!("Invalid move played")
            };

            self_score + move_score
        })
        .sum::<u32>();

    let score_two: u32 = rounds
        .iter()
        .map(|round| {
            match &round[..] {
                // Move::X means we need to lose.
                [choice, Move::X] => {
                    let move_score = match choice {
                        Move::Rock => 3,
                        Move::Paper => 1,
                        Move::Scissor => 2,
                        _ => panic!("No score for this move")
                    };

                    0 + move_score
                },
                // Move::Y means we need to draw.
                [choice, Move::Y] => {
                    let move_score = match choice {
                        Move::Rock => 1,
                        Move::Paper => 2,
                        Move::Scissor => 3,
                        _ => panic!("No score for this move")
                    };

                    3 + move_score
                },
                // Move::Z means we need to win.
                [choice, Move::Z] => {
                    let move_score = match choice {
                        Move::Rock => 2,
                        Move::Paper => 3,
                        Move::Scissor => 1,
                        _ => panic!("No score for this move")
                    };

                    6 + move_score
                },
                _ => panic!("Invalid move played")
            }
        })
        .sum::<u32>();

    println!("Part 1: {}", score_one);
    println!("Part 2: {}", score_two);
}
