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

pub fn read_data_space_saperator<T>(filename: &str) -> Result<Vec<T>, Error>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(filename)?;
    let data: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let mut v: Vec<T> = Vec::new();
    let mut s = String::new();
    for l in data {
        if l=="" {
            v.push(s.trim().parse().unwrap());
            s.clear()
        } else {
            s.push_str(&(" ".to_owned() + l.trim()))
        }

    }
    v.push(s.trim().parse().unwrap());
    Ok(v)
}