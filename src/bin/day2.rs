use aoc::mach::Day;
use aoc::*;

fn main() {
    println!("Hello, Advent Of Code 2020!");
    // read data
    let data = read_data("./data/data2").unwrap();
    let day2 = mach::day2::Day2::new(data);
    // part 1
    let p1 = day2.p1().parse::<usize>().unwrap();
    println!("Part 1: {}", p1);
    //assert_eq!(3376997, p1);
    //part 2
    let p2 = day2.p2().parse::<usize>().unwrap();
    println!("Part 2: {}", p2);
    //assert_eq!(5062623, p2);
}

#[test]
/// Am I getting right data?
fn data_read() {
    println!(
        "{:?}",
        read_data::<aoc::mach::day2::Password>("./data/data2").unwrap()
    );
}
