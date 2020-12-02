use aoc::read_data;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    c: char,
    pass: String,
}

impl Password {
    fn is_valid_1(&self) -> bool {
        let matches = self.pass.matches(self.c).count();
        self.min <= matches && matches <= self.max
    }
    fn is_valid_2(&self) -> bool {
        let matches: Vec<char> = self.pass.chars().collect();
        if matches[self.min - 1] == self.c || matches[self.max - 1] == self.c {
            if matches[self.min - 1] == matches[self.max - 1] {
                return false;
            }
            return true;
        }
        false
    }
}

// 1-3 b: cdefg
impl FromStr for Password {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(' ').collect();
        let mut num = v[0].split('-');
        let min = num.next().unwrap().parse::<usize>()?;
        let max = num.next().unwrap().parse::<usize>()?;
        Ok(Self {
            min,
            max,
            c: v[1].chars().next().unwrap(),
            pass: v[2].to_string(),
        })
    }
}

fn p1(data: &[Password]) -> usize {
    let mut valid: usize = 0;
    for d in data {
        if d.is_valid_1() {
            valid += 1
        }
    }
    valid
}

fn p2(data: &[Password]) -> usize {
    let mut valid: usize = 0;
    for d in data {
        if d.is_valid_2() {
            valid += 1
        }
    }
    valid
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    // part 1
    let data: Vec<Password> = read_data::<Password>("./data/data2").unwrap();
    println!("Part 1: {}", p1(&data));
    //part 2
    println!("Part 2: {}", p2(&data));
    Ok(())
}

#[test]
fn data_read() {
    println!("{:?}", read_data::<String>("./data/data2").unwrap());
}

#[test]
fn calc() {
    let data = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    let mut v: Vec<Password> = Vec::new();
    for d in data {
        v.push(d.parse().unwrap())
    }
    // part 1
    assert_eq!(v[0].is_valid_1(), true);
    assert_eq!(v[1].is_valid_1(), false);
    assert_eq!(v[2].is_valid_1(), true);
    assert_eq!(p1(&v), 2);
    // part 2
    assert_eq!(v[0].is_valid_2(), true);
    assert_eq!(v[1].is_valid_2(), false);
    assert_eq!(v[2].is_valid_2(), false);
    assert_eq!(p2(&v), 1);
}
