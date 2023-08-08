use std::fs;

#[derive(Debug)]
struct Move {
    crates: usize,
    from: usize,
    to: usize,
}

fn main() {
    let file = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    let data: Vec<Vec<&str>> = file
        .trim()
        .split("\n\n")
        .map(|x| x.split("\n").collect())
        .collect();

    // moves will hold the ordered moves, we'll cast each line to be of type
    // Move.
    let moves: Vec<Move> = data[1]
        .iter()
        .map(|line| {
            // sample line: "move 1 from 2 to 1"
            let item: Vec<&str> = line.split(" ").collect();

            Move {
                crates: item[1].parse::<usize>().unwrap(),

                to:     item[5].parse::<usize>().unwrap() - 1,
                from:   item[3].parse::<usize>().unwrap() - 1,
            }
        })
        .collect();

    // stacks will store the stacks, 0th element will contain all crates of
    // stack 0.
    let mut stacks: Vec<Vec<char>> = vec![];

    // Below is the input we have, crates_intermediate holds array of array of
    // chars where the outer array holds each line and inner array holds each
    // char of the line.
    //
    //     [D]
    // [N] [C]
    // [Z] [M] [P]
    //  1   2   3
    let crates_intermediate: Vec<Vec<char>> = data[0]
        .iter()
        .rev()
        .map(|line| line.chars().collect())
        .collect();

    // We need to loop over crates_intermediate and read columns out of
    // Vec<Vec<char>> and get only Vec<char>. Essentially we need to perform a
    // matrix transposition, get relevant rows and make Vec<char> out of them.
    for (idx, c) in crates_intermediate[0].iter().enumerate() {
        if *c == ' ' {
            continue;
        }

        // If we reach here then we expect this character to be an integer,
        // though we are not concerned about it, we'll simply extract all the
        // elements from other rows from a specific index and store them in
        // crates.
        stacks.push(
            crates_intermediate[1..]
                .iter()
                .filter_map(|line| {
                    // From each line we are going to extract a single element at
                    // idx if it exists at all and we don't need whitespaces.
                    if idx < line.len() && line[idx] != ' ' {
                        Some(line[idx])
                    } else {
                        None
                    }
                })
                .collect::<Vec<char>>()
        );
    }


    let result_one: String = crate_mover_9000(stacks.clone(), &moves)
        .iter()
        .filter_map(|stack| stack.last())
        .collect();

    let result_two: String = crate_mover_9001(stacks.clone(), &moves)
        .iter()
        .filter_map(|stack| stack.last())
        .collect();

    println!("Part 1: {}", result_one);
    println!("Part 2: {}", result_two);
}

// crate_mover_9000 takes in stacks, moves and applies the moves to it.
fn crate_mover_9000(mut stacks: Vec<Vec<char>>, moves: &Vec<Move>) -> Vec<Vec<char>> {
    // Go through the moves and move the crates around. (CrateMover 9000)
    for m in moves {
        for _ in 1..=m.crates {
            let c: char = stacks[m.from].pop().unwrap();
            stacks[m.to].push(c);
        }
    }

    stacks
}

// crate_mover_9001 takes in stacks, moves and applies the moves to it.
fn crate_mover_9001(mut stacks: Vec<Vec<char>>, moves: &Vec<Move>) -> Vec<Vec<char>> {
    // Go through the moves and move the crates around. (CrateMover 9001)
    for m in moves {
        let from = &mut stacks[m.from];
        let c: Vec<char> = from.split_off(from.len() - m.crates);

        stacks[m.to].extend(c);
    }

    stacks
}
