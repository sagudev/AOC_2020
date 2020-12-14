use aoc::read_data;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
// loc, binary
struct Mem(String, String);

#[derive(Debug)]
struct Section {
    mask: String,
    mem: Vec<Mem>,
}

impl Section {
    fn from_v(v: Vec<String>) -> Self {
        let mut mem = Vec::new();
        for d in v.iter().skip(1) {
            let start_mem = d.find('[').unwrap();
            let end_mem = d.find(']').unwrap();
            let e = d.find('=').unwrap();
            mem.push(Mem(
                format!(
                    "{:0width$b}",
                    d[start_mem + 1..end_mem].parse::<usize>().unwrap(),
                    width = 36
                ),
                format!(
                    "{:0width$b}",
                    d[e + 2..].parse::<usize>().unwrap(),
                    width = 36
                ),
            ));
        }
        Self {
            mask: v[0][7..].to_string(),
            mem,
        }
    }
}

fn trans(data: &[String]) -> Vec<Section> {
    let mut v = Vec::new();
    let mut r = Vec::new();
    for d in data {
        if d.contains("mask") && !v.is_empty() {
            r.push(Section::from_v(v.clone()));
            v.clear()
        }
        v.push(d.to_string());
    }
    r.push(Section::from_v(v));
    r
}

struct Machine {
    mem: HashMap<usize, usize>,
}

impl Machine {
    fn new() -> Self {
        Self {
            mem: HashMap::new(),
        }
    }
    fn aps(&mut self, s: &Section) {
        let mask: Vec<char> = s.mask.chars().collect();
        for m in &s.mem {
            let mut masked = String::new();
            for (i, c) in m.1.chars().enumerate() {
                match mask[i] {
                    'X' => masked += &c.to_string(),
                    '1' => masked += "1",
                    '0' => masked += "0",
                    _ => panic!("err"),
                }
            }
            //println!("{}: {}", m.0, usize::from_str_radix(&masked, 2).unwrap());
            self.mem.insert(
                usize::from_str_radix(&m.0, 2).unwrap(),
                usize::from_str_radix(&masked, 2).unwrap(),
            );
        }
    }
    fn aps2(&mut self, s: &Section) {
        let mask: Vec<char> = s.mask.chars().collect();
        for m in &s.mem {
            let mut masked = String::new();
            let mut num: usize = 0;
            for (i, c) in m.0.chars().enumerate() {
                match mask[i] {
                    'X' => {
                        masked += "X";
                        num += 1
                    }
                    '1' => masked += "1",
                    '0' => masked += &c.to_string(),
                    _ => panic!("err"),
                }
            }
            for d in 0..2_i32.pow(num as u32) {
                let s = format!("{:0width$b}", d, width = num);
                let mut s = s.chars();
                self.mem.insert(
                    usize::from_str_radix(
                        &masked.chars().map(|x| {
                            if x == 'X' {
                                s.next().unwrap()
                            } else {
                                x
                            }
                        }).collect::<String>(),
                        2,
                    )
                    .unwrap(),
                    usize::from_str_radix(&m.1, 2).unwrap(),
                );
            }
            //println!("{}: {}", m.0, usize::from_str_radix(&masked, 2).unwrap());
        }
    }
}

//u8::from_str_radix(&s[..7].replace('F', "0").replace('B', "1"), 2).unwrap()

fn p1(data: &[Section]) -> usize {
    let mut m = Machine::new();
    for d in data {
        m.aps(d);
    }
    let mut res = 0;
    for t in m.mem.values() {
        res += t
    }
    res
}

fn p2(data: &[Section]) -> usize {
    let mut m = Machine::new();
    for d in data {
        m.aps2(d);
    }
    let mut res = 0;
    for t in m.mem.values() {
        res += t
    }
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<String> = read_data("./data/data14").unwrap();
    let data: Vec<Section> = trans(&data);
    // part 1
    println!(
        " What is the sum of all values left in memory after it completes?: {}",
        p1(&data)
    );
    // part 2
    println!("Execute the initialization program using an emulator for a version 2 decoder chip. What is the sum of all values left in memory after it completes? {}", p2(&data));
    Ok(())
}

#[test]
fn data_read() {
    println!("{:?}", read_data::<String>("./data/data14").unwrap());
}

#[test]
fn calc() {
    let data: Vec<String> = vec![
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
        "mem[8] = 11".to_string(),
        "mem[7] = 101".to_string(),
        "mem[8] = 0".to_string(),
    ];
    let data: Vec<Section> = trans(&data);
    // part 1
    let mut m = Machine::new();
    m.aps(&data[0]);
    assert_eq!(m.mem[&8], 64);
    assert_eq!(m.mem[&7], 101);
    assert_eq!(p1(&data), 165);
    // part 2
    let data: Vec<String> = vec![
        "mask = 000000000000000000000000000000X1001X".to_string(),
        "mem[42] = 100".to_string(),
        "mask = 00000000000000000000000000000000X0XX".to_string(),
        "mem[26] = 1".to_string(),
    ];
    let data: Vec<Section> = trans(&data);
    let mut m = Machine::new();
    m.aps2(&data[0]);
    assert_eq!(m.mem[&26], 100);
    assert_eq!(m.mem[&27], 100);
    assert_eq!(m.mem[&58], 100);
    assert_eq!(m.mem[&59], 100);
    m.aps2(&data[1]);
    assert_eq!(m.mem[&16], 1);
    assert_eq!(m.mem[&17], 1);
    assert_eq!(m.mem[&18], 1);
    assert_eq!(m.mem[&19], 1);
    assert_eq!(m.mem[&24], 1);
    assert_eq!(m.mem[&25], 1);
    assert_eq!(m.mem[&26], 1);
    assert_eq!(m.mem[&27], 1);
    assert_eq!(p2(&data), 208);
}
