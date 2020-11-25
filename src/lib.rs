use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

/// Do not have any empty lines at end
pub fn read_data(filename: &str) -> std::io::Result<Vec<String>> {
    /* let file = File::open(fname)?;

    let file_reader = BufReader::new(file);
    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect()) */
    BufReader::new(File::open("data/".to_owned() + filename)?).lines().collect()
}