use std::fs;
use std::collections::HashMap;

fn main() {
    let file = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    let buffer: Vec<char> = file.trim().chars().collect();
    println!("Part 1: {}", start_of_distinct_packet(&buffer, 4));
    println!("Part 2: {}", start_of_distinct_packet(&buffer, 14));
}

// start_of_distinct_packet takes in datstream buffer, n and returns the index
// of when n distinct packets are detected.
fn start_of_distinct_packet(buffer: &Vec<char>, n: usize) -> usize {
    // seen will record the packets that have been seen so far and their last
    // seen index.
    let mut seen: HashMap<char, usize> = HashMap::new();

    // i indexes over the buffer, it'll hold the start of marker eventually.
    let mut i = 0;

    // parsed holds the number of characters that have been parsed.
    let mut parsed = 0;
    while i < buffer.len() {
        // if n characters have been parsed then we've found our marker at i,
        // end the loop.
        if parsed == n {
            break;
        }

        // if the character has been seen already then we clear the map, make
        // parsed = 0 and move forward to the index that character was
        // previously seen at because until that character is included we're not
        // going to be able to find start of marker anyways because there will
        // be at least one duplicate (the character itself).
        match seen.get(&buffer[i]) {
            Some(idx) => {
                i = *idx;

                parsed = 0;
                seen.clear();
            },
            None => {
                seen.insert(buffer[i], i);
                parsed += 1;
            }
        }

        i += 1;
    }

    i
}
