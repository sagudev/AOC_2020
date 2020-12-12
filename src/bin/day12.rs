use aoc::read_data;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

/// Vsi ukazi
#[derive(Debug)]
enum Cmd {
    /// na sever
    North,
    /// na jug
    South,
    /// na vzhod
    East,
    /// na zahod
    West,
    /// obrni levo
    L,
    /// obrni desno
    R,
    /// pojdi naravnost v trenutni smeri
    F,
}

impl FromStr for Cmd {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Cmd::North),
            "S" => Ok(Cmd::South),
            "E" => Ok(Cmd::East),
            "W" => Ok(Cmd::West),
            "L" => Ok(Cmd::L),
            "R" => Ok(Cmd::R),
            "F" => Ok(Cmd::F),
            _ => panic!("wrong cmd"),
        }
    }
}

#[derive(Debug)]
struct Move(Cmd, usize);

impl FromStr for Move {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Move(s[0..1].parse()?, s[1..].parse()?))
    }
}

/// The ship starts by facing east.
struct Boat {
    north: i32,
    east: i32,
    rotation: i32,
}

impl Boat {
    fn new() -> Self {
        Self {
            north: 0,
            east: 0,
            rotation: 0,
        }
    }
    fn mover(&mut self, m: &Move) -> (i32, i32) {
        match m {
            Move(Cmd::North, x) => self.north += *x as i32,
            Move(Cmd::South, x) => self.north -= *x as i32,
            Move(Cmd::East, x) => self.east += *x as i32,
            Move(Cmd::West, x) => self.east -= *x as i32,
            Move(Cmd::L, x) => self.rotation += 360 - *x as i32,
            Move(Cmd::R, x) => self.rotation += *x as i32,
            Move(Cmd::F, x) => match self.rotation {
                0 => self.east += *x as i32,
                90 => self.north -= *x as i32,
                180 => self.east -= *x as i32,
                270 => self.north += *x as i32,
                _ => panic!("rotation"),
            },
        };
        if self.rotation >= 360 {
            self.rotation -= 360;
        }
        (self.east, self.north)
    }
    fn do_mover(&mut self, m: &[Move]) -> (i32, i32) {
        for d in m {
            self.mover(d);
        }
        (self.east, self.north)
    }
}

fn p1(data: &[Move]) -> i32 {
    let mut b = Boat::new();
    let m = b.do_mover(&data);
    m.0.abs() + m.1.abs()
}

struct Boat2 {
    north: i32,
    east: i32,
    w_north: i32,
    w_east: i32,
}

impl Boat2 {
    fn new() -> Self {
        Self {
            north: 0,
            east: 0,
            w_north: 1,
            w_east: 10,
        }
    }
    fn rotate(&mut self, rotation: usize) {
        let n = self.w_north;
        let e = self.w_east;
        match rotation {
            //0 => self.east += *x as i32,
            90 => {
                self.w_north = -e;
                self.w_east = n;
            }
            180 => {
                self.w_north = -n;
                self.w_east = -e;
            }
            270 => {
                self.w_north = e;
                self.w_east = -n;
            }
            _ => panic!("rotation"),
        }
    }
    fn mover(&mut self, m: &Move) -> ((i32, i32), (i32, i32)) {
        match m {
            Move(Cmd::North, x) => self.w_north += *x as i32,
            Move(Cmd::South, x) => self.w_north -= *x as i32,
            Move(Cmd::East, x) => self.w_east += *x as i32,
            Move(Cmd::West, x) => self.w_east -= *x as i32,
            Move(Cmd::L, x) => self.rotate(360 - *x),
            Move(Cmd::R, x) => self.rotate(*x),
            Move(Cmd::F, x) => {
                self.north += self.w_north * *x as i32;
                self.east += self.w_east * *x as i32;
            }
        };
        ((self.east, self.north), (self.w_east, self.w_north))
    }
    fn do_mover(&mut self, m: &[Move]) -> (i32, i32) {
        for d in m {
            self.mover(d);
        }
        (self.east, self.north)
    }
}

fn p2(data: &[Move]) -> i32 {
    let mut b = Boat2::new();
    let m = b.do_mover(&data);
    m.0.abs() + m.1.abs()
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<Move> = read_data("./data/data12").unwrap();
    // part 1
    println!(
        "What is the Manhattan distance between that location and the ship's starting position? {}",
        p1(&data)
    );
    // part 2
    println!(
        "What is the Manhattan distance between that location and the ship's starting position? {}",
        p2(&data)
    );
    Ok(())
}

#[test]
fn data_read() {
    println!("{:?}", read_data::<Move>("./data/data12").unwrap());
}

#[test]
fn calc() {
    let data = vec!["F10", "N3", "F7", "R90", "F11"];
    let data: Vec<Move> = data.iter().map(|line| line.parse().unwrap()).collect();
    // part 1
    let mut b = Boat::new();
    assert_eq!(b.mover(&data[0]), (10, 0));
    assert_eq!(b.mover(&data[1]), (10, 3));
    assert_eq!(b.mover(&data[2]), (17, 3));
    assert_eq!(b.mover(&data[3]), (17, 3));
    assert_eq!(b.mover(&data[4]), (17, -8));
    assert_eq!(p1(&data), 25);
    // part 2
    let mut b = Boat2::new();
    assert_eq!(b.mover(&data[0]), ((100, 10), (10, 1)));
    assert_eq!(b.mover(&data[1]), ((100, 10), (10, 4)));
    assert_eq!(b.mover(&data[2]), ((170, 38), (10, 4)));
    assert_eq!(b.mover(&data[3]), ((170, 38), (4, -10)));
    assert_eq!(b.mover(&data[4]), ((214, -72), (4, -10)));
    assert_eq!(p2(&data), 286);
}
