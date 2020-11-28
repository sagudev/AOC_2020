use crate::day::Day;

pub struct DayX {
    pub data: Vec<String>,
}

impl Day for DayX {
    fn new(data: Vec<String>) -> Box<dyn Day> {
        Box::new(Self { data })
    }
    fn p1(&self) -> String {
        let mut all = 0;
        for mass in self.data.iter() {
            all += do_calc(mass.parse::<i32>().unwrap());
        }
        format!("{}", all)
    }
    fn p2(&self) -> String {
        let mut all = 0;
        for mass in self.data.iter() {
            all += do_calc_ff(mass.parse::<i32>().unwrap());
        }
        format!("{}", all)
    }
}

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
