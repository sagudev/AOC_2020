use aoc::mach::Day;
use aoc::*;

fn main() {
    println!("Hello, Advent Of Code 2020!");
    // read data
    let data = read_data("./data/data1").unwrap();
    let day1 = mach::day1::Day1::new(data);
    // part 1
    let p1 = day1.p1().parse::<i32>().unwrap();
    println!("Part 1: {}", p1);
    //part 2
    let p2 = day1.p2().parse::<i32>().unwrap();
    println!("Part 2: {}", p2);
}

#[test]
/// Am I getting right data?
fn data_read() {
    println!("{:?}", read_data("./data/data1").unwrap());
}
