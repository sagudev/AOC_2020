use aoc::read_data;
use std::collections::HashMap;
use std::error::Error;

fn trans(s: &str) -> Vec<usize> {
    s.split(',').map(|x| x.parse().unwrap()).collect()
}

fn turrn(data: &mut Vec<usize>, videno: &mut HashMap<usize, (usize, usize)>, turn: usize) -> usize {
    if turn - 1 < data.len() {
        //println!("{} || {}", turn, data[turn - 1]);
        videno.insert(data[turn - 1], (turn, 0));
        return data[turn - 1];
    } else {
        let prej = data[turn - 2];
        let v = videno.get(&prej).unwrap();
        //println!("{} | v: {:?} prej: {} zdej: {}", turn, v, prej, v.0 - v.1);
        if v.1 == 0 {
            data.push(0);
            let m = videno.entry(0).or_insert((0,0));
            *m = (turn, m.0)
        } else {
            data.push(v.0 - v.1);
            let m = videno.entry(v.0 - v.1).or_insert((0,0));
            *m = (turn, m.0)
        }
        //println!("last is: {} on {}", data[turn - 2], first);
    }
    *data.last().unwrap()
}

fn p1(data: &[usize]) -> usize {
    let mut data: Vec<usize> = data.to_vec();
    let mut videno: HashMap<usize, (usize, usize)> = HashMap::new();
    for turn in 1..=2020 {
        turrn(&mut data, &mut videno, turn);
    }
    *data.last().unwrap()
}

fn p2(data: &[usize]) -> usize {
    let mut data: Vec<usize> = data.to_vec();
    let mut videno: HashMap<usize, (usize, usize)> = HashMap::new();
    for turn in 1..=30000000 {
        //println!("{}", turn);
        turrn(&mut data, &mut videno, turn);
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
    let mut videno: HashMap<usize, (usize, usize)> = HashMap::new();
    // part 1
    assert_eq!(turrn(&mut data, &mut videno, 1), 0);
    assert_eq!(turrn(&mut data, &mut videno, 2), 3);
    assert_eq!(turrn(&mut data, &mut videno, 3), 6);
    assert_eq!(turrn(&mut data, &mut videno, 4), 0);
    assert_eq!(turrn(&mut data, &mut videno, 5), 3);
    assert_eq!(turrn(&mut data, &mut videno, 6), 3);
    assert_eq!(turrn(&mut data, &mut videno, 7), 1);
    assert_eq!(turrn(&mut data, &mut videno, 8), 0);
    assert_eq!(turrn(&mut data, &mut videno, 9), 4);
    assert_eq!(turrn(&mut data, &mut videno, 10), 0);
    assert_eq!(p1(&trans("0,3,6")), 436);

    assert_eq!(p1(&trans("1,3,2")), 1);
    assert_eq!(p1(&trans("2,1,3")), 10);
    assert_eq!(p1(&trans("1,2,3")), 27);
    assert_eq!(p1(&trans("2,3,1")), 78);
    assert_eq!(p1(&trans("3,2,1")), 438);
    assert_eq!(p1(&trans("3,1,2")), 1836);

    // part 2
    assert_eq!(p2(&trans("0,3,6")), 175594);
    assert_eq!(p2(&trans("1,3,2")), 2578);
    assert_eq!(p2(&trans("2,1,3")), 3544142);
    assert_eq!(p2(&trans("1,2,3")), 261214);
    assert_eq!(p2(&trans("2,3,1")), 6895259);
    assert_eq!(p2(&trans("3,2,1")), 18);
    assert_eq!(p2(&trans("3,1,2")), 362);
}
