use std::fmt::Debug;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::str::FromStr;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
pub mod mach;

pub fn read_data<T>(filename: &str) -> Result<Vec<T>, Error>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(filename)?;
    let v = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().parse::<T>().unwrap())
        .collect();
    Ok(v)
}

#[cfg(feature = "wasm")]
pub fn data_vec<T>(vec: Vec<String>) -> Result<Vec<T>, Error>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let v = vec.iter().map(|line| line.parse::<T>().unwrap()).collect();
    Ok(v)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(feature = "wasm")]
#[allow(unused_macros)]
/// Just in case (printer macro)
macro_rules! printer {
    // console.log import
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
/// this is called from js
/// data is split by newline
pub fn js_mach(day: mach::Days, s: String) -> Box<[JsValue]> {
    let day = day.new(
        s.lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect(),
    );
    vec![
        wasm_bindgen::JsValue::from_str(&day.p1()),
        wasm_bindgen::JsValue::from_str(&day.p2()),
    ]
    .into_boxed_slice()
}
