use std::collections::HashSet;
use aoc::read_data;
use std::collections::HashMap;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Bag(String, usize);

type Contain = Option<Vec<Bag>>;

impl FromStr for Bag {
    type Err = ParseIntError;
    // 4 shiny brown bags
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //println!("{}", s);
        Ok(Self(s[1..s.len() - 4].trim().to_string(), s[0..1].parse()?))
    }
}

fn hash_data(data: &[String]) -> HashMap<&str, Contain> {
    let mut m = HashMap::new();
    for d in data {
        let t: Vec<&str> = d.split("contain").collect();
        if t[1].contains("other") {
            m.insert(t[0][..t[0].len() - 5].trim(), None);
        } else {
            let u = t[1][..t[1].len() - 1]
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.trim().parse().unwrap())
                .collect();
            m.insert(t[0][..t[0].len() - 5].trim(), Some(u));
        }
    }
    m
}

fn can_bag_hold_gold(bag: &str, hash: &HashMap<&str, Contain>) -> bool {
    if let Some(t) = hash.get(bag) {
        if let Some(v) = t {
            for u in v {
                if u.0.contains("shiny gold") {
                    return true;
                }
                if can_bag_hold_gold(&u.0, hash) {
                    return true;
                }
            };
        }
    }
    false
}

fn p1(hash: &HashMap<&str, Contain>) -> usize {
    let mut can = HashSet::new();
    for k in hash.keys() {
        if k.contains("shiny gold") {
            continue;
        }
        if can_bag_hold_gold(k, hash) {
            can.insert(k);
        }
    }
    can.len()
}

fn must_hold(bag: &str, hash: &HashMap<&str, Contain>) -> usize {
    let mut i = 0;
    println!("-{}", bag);
    if let Some(t) = hash.get(bag) {
        if let Some(v) = t {
            for u in v {
                println!("{:?}", u);
                i += u.1;
                i += u.1 * must_hold(&u.0, hash);
            };
        }
    } else {
        panic!("hi")
    }
    i
}


fn p2(hash: &HashMap<&str, Contain>) -> usize {
    must_hold("shiny gold", hash)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<String> = read_data("./data/data7").unwrap();
    let hash = hash_data(&data);
    //println!("|| {:#?}", hash);
    // part 1
    println!("How many bag colors can eventually contain at least one shiny gold bag?  {}", p1(&hash));
    // part 2
    println!("How many individual bags are required inside your single shiny gold bag? {}", p2(&hash));
    Ok(())
}

#[test]
fn data_read() {
    println!("{:?}", read_data::<String>("./data/data7").unwrap());
}

#[test]
fn calc() {
    let data: Vec<String> = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
        "bright white bags contain 1 shiny gold bag.".to_string(),
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
        "faded blue bags contain no other bags.".to_string(),
        "dotted black bags contain no other bags.".to_string(),
    ];
    let hash = hash_data(&data);
    // p1
    assert_eq!(p1(&hash), 4);
    // p2
    assert_eq!(p2(&hash), 32);
}
