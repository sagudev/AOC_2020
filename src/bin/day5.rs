use std::collections::HashSet;
use aoc::read_data;
use std::error::Error;

fn get_row(s: &str) -> u8 {
    u8::from_str_radix(&s[..7].replace('F', "0").replace('B', "1"), 2).unwrap()
}

fn get_col(s: &str) -> u8 {
    u8::from_str_radix(&s[s.len() - 3..].replace('L', "0").replace('R', "1"), 2).unwrap()
}

fn calc_id(s: &str) -> usize {
    get_row(s) as usize * 8 + get_col(s) as usize
}

/// max()
fn p1(data: &[String]) -> usize {
    let mut max = 0;
    for d in data {
        max = std::cmp::max(calc_id(d), max)
    }
    max
}

fn min(data: &[String]) -> usize {
    let mut min = 1000;
    for d in data {
        min = std::cmp::min(calc_id(d), min)
    }
    min
}

fn p2(data: &[String]) -> usize {
    let full_set: HashSet<usize> = (min(&data)..p1(&data)).collect();
    let my_set: HashSet<usize> = data.iter().map(|x| {calc_id(x)}).collect();
    let diff: Vec<usize> = full_set.difference(&my_set).copied().collect();
    diff[0]
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    // part 1
    let data: Vec<String> = read_data("./data/data5").unwrap();
    println!("As a sanity check, look through your list of boarding passes. What is the highest seat ID on a boarding pass? {}", p1(&data));
    //part 2
    println!("What is the ID of your seat?: {}", p2(&data));
    Ok(())
}

#[test]
fn data_read() {
    println!("{:?}", read_data::<String>("./data/data5").unwrap());
}

#[test]
fn calc() {
    // part1
    let s = "FBFBBFFRLR";
    assert_eq!(get_row(s), 44);
    assert_eq!(get_col(s), 5);
    assert_eq!(calc_id(s), 357);
    let s = "BFFFBBFRRR";
    assert_eq!(get_row(s), 70);
    assert_eq!(get_col(s), 7);
    assert_eq!(calc_id(s), 567);
    let s = "FFFBBBFRRR";
    assert_eq!(get_row(s), 14);
    assert_eq!(get_col(s), 7);
    assert_eq!(calc_id(s), 119);
    let s = "BBFFBBFRLL";
    assert_eq!(get_row(s), 102);
    assert_eq!(get_col(s), 4);
    assert_eq!(calc_id(s), 820);
}
