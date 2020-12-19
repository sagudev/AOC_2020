use aoc::read_data_space_saperator2;
use std::collections::HashMap;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Expr {
    Str(String),
    Ref(usize),
    Or(Box<Expr>, Box<Expr>),
    And(Vec<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Str(x) => write!(f, " \"{}\" ", x),
            Expr::Ref(x) => write!(f, " {} ", x),
            Expr::Or(x, y) => write!(f, " {} | {} ", x, y),
            Expr::And(x) => {
                let mut s = String::new();
                for d in x {
                    s = format!("{}{}", s, d);
                }
                write!(f, " ( {} ) ", s)
            }
        }
    }
}

impl Expr {
    fn noref(&mut self, h: &HashMap<usize, Expr>) {
        //println!("|{:?}", self);
        match self {
            Expr::And(ref mut x) => {
                for d in x {
                    d.noref(h);
                }
            }
            Expr::Or(ref mut x, ref mut y) => {
                x.noref(h);
                y.noref(h);
            }
            Expr::Ref(y) => {
                *self = h.get(&y).unwrap().clone();
                self.noref(h);
            }
            Expr::Str(_) => {}
        }
    }
}

impl FromStr for Expr {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('"') {
            Ok(Expr::Str(s.split('"').nth(1).unwrap().to_string()))
        } else if s.contains('|') {
            let mut n11 = 0;
            let mut n12 = 0;
            let mut seen = 0;
            let mut n21 = 0;
            let mut n22 = 0;
            for z in s.split_ascii_whitespace() {
                if let Ok(x) = z.parse() {
                    if n11 == 0 {
                        n11 = x;
                    } else if n12 == 0 {
                        n12 = x;
                    } else if n21 == 0 {
                        n21 = x;
                    } else if n22 == 0 {
                        n22 = x;
                    }
                } else if n12 == 0 {
                    seen = 1;
                } else {
                    seen = 2;
                }
            }
            if seen == 2 {
                Ok(Expr::Or(
                    Box::new(Expr::And(vec![Expr::Ref(n11), Expr::Ref(n12)])),
                    Box::new(Expr::And(vec![Expr::Ref(n21), Expr::Ref(n22)])),
                ))
            } else {
                Ok(Expr::Or(Box::new(Expr::Ref(n11)), Box::new(Expr::Ref(n21))))
            }
        } else {
            let mut v = Vec::new();
            for z in s.split_ascii_whitespace() {
                if let Ok(x) = z.parse() {
                    v.push(Expr::Ref(x));
                } else {
                    panic!("p2");
                }
            }
            Ok(Expr::And(v))
        }
    }
}

/// merge all rules into one
fn rule(s: &str) -> Expr {
    let mut hm: HashMap<usize, Expr> = HashMap::new();
    for l in s.split('$') {
        if l == "" {
            continue;
        }
        //print!("{}", l);
        let mut l = l.split(':');
        let i: usize = l.next().unwrap().parse().unwrap();
        let e: Expr = l.next().unwrap().parse().unwrap();
        //println!("//{} || {:?}", i, e);
        hm.insert(i, e);
    }
    // now merge all in rule No. 0
    let mut o = hm.get(&0).unwrap().clone();
    // replace refs
    o.noref(&hm);
    o
}

fn p1(data: &[String]) -> i32 {
    5
}

fn p2(data: &[String]) -> i32 {
    5
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<String> = read_data_space_saperator2("./data/data19", '$').unwrap();
    let rule = rule(&data[0]);
    println!("{}", rule);
    // part 1
    println!("How many messages completely match rule 0? {}", p1(&data));
    // part 2
    println!("How many messages completely match rule 0? {}", p2(&data));
    Ok(())
}

#[test]
fn data_read19() {
    println!(
        "{:?}",
        read_data_space_saperator2::<String>("./data/data19", '$').unwrap()
    );
}

#[test]
fn calc19() {
    let data = vec![
        "$0: 4 1 5$1: 2 3 | 3 2$2: 4 4 | 5 5$3: 4 5 | 5 4$4: \"a\"$5: \"b\"$".to_string(),
        "$ababbb$bababa$abbbab$aaabbb$aaaabbb".to_string(),
    ];
    let rule = rule(&data[0]);
    println!("{}", rule);
    // part 1
}
