use aoc::read_data;
use std::collections::HashSet;
use std::error::Error;
use std::iter::FromIterator;

fn my_device(data: &[i64]) -> i64 {
    data.iter().max().unwrap() + 3
}

fn jolts_1_3(data: &[i64]) -> (i64, i64) {
    let h: HashSet<i64> = HashSet::from_iter(data.iter().cloned());
    let mut l1 = 0;
    let mut l3 = 0;
    let mut last = 0;
    for j in 0..=my_device(&data) {
        if h.get(&j).is_some() {
            match j - last {
                1 => l1 += 1,
                //2 => (0),
                3 => l3 += 1,
                _ => {}
            }
            last = j;
        }
    }
    (l1, l3 + 1)
}

fn p1(data: &[i64]) -> i64 {
    let t = jolts_1_3(&data);
    t.0 * t.1
}

// https://www.reddit.com/r/adventofcode/comments/kabi91/2020_day_10_closedform_mathematical_solution/gfaokhy?utm_source=share&utm_medium=web2x&context=3
fn m1(data: &[i64]) -> (u32, u32) {
    let mut data: Vec<i64> = data.iter().copied().collect();
    data.push(0);
    data.sort_unstable();
    let mut d1: Vec<i64> = data
        .iter()
        .map(|d| {
            let mut count = 0;
            if data.contains(&(d - 3)) {
                count += 1;
            }
            if data.contains(&(d - 2)) {
                count += 1;
            }
            if data.contains(&(d - 1)) {
                count += 1;
            }
            count
        })
        .collect();
    d1.remove(0);
    println!("{:?}", d1);
    let mut full = 0;
    let mut tri = 0;
    for i in 0..d1.len() {
        if d1[i] != 1 {
            full += 1;
        }
        if i + 2 < d1.len() && d1[i] != 1 && d1[i + 1] != 1 && d1[i + 2] != 1 {
            tri += 1;
        }
    }
    println!("{} | {}", tri, full);
    (full - (tri * 3), tri)
}

fn p2(data: &[i64]) -> i64 {
    let t = m1(&data);
    2_i64.pow(t.0) * 7_i64.pow(t.1)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<i64> = read_data("./data/data10").unwrap();
    // part 1
    println!("What is the number of 1-jolt differences multiplied by the number of 3-jolt differences? {:?}", p1(&data));
    // part 2
    println!("What is the total number of distinct ways you can arrange the adapters to connect the charging outlet to your device? {}", p2(&data));
    Ok(())
}

#[test]
fn data_read() {
    println!("{:?}", read_data::<i64>("./data/data10").unwrap());
}

#[test]
fn calc() {
    let data: Vec<i64> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    let data1: Vec<i64> = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    // part 1
    assert_eq!(my_device(&data), 22);
    assert_eq!(jolts_1_3(&data), (7, 5));

    assert_eq!(my_device(&data1), 49 + 3);
    assert_eq!(jolts_1_3(&data1), (22, 10));
    // part 2
    //assert_eq!(jolts_min(&data), 22);
    assert_eq!(p2(&data), 8);
    assert_eq!(p2(&data1), 19208);
}

/*
(0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4, 5, 6, 7, 10,     12, 15, 16, 19, (22)
(0), 1, 4, 5,    7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4, 5,    7, 10,     12, 15, 16, 19, (22)
(0), 1, 4,    6, 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4,    6, 7, 10,     12, 15, 16, 19, (22)
(0), 1, 4,       7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4,       7, 10,     12, 15, 16, 19, (22)
diff 3
2**3=8

(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 64, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49, (52)
*/
