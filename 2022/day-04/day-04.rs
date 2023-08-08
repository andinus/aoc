use std::fs;

fn main() {
    let file = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    let lines = file
        .trim()
        .lines();

    // pairs holds list of all assignments for each partner.
    let mut pairs: Vec<Vec<Vec<u32>>> = lines
        .map(|assignment| {
            assignment
                .split(",")
                .map(|x|
                     x
                     .split("-")
                     .map(|i| i.parse::<u32>().unwrap())
                     .collect::<Vec<u32>>()
                )
                .collect::<Vec<Vec<u32>>>()
        })
        .collect();

    // For every pair we sort partners by their starting section.
    pairs
        .iter_mut()
        .for_each(
            |partners| partners.sort_unstable_by_key(|section| section[0])
        );

    // Because we sorted every partner by their starting section, we know that
    // first partner's start section is going to be more than or equal to the
    // second partner's start section. So we only need to check if second
    // partner's sections are bounded by first partner's sections.
    let result_one = pairs
        .iter()
        .fold(0, |result, partners| {
            let first: &Vec<u32> = &partners[0];
            let second: &Vec<u32> = &partners[1];

            // For one partner's section to contain the other's, either the
            // second's end section should be <= first's end section. Or if
            // first's & second's start section are same then we need to check
            // if first's end section <= second's end section.
            result + (second[1] <= first[1]
                      || (first[0] == second[0] && first[1] <= second[1])) as u32
        });


    let result_two = pairs
        .iter()
        .fold(0, |result, partners| {
            let first: &Vec<u32> = &partners[0];
            let second: &Vec<u32> = &partners[1];

            // For one partner's section to not overlap the other's, second's
            // start section must be > first's end section, anything else means
            // that they overlap.
            result + (second[0] <= first[1]) as u32
        });


    println!("Part 1: {}", result_one);
    println!("Part 2: {}", result_two);
}
