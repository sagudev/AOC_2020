use aoc::read_data;
use std::error::Error;

/// fuel for one module (mass)
fn do_calc(mass: i32) -> i32 {
    (mass as f64 / 3.0).trunc() as i32 - 2
}

/// all fuel by module
fn do_calc_ff(mass: i32) -> i32 {
    let mut all = 0;
    let mut old = mass;
    loop {
        old = do_calc(old);
        if old <= 0 {
            break;
        };
        all += old;
    }
    all
}

fn p1(data: &[String]) -> i32 {
    let mut all = 0;
    for mass in data {
        all += do_calc(mass.parse::<i32>().unwrap());
    }
    all
}

fn p2(data: &[String]) -> i32 {
    let mut all = 0;
    for mass in data {
        all += do_calc_ff(mass.parse::<i32>().unwrap());
    }
    all
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<String> = read_data("./data/datax").unwrap();
    // part 1
    println!("Fuel needed (part 1): {}", p1(&data));
    // part 2
    println!("Fuel needed (part 2): {}", p2(&data));
    Ok(())
}

#[test]
fn data_read() {
    println!("{:?}", read_data::<String>("./data/datax").unwrap());
}

#[test]
fn calc() {
    // part 1
    assert_eq!(do_calc(12), 2);
    assert_eq!(do_calc(14), 2);
    assert_eq!(do_calc(1969), 654);
    assert_eq!(do_calc(100756), 33583);
    // part 2
    assert_eq!(do_calc_ff(100756), 50346);
    assert_eq!(do_calc_ff(1969), 966);
    assert_eq!(do_calc_ff(14), 2);
}
