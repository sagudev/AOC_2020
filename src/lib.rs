use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
pub mod mach;
#[cfg(feature = "wasm")]
use crate::mach::Day;

pub fn read_data<P>(filename: P) -> Result<Vec<String>, Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut v = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        if !line.is_empty() {
            v.push(line);
        }
    }
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
    let mut data = Vec::new();
    for line in s.lines() {
        let line = line.trim();
        if !line.is_empty() {
            data.push(line.to_string());
        }
    }
    let day = day.new(data);
    vec![
        wasm_bindgen::JsValue::from_str(&day.p1()),
        wasm_bindgen::JsValue::from_str(&day.p2()),
    ]
    .into_boxed_slice()
}
