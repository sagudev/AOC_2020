use aoc::read_data;
use std::error::Error;

fn can_number_next(data: &[u64], u: u64, p: usize) -> bool {
    let len = data.len();
    for i in 1..=p {
        for j in 1..=p {
            if i == j || i >= len || j >= len {
                continue;
            }
            //println!("{} vs. {}", data[len - i], data[len - j]);
            if (data[len - i] + data[len - j]) == u {
                return true;
            }
        }
    }
    false
}

fn p1(data: &[u64], p: usize) -> Option<u64> {
    for i in p..data.len() - 1 {
        if !can_number_next(&data[..i], data[i], p) {
            return Some(data[i]);
        }
    }
    None
}

fn find_set(data: &[u64], u: u64) -> Vec<u64> {
    let mut sum = 0;
    let mut v = Vec::new();
    let len = data.len();
    for i in 0..len {
        for j in data.iter().take(len).skip(i) {
            v.push(*j);
            sum += j;
            if sum == u {
                return v;
            }
        }
        sum = 0;
        v.clear();
    }
    v
}

fn p2(data: &[u64], u: u64) -> u64 {
    let t = find_set(&data, u);
    t.iter().min().unwrap() + t.iter().max().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<u64> = read_data("./data/data9").unwrap();
    // part 1
    println!("The first step of attacking the weakness in the XMAS data is to find the first number in the list (after the preamble) which is not the sum of two of the 25 numbers before it. What is the first number that does not have this property? {}", p1(&data, 25).unwrap());
    // part 2
    println!("What is the encryption weakness in your XMAS-encrypted list of numbers? {}", p2(&data, p1(&data, 25).unwrap()));
    Ok(())
}

#[test]
fn data_read() {
    println!("{:?}", read_data::<u64>("./data/data9").unwrap());
}

#[test]
fn calc() {
    let mut data1: Vec<u64> = (1..=25).collect();
    // 20 on start
    data1.rotate_right(6);
    let data: Vec<u64> = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    // part 1
    assert_eq!(can_number_next(&data1, 26, 25), true);
    assert_eq!(can_number_next(&data1, 49, 25), true);
    assert_eq!(can_number_next(&data1, 100, 25), false);
    assert_eq!(can_number_next(&data1, 50, 25), false);
    data1.push(45);
    assert_eq!(can_number_next(&data1, 26, 25), true);
    assert_eq!(can_number_next(&data1, 65, 25), false);
    assert_eq!(can_number_next(&data1, 64, 25), true);
    assert_eq!(can_number_next(&data1, 66, 25), true);

    assert_eq!(can_number_next(&data[..13], 127, 5), false);
    assert_eq!(p1(&data, 5), Some(127));
    // part 2
    assert_eq!(find_set(&data, 127), vec![15, 25, 47, 40]);
    assert_eq!(p2(&data, 127), 62);
}
