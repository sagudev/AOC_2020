use aoc::read_data;
use std::error::Error;

fn trans(s: &str) -> Vec<usize> {
    s.split(',').map(|x| x.parse().unwrap()).collect()
}

fn turrn(data: &mut Vec<usize>, turn: usize) -> usize {
    if turn - 1 < data.len() {
        return data[turn - 1];
    } else {
        let prej = data[turn - 2];
        let mut iter = data.iter();
        let mut first = iter.rposition(|n| *n == prej).unwrap();
        if let Some(second) = iter.rposition(|n| *n == prej) {
            data.push((first + 1) - (second + 1));
        } else {
            data.push(0);
        }
        //println!("last is: {} on {}", data[turn - 2], first);
    }
    *data.last().unwrap()
}

fn p1(data: &[usize]) -> usize {
    let mut data: Vec<usize> = data.to_vec();
    for turn in 1..=2020 {
        turrn(&mut data, turn);
    }
    *data.last().unwrap()
}

fn p2(data: &[usize]) -> usize {
    let mut data: Vec<usize> = data.to_vec();
    for turn in 1..=30000000 {
        //println!("{}", turn);
        turrn(&mut data, turn);
    }
    *data.last().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<String> = read_data("./data/data15").unwrap();
    let data: Vec<usize> = data[0].split(',').map(|x| x.parse().unwrap()).collect();
    // part 1
    println!("what will be the 2020th number spoken? {}", p1(&data));
    // part 2
    println!("what will be the 30000000th number spoken? {}", p2(&data));
    Ok(())
}

#[test]
fn data_rread() {
    println!("{:?}", read_data::<String>("./data/data15").unwrap());
}

#[test]
fn calc() {
    let mut data: Vec<usize> = trans("0,3,6");
    // part 1
    assert_eq!(turrn(&mut data, 1), 0);
    assert_eq!(turrn(&mut data, 2), 3);
    assert_eq!(turrn(&mut data, 3), 6);
    assert_eq!(turrn(&mut data, 4), 0);
    assert_eq!(turrn(&mut data, 5), 3);
    assert_eq!(turrn(&mut data, 6), 3);
    assert_eq!(turrn(&mut data, 7), 1);
    assert_eq!(turrn(&mut data, 8), 0);
    assert_eq!(turrn(&mut data, 9), 4);
    assert_eq!(turrn(&mut data, 10), 0);
    assert_eq!(p1(&trans("0,3,6")), 436);

    assert_eq!(p1(&trans("1,3,2")), 1);
    assert_eq!(p1(&trans("2,1,3")), 10);
    assert_eq!(p1(&trans("1,2,3")), 27);
    assert_eq!(p1(&trans("2,3,1")), 78);
    assert_eq!(p1(&trans("3,2,1")), 438);
    assert_eq!(p1(&trans("3,1,2")), 1836);
}
