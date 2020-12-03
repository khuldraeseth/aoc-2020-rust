use std::io;
use std::io::BufRead;
/*
fn read_line(line: String) -> Vec<bool> {
    let mut result = Vec::new();
    for c in line.chars() {
        result.push(c == '#');
    }
    return result;
}
*/
fn read_map_io() -> Vec<Vec<bool>> {
    let stdin = io::stdin();
    return stdin.lock()
        .lines()
        .map(|line| line.unwrap().chars().map(|c| c == '#').collect())
        .collect();
}

fn main() {
    let map = read_map_io();

    let mut num_trees = 0;
    for i in 1 .. map.len() {
        if i%2 == 0 && map[i][(i/2) % map[i].len()] {
            num_trees += 1;
            //println!("{} {}", i, (3*i) % map[i].len());
        }
    }

    println!("{}", num_trees);
}
