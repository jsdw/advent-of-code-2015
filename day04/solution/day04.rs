extern crate crypto;

use std::env;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {

    let input = env::args().nth(1).expect("need puzzle input");

    let first_n = (1..).filter(|n| count_zeros(&format!("{}{}", &input, n)) >= 5).next().unwrap();
    println!("Star 1: {}", first_n);

    let second_n = (1..).filter(|n| count_zeros(&format!("{}{}", &input, n)) >= 6).next().unwrap();
    println!("Star 2: {}", second_n);

}

fn count_zeros(input: &str) -> usize{
    let mut hash = Md5::new();
    hash.input_str(input);
    hash.result_str().chars().take_while(|&c| c == '0').count()
}
