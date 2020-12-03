use std::io;
use std::io::BufRead;
//use std::vec;

fn main() {
//    let mut input = String::new();
    let stdin = io::stdin();
    let nums = stdin.lock()
        .lines()
        .map(|s| s.unwrap().trim().parse::<i32>().unwrap());
    
    let mut xs = Vec::new();
    for x in nums {
        xs.push(x);
    }    

    for x in &xs {
        for y in &xs {
            if x + y == 2020 {
                println!("{} {} {}", x, y, x*y);
            }
        }
    }

    println!();

    for x in &xs {
        for y in &xs {
            for z in &xs {
                if x + y + z == 2020 {
                    println!("{} {} {} {}", x, y, z, x*y*z);
                }
            }
        }
    }
}
