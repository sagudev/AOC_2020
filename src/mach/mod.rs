/// Every day impl these
pub trait Day {
    /// Part 1
    fn p1(&self) -> String;
    /// Part 2
    fn p2(&self) -> String;
    /// Insert data
    fn new(data: Vec<String>) -> Self;
}

pub mod dayx;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
#[derive(Debug)]
/// List of all days
pub enum Days {
    Dayx,
}

#[cfg(feature = "wasm")]
impl Days {
    /// Create new day placeholder from data
    pub fn new(&self, data: Vec<String>) -> impl Day {
        match self {
            Days::Dayx => crate::mach::dayx::DayX::new(data)
        }
    }
}