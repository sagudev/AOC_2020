use aoc::read_data_space_saperator2;
use std::collections::HashMap;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Expr {
    Str(char),
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
                //println!("{}", y);
                *self = h.get(&y).unwrap().clone();
                self.noref(h);
            }
            Expr::Str(_) => {}
        }
    }
    fn len(&self) -> usize {
        match self {
            Expr::And(x) => {
                let mut c = 0;
                for d in x {
                    c += d.len();
                }
                c
            }
            Expr::Or(x, _) => x.len(),
            Expr::Ref(_) => {
                panic!("ref in len")
            }
            Expr::Str(_) => 1,
        }
    }
    fn check(&self, i: usize, c: char) -> bool {
        //println!("{}| {}:{}", self, i, c);
        let mut i = i;
        match self {
            Expr::And(x) => {
                for d in x {
                    let len = d.len();
                    if i < len {
                        if !d.check(i, c) {
                            return false
                        }
                    } else {
                        i -= len;
                    }
                }
                true
            }
            Expr::Or(x, y) => x.check(i, c) || y.check(i, c),
            Expr::Ref(_) => panic!("ref in merged"),
            Expr::Str(x) => {
                if i == 0 {
                    *x == c
                } else {
                    panic!("ref in merged {}, {}", x, i)
                }
            }
        }
    }
}

impl FromStr for Expr {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('"') {
            Ok(Expr::Str(s.trim().chars().nth(1).unwrap()))
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
                Ok(Expr::Or(Box::new(Expr::Ref(n11)), Box::new(Expr::Ref(n12))))
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
        //println!("//{} || {}", i, e);
        hm.insert(i, e);
    }
    // now merge all in rule No. 0
    let mut o = hm.get(&0).unwrap().clone();
    // replace refs
    o.noref(&hm);
    o
}

fn check(s: &str, r: &Expr) -> bool {
    for (i, c) in s.trim().chars().enumerate() {
        if !r.check(i, c) {
            return false;
        }
    }
    true
}

fn p1(data: &[&str], r: &Expr) -> i32 {
    let mut c = 0;
    for d in data {
        if check(d, r) {
            c += 1;
        }
    }
    c
}

fn p2(data: &[&str]) -> i32 {
    5
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<String> = read_data_space_saperator2("./data/data19", '$').unwrap();
    //let datac = data.clone();
    //let rule = std::thread::Builder::new()
    //    .stack_size(10_000_000 as usize * 0xFF)
    //    .spawn(move || rule(&datac[0].clone()))
    //    .unwrap()
    //    .join()
    //    .unwrap();
    let rule = rule(&data[0]);
    let data: Vec<&str> = data[1].split('$').filter(|x| *x != "").collect();
    println!("{:?}", rule);
    // part 1
    println!(
        "How many messages completely match rule 0? {}",
        p1(&data, &rule)
    );
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
    let data: Vec<&str> = data[1].split('$').filter(|x| *x != "").collect();
    println!("{}", rule);
    println!("{:?}", data);
    // part 1
    assert_eq!(check(&data[0], &rule), true);
    assert_eq!(check(&data[1], &rule), false);
    assert_eq!(check(&data[2], &rule), true);
    assert_eq!(check(&data[3], &rule), false);
    assert_eq!(check(&data[4], &rule), false);
    assert_eq!(p1(&data, &rule), 2);
}
