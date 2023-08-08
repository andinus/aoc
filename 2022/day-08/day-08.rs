use std::fs;

fn main() {
    let file = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    // trees is a vector of vectors of u32 where each item represent tree
    // height.
    let trees: Vec<Vec<u32>> = file
        .trim()
        .lines()
        .map(|l| {
            l
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        }).collect();


    // visibility will hold the visibility of corresponding elements in trees.
    // we're assuming that trees is rectangular shape.
    let mut visibility: Vec<Vec<bool>> = vec![vec![false; trees[0].len()]; trees.len()];

    // mark first & last row as visibile.
    visibility[0].fill(true);
    visibility[trees.len() - 1].fill(true);

    // mark first & last columns as visible.
    for y in 0..trees.len() {
        visibility[y][0] = true;
        visibility[y][trees[y].len() - 1] = true;
    }

    let mut visibility_top: Vec<u32> = vec![0; trees.len()];
    let mut visibility_left: Vec<u32> = vec![0; trees[0].len()];

    // We're going to move from left to right, top to bottom and keep records of
    // the maximum height of tree that we're looking at in that specific column
    // & row. Based on that we're going to be able to compute all the visible
    // trees (from top, left) in one pass. We can use another pass for trees
    // visible from bottom, right.
    for y in 0..trees.len() {
        for x in 0..trees[y].len() {
            let height = trees[y][x];

            // check if it's visible from left, top.
            if height > visibility_left[y] {
                visibility_left[y] = height;
                visibility[y][x] = true;
            }

            if height > visibility_top[x] {
                visibility_top[x] = height;
                visibility[y][x] = true;
            }
        }
    }

    let mut visibility_bottom: Vec<u32> = vec![0; trees.len()];
    let mut visibility_right: Vec<u32> = vec![0; trees[0].len()];

    for y in (0..trees.len()).rev() {
        for x in (0..trees[y].len()).rev() {
            let height = trees[y][x];

            // check if it's visible from right, bottom
            if height > visibility_right[y] {
                visibility_right[y] = height;
                visibility[y][x] = true;
            }

            if height > visibility_bottom[x] {
                visibility_bottom[x] = height;
                visibility[y][x] = true;
            }
        }
    }

    // count of visible trees.
    let visible = visibility.iter().flatten().filter(|&x| *x).count();
    println!("Part 1: {}", visible);

    let mut max_scenic_score: u32 = 1;
    for y in 1..trees.len() {
        for x in 1..trees[y].len() {
            let height = trees[y][x];

            // calculate scenic score for each direction.
            let mut right_score = 0;
            for h in trees[y][x..].iter().skip(1) {
                right_score += 1;

                if !(*h < height) { break; }
            }

            let mut left_score = 0;
            for h in trees[y][..x].iter().rev() {
                left_score += 1;

                if !(*h < height) { break; }
            }

            let mut top_score = 0;
            for row in trees[..y].iter().rev() {
                top_score += 1;

                if !(row[x] < height) { break; }
            }

            let mut bottom_score = 0;
            for row in trees[y..].iter().skip(1) {
                bottom_score += 1;

                if !(row[x] < height) { break; }
            }

            let scenic_score = right_score * left_score
                * top_score * bottom_score;

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    println!("Part 2: {}", max_scenic_score);
}
