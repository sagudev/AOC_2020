use aoc::read_data;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    /// +
    A,
    /// *
    M,
    /// number
    Num(i64),
    /// ()
    P(VecDeque<Token>),
}

fn parse(s: &str) -> VecDeque<Token> {
    let mut v = VecDeque::new();
    let mut in_p = 0;
    let mut placeholder = String::new();
    'c: for d in s.split_ascii_whitespace() {
        if in_p > 0 {
            if d.contains(')') {
                for c in d.chars() {
                    if c == ')' {
                        in_p -= 1
                    }
                    if in_p == 0 {
                        v.push_back(Token::P(parse(&placeholder)));
                        placeholder.clear();
                        continue 'c;
                    } else {
                        placeholder += std::str::from_utf8(&[c as u8]).unwrap();
                    }
                }
                placeholder += " ";
                if in_p == 0 {
                    v.push_back(Token::P(parse(&placeholder)));
                    placeholder.clear();
                }
                continue;
            }

            if d.contains('(') {
                for c in d.chars() {
                    if c == '(' {
                        in_p += 1
                    }
                }
            }
            placeholder += d;
            placeholder += " ";
            continue;
        }
        if let Ok(x) = d.parse() {
            v.push_back(Token::Num(x));
            continue;
        }
        match d {
            "+" => v.push_back(Token::A),
            "*" => v.push_back(Token::M),
            _ => {
                for (i, c) in d.chars().enumerate() {
                    if c == '(' {
                        in_p += 1;
                    }
                    if i != 0 {
                        placeholder += std::str::from_utf8(&[c as u8]).unwrap();
                    }
                }
                placeholder += " ";
            }
        }
    }
    v
}

fn eval(v: &VecDeque<Token>) -> i64 {
    // true = +
    let mut next: Option<bool> = None;
    let mut n = 0;
    let mut n1 = 0;
    // deal with +
    for t in v {
        match t {
            Token::P(x) => n1 = eval(x),
            Token::Num(x) => n1 = *x,
            Token::A => next = Some(true),
            Token::M => next = Some(false),
        }
        if let Some(x) = next {
            if n1 != 0 {
                if x {
                    n += n1
                } else {
                    n *= n1
                }
                n1 = 0;
            }
        } else {
            n = n1;
            n1 = 0;
        }
    }
    n
}

fn eval_line(s: &str) -> i64 {
    // parse
    let v = parse(s);
    //evaluate
    eval(&v)
}

fn p1(data: &[String]) -> i64 {
    let mut sum = 0;
    for d in data {
        sum += eval_line(d);
    }
    sum
}

fn eval2(v: &VecDeque<Token>) -> i64 {
    let mut m0: VecDeque<Token> = VecDeque::new();
    let mut n = 0;
    let mut h = HashSet::new();
    for (i, t) in v.iter().enumerate() {
        if v.len() > i + 1 && v[i + 1] == Token::A {
            let n0 = if h.contains(&i) {
                if let Some(Token::Num(x)) = m0.pop_back() {
                    x
                } else {
                    panic!("add err");
                }
            } else if let Token::Num(x) = t {
                *x
            } else if let Token::P(x) = t {
                eval2(x)
            } else {
                panic!("add err");
            };
            let n1 = if let Token::Num(x) = v[i + 2] {
                x
            } else if let Token::P(x) = &v[i + 2] {
                eval2(&x)
            } else {
                panic!("add err");
            };
            h.insert(i);
            h.insert(i + 1);
            h.insert(i + 2);
            m0.push_back(Token::Num(n0 + n1));
        } else if !h.contains(&i) {
            m0.push_back(t.clone());
        }
    }
    for (i, m) in m0.iter().enumerate() {
        if i == 0 {
            n = if let Token::Num(x) = m {
                *x
            } else if let Token::P(x) = m {
                eval2(x)
            } else {
                panic!("m err");
            };
        } else if i != 1 && m0[i - 1] == Token::M {
            n *= if let Token::Num(x) = m {
                *x
            } else if let Token::P(x) = m {
                eval2(x)
            } else {
                panic!("mm err");
            };
        }
    }
    n
}

fn eval_line2(s: &str) -> i64 {
    // parse
    let v = parse(s);
    //evaluate
    eval2(&v)
}

fn p2(data: &[String]) -> i64 {
    let mut sum = 0;
    for d in data {
        sum += eval_line2(d);
    }
    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<String> = read_data("./data/data18").unwrap();
    // part 1
    println!("Evaluate the expression on each line of the homework; what is the sum of the resulting values? {}", p1(&data));
    // part 2
    println!("What do you get if you add up the results of evaluating the homework problems using these new rules? {}", p2(&data));
    Ok(())
}

#[test]
fn data_read18() {
    println!("{:?}", read_data::<String>("./data/data18").unwrap());
}

#[test]
fn calc18() {
    // part 1
    assert_eq!(eval_line("1 + 2 * 3 + 4 * 5 + 6"), 71);
    assert_eq!(eval_line("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(eval_line("2 * 3 + (4 * 5)"), 26);
    assert_eq!(eval_line("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    assert_eq!(
        eval_line("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        12240
    );
    assert_eq!(
        eval_line("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        13632
    );
    // part 2
    assert_eq!(eval_line2("1 + 2 * 3 + 4 * 5 + 6"), 231);
    assert_eq!(eval_line2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(eval_line2("2 * 3 + (4 * 5)"), 46);
    assert_eq!(eval_line2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    assert_eq!(
        eval_line2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        669060
    );
    assert_eq!(
        eval_line2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        23340
    );
}
