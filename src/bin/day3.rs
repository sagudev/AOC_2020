use aoc::read_data;
use std::convert::Infallible;
use std::error::Error;
use std::str::FromStr;

/// top-left (0,0)
struct Position(usize, usize);

impl Position {
    fn walk(&self, data: &[Iline]) -> Obj {
        data[self.1].get(self.0)
    }
    // walk_by (right, down)
    fn walk_by(&mut self, right: usize, down: usize) {
        self.0 += right;
        self.1 += down;
    }
}

#[derive(PartialEq, Clone)]
enum Obj {
    Tree,
    Empty,
}

impl Obj {
    fn from_char(s: char) -> Self {
        if s == '.' {
            Obj::Empty
        } else if s == '#' {
            Obj::Tree
        } else {
            panic!("I do not know how to read")
        }
    }
    fn is_tree(&self) -> bool {
        *self == Obj::Tree
    }
}

struct Iline {
    line: Vec<Obj>,
}

impl Iline {
    fn get(&self, idx: usize) -> Obj {
        let mut num = idx;
        let len = self.line.len();
        if idx > len {
            for _ in 0..((idx as f64 / len as f64).floor() as i64) {
                num -= len
            }
        }
        self.line[num].clone()
    }
}

impl FromStr for Iline {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v: Vec<Obj> = Vec::new();
        for el in s.chars() {
            v.push(Obj::from_char(el))
        }
        Ok(Self { line: v })
    }
}

fn step(data: &[Iline], position: &mut Position, right: usize, down: usize) -> bool {
    (*position).walk_by(right, down);
    position.walk(data).is_tree()
}

/// Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?
fn find_trees(data: &[Iline], right: usize, down: usize) -> usize {
    let mut position = Position(0, 0);
    let mut trees = 0;
    while position.1 < data.len() - 1 {
        if step(&data, &mut position, right, down) {
            trees += 1
        }
    }
    trees
}

fn p1(data: &[Iline]) -> usize {
    find_trees(data, 3, 1)
}

fn p2(data: &[Iline]) -> usize {
    find_trees(&data, 1, 1)
        * find_trees(&data, 3, 1)
        * find_trees(&data, 5, 1)
        * find_trees(&data, 7, 1)
        * find_trees(&data, 1, 2)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    // part 1
    let data: Vec<Iline> = read_data("./data/data3").unwrap();
    println!("Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?: {}", p1(&data));
    //part 2
    println!("What do you get if you multiply together the number of trees encountered on each of the listed slopes?: {}", p2(&data));
    Ok(())
}

#[test]
fn data_read() {
    println!("{:?}", read_data::<String>("./data/data3").unwrap());
}

#[test]
fn calc() {
    let data = vec![
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];
    let mut v: Vec<Iline> = Vec::new();
    for d in data {
        v.push(d.parse().unwrap())
    }
    let mut position = Position(0, 0);
    assert_eq!(step(&v, &mut position, 3, 1), false);
    assert_eq!(step(&v, &mut position, 3, 1), true);
    assert_eq!(step(&v, &mut position, 3, 1), false);
    assert_eq!(step(&v, &mut position, 3, 1), true);
    assert_eq!(step(&v, &mut position, 3, 1), true);
    assert_eq!(step(&v, &mut position, 3, 1), false);
    assert_eq!(step(&v, &mut position, 3, 1), true);
    assert_eq!(step(&v, &mut position, 3, 1), true);
    assert_eq!(step(&v, &mut position, 3, 1), true);
    assert_eq!(step(&v, &mut position, 3, 1), true);
    assert_eq!(p1(&v), 7);
    // part 2
    assert_eq!(find_trees(&v, 1, 1), 2);
    assert_eq!(find_trees(&v, 3, 1), 7);
    assert_eq!(find_trees(&v, 5, 1), 3);
    assert_eq!(find_trees(&v, 7, 1), 4);
    assert_eq!(find_trees(&v, 1, 2), 2);
    assert_eq!(p2(&v), 336);
}
