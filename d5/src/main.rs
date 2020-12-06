use std::io;
use std::io::BufRead;

fn weird(s: &str) -> i32 {
    return match s
        { "F" => 0
        , "B" => 1
        , _   => 0
        };
}

fn from_weird_base(s: &str) -> i32 {
    if s == "" {
        return 0;
    }
    return weird(&s[s.len()-1..]) + (2 * from_weird_base(&s[..s.len()-1]));
}

fn weird_prime(s: &str) -> i32 {
    return match s
        { "L" => 0
        , "R" => 1
        , _   => 0
        };
}

fn from_weird_base_prime(s: &str) -> i32 {
    if s == "" {
        return 0;
    }
    return weird_prime(&s[s.len()-1..]) + (2 * from_weird_base_prime(&s[..s.len()-1]));
}

fn foo(s: String) -> i32 {
    return (8 * from_weird_base(&s[..7])) + from_weird_base_prime(&s[7..]);
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock()
        .lines()
        .map(|x| x.unwrap());

    let mut vals = lines.map(foo).collect::<Vec<i32>>();
    vals.sort();

    let mut prev = -1;
    for id in vals {
        if id == prev + 2 {
            println!("{}", id - 1);
        }
        prev = id;
    }

    //println!("{}", vals.max().unwrap());
}
