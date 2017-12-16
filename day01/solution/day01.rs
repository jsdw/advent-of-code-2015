use std::env;
use std::fs;
use std::io::Read;
use self::Move::*;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let moves: Vec<Move> = content.chars().map(|c| if c == '(' { Up } else { Down }).collect();

    let floor = moves.iter().fold(0, |n, &m| if m == Up { n+1 } else { n-1 });
    println!("Star 1: {}", floor);

    let floors = moves.iter().scan(0, |n, &m| { if m == Up { *n += 1 } else { *n -= 1 }; Some(*n) });
    println!("Star 2: {}", floors.take_while(|&n| n >= 0).count() + 1);

}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
enum Move { Up, Down }

