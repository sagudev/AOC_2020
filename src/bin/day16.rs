use aoc::read_data_space_saperator2;
use std::collections::HashMap;
use std::error::Error;

type Range = std::ops::RangeInclusive<usize>;

fn range(s: &str) -> Range {
    //println!("{:?}", s);
    let mut s = s.split('-');
    Range::new(
        s.next().unwrap().parse().unwrap(),
        s.next().unwrap().parse().unwrap(),
    )
}

fn can_be(v: &[usize], r: (Range, Range)) -> bool {
    for d in v {
        if !(r.0.contains(&d) || r.1.contains(&d)) {
            return false;
        }
    }
    true
}

/// Struct of data
#[derive(Debug, Clone)]
struct DATA {
    dater: HashMap<String, (Range, Range)>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl DATA {
    fn new(v: Vec<String>) -> Self {
        let mut dater = HashMap::new();
        for d in v[0].split('|').skip(1) {
            //println!("{:?}", d)
            let mut bi = d.split(':');
            let b1 = bi.next().unwrap();
            let mut b2 = bi.next().unwrap().split("or");
            dater.insert(
                b1.trim().to_string(),
                (
                    range(b2.next().unwrap().trim()),
                    range(b2.next().unwrap().trim()),
                ),
            );
        }
        //println!("{:?}", v[2].split('|').skip(0));
        let my_ticket = v[1]
            .split('|')
            .nth(2)
            .unwrap()
            .split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let nearby_tickets = v[2]
            .split('|')
            .skip(2)
            .map(|x| x.split(',').map(|x| x.parse().unwrap()).collect())
            .collect();
        Self {
            dater,
            my_ticket,
            nearby_tickets,
        }
    }

    fn is_unvalid(&mut self, idx: usize) -> Option<usize> {
        for d in self.nearby_tickets[idx].clone() {
            let mut ok = false;
            for r in self.dater.values() {
                if r.0.contains(&d) || r.1.contains(&d) {
                    ok = true;
                    break;
                }
            }
            if !ok {
                self.nearby_tickets.remove(idx);
                return Some(d);
            }
        }
        None
    }
}

fn p1(data: &mut DATA) -> usize {
    let mut res = 0;
    let mut i = 0;
    while i < data.nearby_tickets.len() {
        if let Some(x) = data.is_unvalid(i) {
            res += x;
        } else {
            i += 1;
        }
    }
    res
}

fn m2(data: &DATA) -> HashMap<String, usize> {
    // first element is vec of all 1st numbers for range detector
    let mut rd: Vec<Vec<usize>> = data.my_ticket.iter().map(|x| [*x].to_vec()).collect();
    for d in &data.nearby_tickets {
        for i in 0..rd.len() {
            rd[i].push(d[i])
        }
    }
    //println!("{:?}", rd);
    let mut h: HashMap<String, Vec<bool>> = HashMap::new();
    let mut res = HashMap::new();
    let mut taken: Vec<bool> = vec![false; data.dater.len()];
    for d in rd {
        for (k, r) in &data.dater {
            let e = h.entry(k.to_string()).or_insert(Vec::new());
            e.push(can_be(&d, r.clone()));
            //print!("{}", can_be(&d, r.clone()));
        }
        //println!("{:?}", d);
    }
    //println!("{:?}", h);
    while !taken.iter().all(|&x| x) {
        //println!("|");
        for (k, v) in &h {
            //println!("{}", v.iter().filter(|x| **x).count());
            if v.iter().filter(|x| **x).count() == 1 {
                let i = v.iter().position(|x| *x).unwrap();
                taken[i] = true;
                //println!("{}", k);
                res.insert(k.to_owned(), data.my_ticket[i]);
            }
        }
        for (i, t) in taken.iter().enumerate() {
            if *t {
                for v in h.values_mut() {
                    //println!("{}", i);
                    v[i] = false;
                }
            }
        }
    }

    //println!("{:?}", h);
    res
}

fn p2(data: &DATA) -> usize {
    let m = m2(data);
    let mut res = 1;
    for (k, v) in m {
        if k.contains("departure") {
            res *= v;
        }
    }
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<String> = read_data_space_saperator2("./data/data16", '|').unwrap();
    let mut data = DATA::new(data);
    // part 1
    println!("Consider the validity of the nearby tickets you scanned. What is your ticket scanning error rate? {}", p1(&mut data));
    // part 2
    println!(
        "What do you get if you multiply those six values together? {}",
        p2(&data)
    );
    Ok(())
}

#[test]
fn data_read16() {
    println!(
        "{:?}",
        read_data_space_saperator2::<String>("./data/data16", '|').unwrap()
    );
}

#[test]
fn calc16() {
    let data = vec![
        "|class: 1-3 or 5-7|row: 6-11 or 33-44|seat: 13-40 or 45-50".to_string(),
        "|your ticket:|7,1,14".to_string(),
        "|nearby tickets:|7,3,47|40,4,50|55,2,20|38,6,12".to_string(),
    ];
    let mut data = DATA::new(data);
    //println!("{:?}", data);
    // part 1
    assert_eq!(data.is_unvalid(0), None);
    assert_eq!(data.clone().is_unvalid(1), Some(4));
    assert_eq!(data.clone().is_unvalid(2), Some(55));
    assert_eq!(data.clone().is_unvalid(3), Some(12));
    assert_eq!(p1(&mut data), 71);
    // part 2
    let data = vec![
        "|class: 0-1 or 4-19|row: 0-5 or 8-19|seat: 0-13 or 16-19".to_string(),
        "|your ticket:|11,12,13".to_string(),
        "|nearby tickets:|3,9,18|15,1,5|5,14,9".to_string(),
    ];
    let mut data = DATA::new(data);
    p1(&mut data);
    let m = m2(&data);
    assert_eq!(m.get("class"), Some(&12));
    assert_eq!(m.get("row"), Some(&11));
    assert_eq!(m.get("seat"), Some(&13));
}
