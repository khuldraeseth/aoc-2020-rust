use std::io;
use std::io::BufRead;
use std::vec;
use regex::Regex;

fn is_passport(s: String) -> bool {
    let fields: Vec<String> =
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    return fields.iter()
        .map(|s| ".*".to_owned() + s + ".*")
        .map(|s| Regex::new(&s).unwrap())
        .map(|re| re.is_match(&s))
        .fold(true, |p, q| p && q);
}

fn is_passport_prime(s: String) -> bool {
    let regexen = vec!
        [ ".*byr:(19[2-9][0-9]|200[0-2]) .*"
        , ".*iyr:(201[0-9]|2020) .*"
        , ".*eyr:(202[0-9]|2030) .*"
        , ".*hgt:((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in) .*"
        , ".*hcl:#[0-9a-f]{6} .*"
        , ".*ecl:(amb|blu|brn|gry|grn|hzl|oth) .*"
        , ".*pid:[0-9]{9} .*"
        ];
    
    return regexen.iter()
        //.map(|s| s.to_string())
        .map(|s| ".*".to_owned() + s + ".*")
        .map(|s| Regex::new(&s).unwrap())
        .map(|re| re.is_match(&s))
        .fold(true, |p, q| p && q);
}

fn foo(lines: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut elem = "".to_string();
    for line in lines {
        elem += " ";
        if line == "" {
            println!("{}", elem);
            result.push("".to_owned() + &elem);
            elem.clear();
        }
        elem += &line;
    }
    return result;
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock()
        .lines()
        .map(|res| res.unwrap())
        .collect::<Vec<String>>();
    
    let passports = foo(lines);
    
    let mut count = 0;
    for p in passports {
        if is_passport_prime(p) {
            count += 1;
        }
    }

    println!("{}", count);
}
