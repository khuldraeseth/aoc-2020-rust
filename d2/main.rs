use std::io;
use std::io::BufRead;

struct Foo {
    lo: i32,
    hi: i32,
    c: char,
    s: String,
}

fn mk_foo(s: String) -> Foo {
    let pieces: Vec<&str> = s.split(" ").collect();
    let nums: Vec<i32> = pieces[0]
        .split("-")
        .map(|ds| ds.parse::<i32>().unwrap())
        .collect();

    return Foo { lo: nums[0], hi: nums[1], c: pieces[1].chars().nth(0).unwrap(), s: String::from(pieces[2]) };
}

fn count_if<F,Ts>(f: F, xs: Ts) -> i32
where F: Fn(Ts::Item) -> bool,
      Ts: Iterator {
    return xs.map(|x| match f(x) { false => 0, true => 1 }).sum();
}

fn count(y: char, xs: String) -> i32 {
    return count_if(|c| c == y, xs.chars());
}

fn is_legit(foo: Foo) -> bool {
    let n = count(foo.c, foo.s);
    return foo.lo <= n && n <= foo.hi;
}

fn is_legit_prime(foo: Foo) -> bool {
    return (foo.s.as_bytes()[(foo.lo-1) as usize] as char == foo.c)
         ^ (foo.s.as_bytes()[(foo.hi-1) as usize] as char == foo.c);
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let foos = lines.map(|s| mk_foo(s.unwrap()));

    println!("{}", count_if(is_legit_prime, foos));
}
