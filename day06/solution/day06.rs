use std::env;
use std::fs;
use std::io::Read;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let commands: Vec<Command> = content.lines().map(parse_command).collect();

    let mut lights: Lights1 = [[false;1000];1000];
    for cmd in &commands {
        step1(cmd, &mut lights);
    }

    let on_count: usize = lights.iter().map(|l| l.iter().filter(|b| **b).count()).sum();
    println!("Star 1: {:?}", on_count);

    let mut lights: Lights2 = [[0;1000];1000];
    for cmd in &commands {
        step2(cmd, &mut lights);
    }

    let brightness: u32 = lights.iter().map(|l| { let b: u32 = l.iter().sum(); b }).sum();
    println!("Star 2: {:?}", brightness);

}

fn step1(cmd: &Command, grid: &mut Lights1) {
    match *cmd {
        Command::On(start,end) => {
            for Coord{x,y} in Rectangle::at(start,end) {
                grid[x][y] = true;
            }
        },
        Command::Off(start,end) => {
            for Coord{x,y} in Rectangle::at(start,end) {
                grid[x][y] = false;
            }
        },
        Command::Toggle(start,end) => {
            for Coord{x,y} in Rectangle::at(start,end) {
                grid[x][y] = !grid[x][y];
            }
        }
    }
}

fn step2(cmd: &Command, grid: &mut Lights2) {
    match *cmd {
        Command::On(start,end) => {
            for Coord{x,y} in Rectangle::at(start,end) {
                grid[x][y] += 1;
            }
        },
        Command::Off(start,end) => {
            for Coord{x,y} in Rectangle::at(start,end) {
                if grid[x][y] > 0 { grid[x][y] -= 1 };
            }
        },
        Command::Toggle(start,end) => {
            for Coord{x,y} in Rectangle::at(start,end) {
                grid[x][y] += 2;
            }
        }
    }
}

fn parse_command(s: &str) -> Command {
    let on_prefix = "turn on ";
    let off_prefix = "turn off ";
    let toggle_prefix = "toggle ";

    if s.starts_with(on_prefix) {
        let (c1,c2) = parse_coords( s.trim_left_matches(on_prefix) );
        Command::On(c1,c2)
    } else if s.starts_with(off_prefix) {
        let (c1,c2) = parse_coords( s.trim_left_matches(off_prefix) );
        Command::Off(c1,c2)
    } else if s.starts_with(toggle_prefix) {
        let (c1,c2) = parse_coords( s.trim_left_matches(toggle_prefix) );
        Command::Toggle(c1,c2)
    } else {
        panic!("Command not recognised: {}", s)
    }
}

fn parse_coords(s: &str) -> (Coord,Coord) {
    let mut both = s.trim().split(" through ").map(parse_coord);
    (both.next().unwrap(), both.next().unwrap())
}

fn parse_coord(s: &str) -> Coord {
    let mut cs = s.split(',').map(|n| n.parse().unwrap());
    Coord{ x: cs.next().unwrap(), y: cs.next().unwrap() }
}

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
enum Command {
    On(Coord,Coord),
    Off(Coord,Coord),
    Toggle(Coord,Coord)
}

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
struct Coord {
    x: usize,
    y: usize
}

struct Rectangle {
    start: Coord,
    end: Coord,
    curr: Coord
}
impl Rectangle {
    fn at(start: Coord, end: Coord) -> Rectangle {
        Rectangle { start, end, curr: start}
    }
}
impl Iterator for Rectangle {
    type Item = Coord;
    fn next(&mut self) -> Option<Coord> {
        let Coord{x,y} = self.curr;

        if x < self.end.x {
            self.curr.x += 1;
        } else if y <= self.end.y {
            self.curr.x = self.start.x;
            self.curr.y += 1;
        }

        if y > self.end.y { None }
        else { Some(Coord{x,y}) }
    }
}

type Lights1 = [[bool;1000];1000];
type Lights2 = [[u32;1000];1000];