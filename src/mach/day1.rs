use crate::mach::Day;

pub struct Day1 {
    pub data: Vec<i32>,
}

impl Day<i32> for Day1 {
    fn new(data: Vec<i32>) -> Self {
        Self { data }
    }
    fn p1(&self) -> String {
        format!("{}", p1(&self.data))
    }
    fn p2(&self) -> String {
        format!("{}", p2(&self.data))
    }
}

use std::cmp;

/// https://users.rust-lang.org/t/sort-3-tuple-idiomatic-code/15913
pub fn sort<T: Eq + Ord + Copy>(tpl: (T, T, T)) -> (T, T, T) {
    let (a, b, c) = tpl;
    let mut v = [a, b, c];
    v.sort();
    (v[0], v[1], v[2])
}

/// find sum 2020 in 2 numbers
fn find_2020(v: &[i32]) -> Option<(i32, i32)> {
    for e in v {
        for c in v {
            if (e + c) == 2020 {
                // ordering for test
                return Some((cmp::min(*e, *c), cmp::max(*e, *c)));
            }
        }
    }
    None
}

fn p1(v: &[i32]) -> i32 {
    let par = find_2020(v).unwrap();
    par.0 * par.1
}

/// find sum 2020 in 2 numbers
fn find_2020_3(v: &[i32]) -> Option<(i32, i32, i32)> {
    for e in v {
        for c in v {
            for s in v {
                if (e + c + s) == 2020 {
                    // ordering for test
                    return Some(sort((*e, *c, *s)));
                }
            }
        }
    }
    None
}

fn p2(v: &[i32]) -> i32 {
    let par = find_2020_3(v).unwrap();
    par.0 * par.1 * par.2
}

#[test]
fn calc() {
    // part 1
    let v = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(find_2020(&v), Some((299, 1721)));
    assert_eq!(p1(&v), 514579);
    // part 2
    assert_eq!(find_2020_3(&v), Some((366, 675, 979)));
    assert_eq!(p2(&v), 241861950);
}
