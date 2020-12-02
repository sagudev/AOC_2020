use std::fmt::Debug;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::str::FromStr;

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
