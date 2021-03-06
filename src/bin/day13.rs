#![allow(warnings)]
use aoc::read_data;
use std::error::Error;
use std::num::ParseIntError;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::mpsc::sync_channel;
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Bus {
    Id(usize),
    X,
}

impl Bus {
    pub fn unwrap(self) -> usize {
        match self {
            Bus::Id(val) => val,
            _ => panic!("called `Bus::unwrap()` on a `None` value"),
        }
    }
}

impl FromStr for Bus {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('x') {
            Ok(Bus::X)
        } else {
            Ok(Bus::Id(s.parse()?))
        }
    }
}

/// return id of bus and minutes
fn z1(d2: &[usize], d1: usize) -> (usize, usize) {
    let mut best = (0, 1000);
    for d in d2 {
        let r = (d1 as f64 / *d as f64).ceil() as usize;
        let bus_cycle = r * d;
        let wait = bus_cycle - d1;
        //println!("{} {} {} {}", d, r, bus_cycle, wait);
        if best.1 > wait {
            best = (*d, wait);
        }
    }
    best
}

fn p1(d2: &[usize], d1: usize) -> usize {
    let z = z1(d2, d1);
    z.0 * z.1
}

/* fn xvec(data: &[Bus]) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::new();
    let mut nx = 0;
    for d in data {
        match d {
            Bus::Id(x) => {
                if let Bus::Id(c) = data[0] {
                    v.push(x + c + nx);
                }
                nx = 0;
            }
            Bus::X => nx += 1,
        }
    }
    v
} */

fn check_t2(data: &[Bus], t: usize) -> bool {
    let mut t = t;
    for d in data {
        match d {
            Bus::X => {}
            Bus::Id(x) => {
                if t % x != 0 {
                    return false;
                } else {
                    println!("{} {}", t, x);
                }
            }
        }
        t += 1
    }
    true
}

fn check_t22(data: &[Bus], t: usize, idx: usize) -> bool {
    //println!("|||");
    let mut tt = t + 1;
    for i in data.iter().skip(idx + 1) {
        //println!("||{:?}", i);
        match i {
            Bus::X => {}
            Bus::Id(x) => {
                if tt % x != 0 {
                    return false;
                }
            }
        }
        tt += 1
    }
    let mut tt = t - 1;
    for i in data.iter().rev().skip(data.len() - idx) {
        //println!("|{:?}", i);
        match i {
            Bus::X => {}
            Bus::Id(x) => {
                if tt % x != 0 {
                    return false;
                }
            }
        }
        tt -= 1
    }
    true
}

fn p222(data: &[Bus]) -> usize {
    let c = data
        .iter()
        .filter(|x| **x != Bus::X)
        .map(|x| x.unwrap())
        .max()
        .unwrap();
    let i = data.iter().position(|&x| x == Bus::Id(c)).unwrap();
    println!("{}", c);
    let mut t = c;
    //let r = (500000000000000.0 / c as f64).ceil() as usize;
    //t = r * c;
    println!("{}", t);
    loop {
        //println!("||||||||||| {}", t);
        if check_t22(&data, t, i) {
            break;
        }
        t += c;
    }
    t - i
}

#[allow(dead_code)]
fn p22(data: &[Bus]) -> usize {
    if let Bus::Id(c) = data[0] {
        let mut t = c;
        //let r = (100000000000000.0 / c as f64).ceil() as usize;
        //t = r * c;
        loop {
            //println!("||||||||||| {}", t);
            if check_t2(&data, t) {
                break;
            }
            t += c;
        }
        t
    } else {
        0
    }
}

fn check_t(data: &[(usize, Bus)], t: usize) -> Option<usize> {
    for (i, d) in data {
        match d {
            Bus::X => {}
            Bus::Id(x) => {
                if (t + i) % x != 0 {
                    //println!("{}: {} + {}", x, t, i);
                    return None;
                }
            }
        }
    }
    Some(t)
}

use std::sync::mpsc::channel;
use std::thread;

/// main thread
struct MT<T, U> {
    tx: std::sync::mpsc::Sender<T>,
    rx: std::sync::mpsc::Receiver<U>,
}

