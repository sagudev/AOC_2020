use aoc::day::Day;
use aoc::*;

fn main() {
    println!("Hello, Advent Of Code 2020!");
    // part 1
    let data = read_data("./data/datax").unwrap();
    let dayx = day::x::DayX::new(data);
    let p1 = dayx.p1().parse::<i32>().unwrap();
    println!("Fuel needed (part 1): {}", p1);
    assert_eq!(3376997, p1);
    //part 2
    let p2 = dayx.p2().parse::<i32>().unwrap();
    println!("Fuel needed (part 2): {}", p2);
    assert_eq!(5062623, p2);
}

#[test]
fn data_read() {
    println!("{:?}", read_data("./data/datax").unwrap());
}