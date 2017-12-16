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

    let dims: Vec<Dimensions> = content.lines().filter_map(parse_dimensions).collect();

    let paper_area: usize = dims.iter().map(|&d| d.surface_area() + d.smallest_area()).sum();
    println!("Star 1: {}", paper_area);

    let ribbon_length: usize = dims.iter().map(|&d| d.ribbon_length()).sum();
    println!("Star 2: {}", ribbon_length);

}

fn parse_dimensions(dims: &str) -> Option<Dimensions> {
    let mut ns = dims.split('x');
    let length = ns.next()?.parse().ok()?;
    let width = ns.next()?.parse().ok()?;
    let height = ns.next()?.parse().ok()?;
    Some(Dimensions { length, width, height })
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct Dimensions {
    length: usize,
    width: usize,
    height: usize
}

impl Dimensions {
    fn surface_area(&self) -> usize {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }
    fn smallest_area(&self) -> usize {
        let mut v = vec![self.length, self.width, self.height];
        v.sort();
        v[0] * v[1]
    }
    fn ribbon_length(&self) -> usize {
        let mut v = vec![self.length, self.width, self.height];
        v.sort();
        v[0] * 2 + v[1] * 2 + v[0] * v[1] * v[2]
    }
}

