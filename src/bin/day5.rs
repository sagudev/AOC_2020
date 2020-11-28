use aoc::mach::Day;
use aoc::*;

fn main() {
    println!("Hello, Advent Of Code 2020!");
    // read data
    let data = read_data("./data/data5").unwrap();
    let day5 = mach::day5::Day5::new(data);
    // part 1
    let p1 = day5.p1().parse::<i32>().unwrap();
    println!("Fuel needed (part 1): {}", p1);
    assert_eq!(3376997 + 1, p1);
    //part 2
    let p2 = day5.p2().parse::<i32>().unwrap();
    println!("Fuel needed (part 2): {}", p2);
    assert_eq!(5062623 + 1, p2);
}

#[test]
/// Am I getting right data?
fn data_read() {
    println!("{:?}", read_data("./data/data5").unwrap());
}
