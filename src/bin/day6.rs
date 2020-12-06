use aoc::read_data_space_saperator;
use std::collections::HashSet;
use std::error::Error;

fn line_yes(s: &str) -> usize {
    let my_set: HashSet<char> = s.chars().collect();
    let set: HashSet<char> = ('a'..='z').collect();
    let v: Vec<char> = set.difference(&my_set).copied().collect();
    //println!("{} || {} || {:?} || {:?}", v.len(), 26-v.len(), v, my_set);
    26 - v.len()
}

fn p1(data: &[String]) -> usize {
    let mut res = 0;
    for d in data {
        res += line_yes(d);
    }
    res
}

fn group_yes(s: &str) -> usize {
    let v: Vec<HashSet<char>> = s
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.chars().collect())
        .collect();
    let mut c: HashSet<char> = v[0].clone();
    //let v: Vec<char> = set.difference(&my_set).copied().collect();
    for e in v {
        c = c.intersection(&e).copied().collect()
    }
    //println!("{} || {} || {:?} || {:?}", v.len(), 26-v.len(), v, my_set);
    //26 - v.len()
    c.len()
}

fn p2(data: &[String]) -> usize {
    let mut res = 0;
    for d in data {
        res += group_yes(d);
    }
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<String> = read_data_space_saperator("./data/data6").unwrap();
    // part 1
    println!("For each group, count the number of questions to which anyone answered yes. What is the sum of those counts? {}", p1(&data));
    //part 2
    println!("For each group, count the number of questions to which everyone answered yes. What is the sum of those counts? {}", p2(&data));
    Ok(())
}

#[test]
fn data_read() {
    println!(
        "{:?}",
        read_data_space_saperator::<String>("./data/data6").unwrap()[0]
    );
}

#[test]
fn calc() {
    // part 1
    assert_eq!(line_yes("abcx abcy abcz"), 6);
    let data: Vec<&str> = vec!["abc", "a b c", "ab ac", "a a a a", "b"];
    assert_eq!(line_yes(&data[0]), 3);
    assert_eq!(line_yes(&data[1]), 3);
    assert_eq!(line_yes(&data[2]), 3);
    assert_eq!(line_yes(&data[3]), 1);
    assert_eq!(line_yes(&data[4]), 1);
    assert_eq!(
        p1(&data
            .iter()
            .map(|x| { x.to_string() })
            .collect::<Vec<String>>()),
        11
    );
    // part 2
    assert_eq!(group_yes(&data[0]), 3);
    assert_eq!(group_yes(&data[1]), 0);
    assert_eq!(group_yes(&data[2]), 1);
    assert_eq!(group_yes(&data[3]), 1);
    assert_eq!(group_yes(&data[4]), 1);
    assert_eq!(
        p2(&data
            .iter()
            .map(|x| { x.to_string() })
            .collect::<Vec<String>>()),
        6
    );
}
