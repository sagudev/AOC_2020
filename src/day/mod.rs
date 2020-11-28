pub trait Day {
    fn p1(&self) -> String;
    fn p2(&self) -> String;
    fn new(data: Vec<String>) -> Self;
}

pub mod x;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
#[derive(Debug)]
pub enum Days {
    Dayx,
}

#[cfg(feature = "wasm")]
impl Days {
    pub fn new(&self, data: Vec<String>) -> impl Day {
        match self {
            Days::Dayx => crate::day::x::DayX::new(data)
        }
    }
}