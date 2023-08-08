use std::fs;

fn main() {
    let list = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    let mut calories: Vec<u32> = list
        .split("\n\n")
        .map(
            |elf_inventory| {
                elf_inventory
                    .lines()
                    .map(|i| i.parse::<u32>().unwrap())
                    .sum::<u32>()
            }
        )
        .collect();

    calories.sort_unstable();
    calories.reverse();

    println!("Part 1: {}", calories[0]);
    println!("Part 2: {}", calories.iter().take(3).sum::<u32>());
}
