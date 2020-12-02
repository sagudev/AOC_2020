use std::str::FromStr;

/// Every day impl these
pub trait Day<T: FromStr> {
    /// Part 1
    fn p1(&self) -> String;
    /// Part 2
    fn p2(&self) -> String;
    /// Insert data
    fn new(data: Vec<T>) -> Self;
}

pub mod day1;
pub mod dayx;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
#[derive(Debug)]
/// List of all days
pub enum Days {
    Dayx,
    Day1,
}

#[cfg(feature = "wasm")]
impl Days {
    /// Create new day placeholder from data
    pub fn new(&self, data: Vec<String>) -> Box<dyn Day> {
        match self {
            Days::Dayx => Box::new(crate::mach::dayx::DayX::new(data)),
            Days::Day1 => Box::new(crate::mach::day1::Day1::new(crate::data_vec(data).unwrap())),
        }
    }
}
