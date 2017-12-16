use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashSet;
use self::Direction::*;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let dirs: Vec<Direction> = content.chars().map(parse_direction).collect();

    // Star 1
    let seen: HashSet<(isize,isize)> = to_coords(&dirs).iter().cloned().collect();
    println!("Star 1: {}", seen.iter().count());

    // Star 2
    let mut santa = vec![];
    let mut robo_santa = vec![];
    for (idx, &dir) in dirs.iter().enumerate() {
        if idx % 2 == 0 {
            santa.push(dir)
        } else {
            robo_santa.push(dir)
        }
    }
    let seen: HashSet<(isize,isize)> = to_coords(&santa).iter().chain(to_coords(&robo_santa).iter()).cloned().collect();
    println!("Star 2: {}", seen.iter().count());

}

fn to_coords(dirs: &[Direction]) -> Vec<(isize,isize)> {
    let mut coords = vec![(0,0)];
    for (last,&dir) in dirs.iter().enumerate() {
        let (x,y) = coords[last];
        coords.push(match dir {
            Up    => (x, y+1),
            Right => (x+1, y),
            Down  => (x, y-1),
            Left  => (x-1, y)
        });
    }
    coords
}

fn parse_direction(dir: char) -> Direction {
    match dir {
        '^' => Up,
        '>' => Right,
        'v' => Down,
        _ => Left
    }
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Direction {
    Up, Right, Down, Left
}
