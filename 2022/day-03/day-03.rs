use std::fs;
use std::collections::HashSet;

fn main() {
    let file = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    let priority_one: u32 = file
        .lines()
        .map(|runsack| {
            let (first, second) = runsack.split_at(runsack.len() / 2);

            let first_compartment: HashSet<char> = first.chars().collect();
            let second_compartment: HashSet<char> = second.chars().collect();

            first_compartment
                .intersection(&second_compartment)
                .map(|item| {
                    match item {
                        'a'..='z' => *item as u32 - 'a' as u32 + 1,
                        'A'..='Z' => *item as u32 - 'A' as u32 + 26 + 1,
                        _ => unreachable!("Invalid item: {}", item)
                        // _ => panic!(),
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    let mut lines = file.lines().peekable();
    let mut priority_two: u32 = 0;
    while lines.peek().is_some() && !lines.peek().unwrap().is_empty() {
        let first: HashSet<char> = lines.next().unwrap().chars().collect();
        let second: HashSet<char> = lines.next().unwrap().chars().collect();
        let third: HashSet<char> = lines.next().unwrap().chars().collect();

        let intermediate: HashSet<char> = first.intersection(&second).cloned().collect();
        priority_two += intermediate
            .intersection(&third)
            .map(|item| {
                match item {
                    'a'..='z' => *item as u32 - 'a' as u32 + 1,
                    'A'..='Z' => *item as u32 - 'A' as u32 + 26 + 1,
                    _ => panic!("Invalid item: {}", item),
                }
            })
            .sum::<u32>();
    }

    println!("Part 1: {priority_one}");
    println!("Part 2: {priority_two}");

}