/// worker thread
struct WT<T, U> {
    tx: std::sync::mpsc::Sender<U>,
    rx: std::sync::mpsc::Receiver<T>,
}

/// 2-way channel T for send, U for recive
struct RTX<T, U> {
    /// main thread
    m: MT<T, U>,
    /// worker thread
    w: WT<T, U>,
}

impl<T, U> RTX<T, U> {
    fn new() -> Self {
        let (tx, rx) = channel();
        let (txx, rxx) = channel();
        Self {
            m: MT { tx, rx: rxx },
            w: WT { tx: txx, rx },
        }
    }
    fn get_mw(self) -> (MT<T, U>, WT<T, U>) {
        (self.m, self.w)
    }
}

fn wt(wt: WT<usize, Option<usize>>, data: &std::vec::Vec<(usize, Bus)>) {
    loop {
        let get = wt.rx.recv().unwrap();
        wt.tx.send(check_t(&data, get)).unwrap()
    }
}

// inti threds 1 time and then send cmd
fn p2(data: &[Bus]) -> usize {
    let data: Arc<Vec<(usize, Bus)>> = Arc::new(
        data.iter()
            .enumerate()
            .filter(|(_, x)| **x != Bus::X)
            .map(|(x, y)| (x, *y))
            .collect(),
    );
    let (m1, w1) = RTX::new().get_mw();
    let (m2, w2) = RTX::new().get_mw();
    let (m3, w3) = RTX::new().get_mw();
    let (m4, w4) = RTX::new().get_mw();
    {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            wt(w1, &data);
        });
    }
    {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            wt(w2, &data);
        });
    }
    {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            wt(w3, &data);
        });
    }
    {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            wt(w4, &data);
        });
    }
    if let (_, Bus::Id(c)) = data[0] {
        let mut t = c;
        loop {
            m1.tx.send(t).unwrap();
            t += c;
            m2.tx.send(t).unwrap();
            t += c;
            m3.tx.send(t).unwrap();
            t += c;
            m4.tx.send(t).unwrap();
            t += c;

            if let Some(x) = m1.rx.recv().unwrap() {
                return x;
            }
            if let Some(x) = m2.rx.recv().unwrap() {
                return x;
            }
            if let Some(x) = m3.rx.recv().unwrap() {
                return x;
            }
            if let Some(x) = m4.rx.recv().unwrap() {
                return x;
            }
        }
    }
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<String> = read_data("./data/data13").unwrap();
    let d1: usize = data[0].parse().unwrap();
    let d2: Vec<usize> = data[1]
        .split(',')
        .filter(|x| !x.contains('x'))
        .map(|x| x.parse().unwrap())
        .collect();
    let d3: Vec<Bus> = data[1].split(',').map(|x| x.parse().unwrap()).collect();
    // part 1
    println!("What is the ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait for that bus? {}", p1(&d2, d1));
    // part 2
    println!("What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list? {}", p2(&d3));
    Ok(())
}

#[test]
fn data_read13() {
    println!("{:?}", read_data::<String>("./data/data13").unwrap());
}

#[test]
fn calc13() {
    let data: Vec<String> = vec!["939".to_string(), "7,13,x,x,59,x,31,19".to_string()];
    let d1: usize = data[0].parse().unwrap();
    let d2: Vec<usize> = data[1]
        .split(',')
        .filter(|x| !x.contains('x'))
        .map(|x| x.parse().unwrap())
        .collect();
    let d3: Vec<Bus> = data[1].split(',').map(|x| x.parse().unwrap()).collect();
    // part 1
    assert_eq!(z1(&d2, d1), (59, 5));
    assert_eq!(p1(&d2, d1), 295);
    // part 2
    assert_eq!(p2(&d3), 1068781);
    let d3: Vec<Bus> = "17,x,13,19"
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(p2(&d3), 3417);
    let d3: Vec<Bus> = "67,7,59,61"
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(p2(&d3), 754018);
    let d3: Vec<Bus> = "67,x,7,59,61"
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(p2(&d3), 779210);
    let d3: Vec<Bus> = "67,7,x,59,61"
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(p2(&d3), 1261476);
    let d3: Vec<Bus> = "1789,37,47,1889"
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(p2(&d3), 1202161486);
}
