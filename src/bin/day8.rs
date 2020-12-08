use aoc::read_data;
use std::collections::HashSet;
use std::convert::Infallible;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Operation {
    type Err = Infallible;
    // 4 shiny brown bags
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jmp" => Ok(Operation::Jmp),
            "acc" => Ok(Operation::Acc),
            "nop" => Ok(Operation::Nop),
            _ => panic!("wrong op"),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction(Operation, i32);

impl Instruction {
    /// return true if no work to do
    fn flip(&mut self) -> bool {
        match self.0 {
            Operation::Jmp => self.0 = Operation::Nop,
            Operation::Nop => self.0 = Operation::Jmp,
            _ => return true,
        }
        false
    }
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    // 4 shiny brown bags
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //println!("{}", s);
        Ok(Self(s[..3].trim().parse().unwrap(), s[3..].trim().parse()?))
    }
}

/// asm executor
#[derive(Default, Debug)]
struct ASX {
    acc: i32,
    idx: usize,
}

impl ASX {
    fn new() -> Self {
        Default::default()
    }

    fn reset(&mut self) {
        self.idx = 0;
        self.acc = 0;
    }

    /// return true on graceful shutdown
    /// or false on infinite loop
    fn exec(&mut self, data: &[Instruction]) -> bool {
        let mut executed: HashSet<usize> = HashSet::new();
        loop {
            if executed.contains(&self.idx) {
                break false;
            }
            if self.idx >= data.len() {
                break true;
            }
            executed.insert(self.idx);
            match &data[self.idx].0 {
                Operation::Nop => self.idx += 1,
                Operation::Acc => {
                    self.acc += data[self.idx].1;
                    self.idx += 1
                }
                Operation::Jmp => self.idx = (self.idx as i32 + data[self.idx].1) as usize,
            }
        }
    }
}

fn p1(data: &[Instruction]) -> i32 {
    let mut machine = ASX::new();
    machine.exec(data);
    machine.acc
}

fn p2(data: &[Instruction]) -> i32 {
    let mut data: Vec<Instruction> = data.to_vec();
    let mut machine = ASX::new();
    for i in 0..data.len() {
        if data[i].flip() {
            continue;
        }
        if machine.exec(&data) {
            break;
        }
        machine.reset();
        data[i].flip();
    }
    machine.acc
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<Instruction> = read_data("./data/data8").unwrap();
    // part 1
    println!("Run your copy of the boot code. Immediately before any instruction is executed a second time, what value is in the accumulator? {}", p1(&data));
    // part 2
    println!("Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to jmp). What is the value of the accumulator after the program terminates? {}", p2(&data));
    Ok(())
}

#[test]
fn data_read() {
    println!("{:?}", read_data::<Instruction>("./data/data8").unwrap());
}

#[test]
fn calc() {
    let data: Vec<&str> = vec![
        "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ];
    let data: Vec<Instruction> = data.iter().map(|x| x.parse().unwrap()).collect();
    // part 1
    let mut machine = ASX::new();
    assert_eq!(machine.exec(&data), false);
    assert_eq!(machine.acc, 5);
    assert_eq!(machine.idx, 1);
    assert_eq!(p1(&data), 5);
    // part 2
    assert_eq!(p2(&data), 8);
}
