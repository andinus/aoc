use std::fs;

fn main() {
    let file = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    // input holds the given instructions, first value in tuple indicates the
    // number of clock cycles an operation takes and the second, the value by
    // which the register must be incremented after the instruction.
    let instructions: Vec<(u8, isize)> = file
        .trim()
        .lines()
        .map(|l| {
            let (instruction, value) = l.split_at(4);

            match instruction {
                "noop" => (1, 0),
                "addx" => (2, value.trim().parse::<isize>().unwrap()),
                _ => unreachable!()
            }
        })
        .collect();

    // x holds register x value.
    let mut x: isize = 1;

    // cycles holds the register values during every clock cycle.
    let mut cycles: Vec<isize> = vec![1];
    for (clock, value) in instructions {
        cycles.extend(std::iter::repeat(x).take(clock as usize));
        x += value;
    }

    let part_one: isize = cycles
        .iter()
        .enumerate()
        .filter(|(i, _)| (*i == 20 || (*i > 40 && (*i - 20) % 40 == 0)))
        .fold(0, |acc, (i, &v)| acc + (v * (i as isize)));

    println!("Part 1: {}", part_one);

    println!("Part 2:");
    for (i, v) in cycles.iter().enumerate().skip(1) {
        let p = (i - 1) % 40;

        // v here controls the middle position of the sprite, the sprite spans 3
        // columns, i here is the cycle and p represents the pixel CRT is
        // drawing in, which is i - 1.
        if matches!((p as isize - v).abs(), 0 | 1) {
            print!("▓");
        } else {
            print!("░");
        }

        if i % 40 == 0 {
            print!("\n");
        }
    }
}
