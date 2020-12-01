use aoc::read_data;
use std::cmp;
use std::error::Error;

/// https://users.rust-lang.org/t/sort-3-tuple-idiomatic-code/15913
pub fn sort<T: Eq + Ord + Copy>(tpl: (T, T, T)) -> (T, T, T) {
    let (a, b, c) = tpl;
    let mut v = [a, b, c];
    v.sort();
    (v[0], v[1], v[2])
}

/// find sum 2020 in 2 numbers
fn find_2020(v: &Vec<i32>) -> Option<(i32, i32)> {
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

fn p1(v: &Vec<i32>) -> i32 {
    let par = find_2020(v).unwrap();
    par.0 * par.1
}

/// find sum 2020 in 2 numbers
fn find_2020_3(v: &Vec<i32>) -> Option<(i32, i32, i32)> {
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

fn p2(v: &Vec<i32>) -> i32 {
    let par = find_2020_3(v).unwrap();
    par.0 * par.1 * par.2
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data = read_data("./data/data1").unwrap();
    // lets reorgenize data
    let mut v: Vec<i32> = Vec::new();
    for d in data {
        v.push(d.parse::<i32>().unwrap());
    }
    // part 1
    println!("Find the two entries that sum to 2020; what do you get if you multiply them together? {:?}", p1(&v));
    //assert_eq!(842016, p1(&v));
    //part 2
    println!(
        "In your expense report, what is the product of the three entries that sum to 2020? {:?}",
        p2(&v)
    );
    //assert_eq!(9199664, p1(&v));
    Ok(())
}

#[test]
fn data_read() {
    println!("{:?}", read_data("./data/data1").unwrap());
}

#[test]
fn day1_calc() {
    // part 1
    let v = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(find_2020(&v), Some((299, 1721)));
    assert_eq!(p1(&v), 514579);
    // part 2
    assert_eq!(find_2020_3(&v), Some((366, 675, 979)));
    assert_eq!(p2(&v), 241861950);
}
