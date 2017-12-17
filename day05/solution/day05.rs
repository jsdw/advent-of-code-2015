use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashMap;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let good_lines = content.lines()
        .filter(|&s| at_least_three_vowels(s))
        .filter(|&s| no_bad_pairs(s))
        .filter(|&s| twice_in_a_row(s))
        .count();
    println!("Star 1: {}", good_lines);

    let good_lines2 = content.lines()
        .filter(|&s| letter_sandwich(s))
        .filter(|&s| two_pairs(s))
        .count();
    println!("Star 2: {}", good_lines2);
}

fn at_least_three_vowels(s: &str) -> bool {
    s.chars()
        .filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u').count() >= 3
}

fn no_bad_pairs(s: &str) -> bool {
    s.chars()
        .zip(s.chars().skip(1))
        .filter(|&(a,b)| (a=='a'&&b=='b') || (a=='c'&&b=='d') || (a=='p'&&b=='q') || (a=='x'&&b=='y')).count() == 0
}

fn twice_in_a_row(s: &str) -> bool {
    s.chars()
        .zip(s.chars().skip(1))
        .filter(|&(a,b)| a==b).count() >= 1
}

fn letter_sandwich(s: &str) -> bool {
    s.as_bytes().windows(3).filter(|s| s[0] == s[2] && s[0] != s[1]).count() > 0
}

fn two_pairs(s: &str) -> bool {
    for (idx,c) in s.as_bytes().windows(2).enumerate() {
        if s[idx+2 ..].as_bytes().windows(2).find(|&c2| c == c2).is_some() {
            return true
        }
    }
    return false
}