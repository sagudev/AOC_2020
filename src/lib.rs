use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::io::Error;

pub fn read_data<P>(filename: P) -> Result<Vec<String>, Error>
where P: AsRef<Path>, {
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