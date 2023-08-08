use std::fs;
use std::collections::HashMap;

fn main() {
    let file = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    // lines collect all the lines from input, we're not concerned with ls
    // commands.
    let lines = file
        .trim()
        .lines()
        .filter(|l| !(l.starts_with("$ ls") || l.starts_with("dir ")))
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    // filesystem hash map stores the dir and it's size.
    let mut filesystem: HashMap<Vec<String>, usize> = HashMap::new();

    // cwd stores the current working directory.
    let mut cwd: Vec<String> = vec![ "/".to_string() ];

    for l in &lines {
        match l[0] {
            "$" => {
                let (command, dir) = (l[1], l[2]);
                if command != "cd" {
                    panic!("Unsupported command");
                }

                // for cd we're expecting one argument as dir.
                match dir {
                    "/"  => { cwd.drain(1..); }
                    ".." => { cwd.pop();      },
                    _ =>    { cwd.push(dir.to_string());  }
                }
            },
            _ => {
                // listing can be in this format "<size> file".
                let size = l[0].parse::<usize>().unwrap();

                for i in 0..cwd.len() {
                    *filesystem.entry( (cwd[0..=i]).to_vec() ).or_insert(0) += size;
                }
            }
        }
    }

    let result_one: usize = filesystem
        .values()
        .filter(|&&v| v <= 100000)
        .fold(0, |result, v| result + v);

    println!("Part 1: {}", result_one);

    let total_disk_space = 70_000_000;
    let min_required_space = 30_000_000;

    let free_space = total_disk_space
        - *filesystem.entry(vec![ "/".to_string() ]).or_default();

    let required_space: usize = min_required_space - free_space;

    let mut cleanup_directories: Vec<usize> = filesystem
        .values()
        .filter(|&&v| v >= required_space)
        .map(|path| *path)
        .collect();

    cleanup_directories.sort_unstable();

    let result_two: usize = cleanup_directories[0];
    println!("Part 2: {}", result_two);
}
