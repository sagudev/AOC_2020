use aoc::read_data_space_saperator;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

/// Passport
struct Passport {
    /// byr (Birth Year)
    byr: Option<usize>,
    /// iyr (Issue Year)
    iyr: Option<usize>,
    /// eyr (Expiration Year)
    eyr: Option<usize>,
    /// hgt (Height)
    hgt: Option<String>,
    /// hcl (Hair Color)
    hcl: Option<String>,
    /// ecl (Eye Color)
    ecl: Option<String>,
    /// pid (Passport ID)
    pid: Option<String>,
    #[allow(dead_code)]
    /// cid (Country ID)
    cid: Option<String>,
}

impl Passport {
    /*
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
    */
    fn is_data_valid(&self) -> bool {
        is_byr(self.byr.unwrap())
            && is_iyr(self.iyr.unwrap())
            && is_eyr(self.eyr.unwrap())
            && is_hgt(&self.hgt.as_ref().unwrap())
            && is_hcl(&self.hcl.as_ref().unwrap())
            && is_ecl(&self.ecl.as_ref().unwrap())
            && is_pid(&self.pid.as_ref().unwrap())
        // cid skipped
    }
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
        //&& self.cid.is_some()
    }
}

/// four digits; at least 1920 and at most 2002.
fn is_byr(byr: usize) -> bool {
    byr >= 1902 && byr <= 2002
}

/// four digits; at least 2010 and at most 2020.
fn is_iyr(iyr: usize) -> bool {
    iyr >= 2010 && iyr <= 2020
}

/// four digits; at least 2020 and at most 2030.
fn is_eyr(eyr: usize) -> bool {
    eyr >= 2020 && eyr <= 2030
}

/// exactly one of: amb blu brn gry grn hzl oth.
fn is_ecl(ecl: &str) -> bool {
    matches!(ecl, "amb" | "blu" | "grn" | "gry" | "brn" | "hzl" | "oth")
}

/// a nine-digit number, including leading zeroes.
fn is_pid(pid: &str) -> bool {
    pid.len() == 9 && pid[0..9].chars().all(|x| matches!(x, '0'..='9'))
}

/// a # followed by exactly six characters 0-9 or a-f.
fn is_hcl(hex: &str) -> bool {
    hex.len() == 7
        && hex.starts_with('#')
        && hex[1..7].chars().all(|x| match x {
            '0'..='9' => true,
            'a'..='f' => true,
            _ => false,
        })
}

/// a number followed by either cm or in:
/// If cm, the number must be at least 150 and at most 193.
/// If in, the number must be at least 59 and at most 76.
fn is_hgt(hgt: &str) -> bool {
    //println!("{}", &hgt[..=2]);
    match &hgt[hgt.len() - 2..] {
        "cm" => {
            if let Ok(cm) = hgt[..3].parse::<i32>() {
                cm >= 150 && cm <= 193
            } else {
                false
            }
        }
        "in" => {
            if let Ok(cm) = hgt[..2].parse::<i32>() {
                cm >= 59 && cm <= 76
            } else {
                false
            }
        }
        _ => false,
    }
}

impl FromStr for Passport {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(' ').collect();
        let mut byr: Option<usize> = None;
        let mut iyr: Option<usize> = None;
        let mut eyr: Option<usize> = None;
        let mut hgt: Option<String> = None;
        let mut hcl: Option<String> = None;
        let mut ecl: Option<String> = None;
        let mut pid: Option<String> = None;
        let mut cid: Option<String> = None;
        for f in v {
            //println!("-{}", &f[0..=3]);
            match &f[0..=3] {
                "byr:" => byr = Some(f.replace("byr:", "").parse()?),
                "iyr:" => iyr = Some(f.replace("iyr:", "").parse()?),
                "eyr:" => eyr = Some(f.replace("eyr:", "").parse()?),
                "hgt:" => hgt = Some(f.replace("hgt:", "")),
                "hcl:" => hcl = Some(f.replace("hcl:", "")),
                "ecl:" => ecl = Some(f.replace("ecl:", "")),
                "pid:" => pid = Some(f.replace("pid:", "")),
                "cid:" => cid = Some(f.replace("cid:", "")),
                _ => panic!(""),
            }
        }
        Ok(Self {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
        })
    }
}

fn is_valid(s: &str) -> bool {
    if let Ok(r) = s.parse::<Passport>() {
        r.is_valid()
    } else {
        false
    }
}

fn data_validation(s: &str) -> bool {
    if let Ok(r) = s.parse::<Passport>() {
        if r.is_valid() {
            return r.is_data_valid();
        }
    }
    false
}

fn p1(data: &[String]) -> usize {
    let mut c = 0;
    for d in data {
        if is_valid(d) {
            c += 1
        }
    }
    c
}

fn p2(data: &[String]) -> usize {
    let mut c = 0;
    for d in data {
        if data_validation(d) {
            c += 1
        }
    }
    c
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    // part 1
    let data: Vec<String> = read_data_space_saperator("./data/data4").unwrap();
    println!("Count the number of valid passports - those that have all required Passport. Treat cid as optional. In your batch file, how many passports are valid? {}", p1(&data));
    //part 2
    println!("Count the number of valid passports - those that have all required Passport and valid values. Continue to treat cid as optional. In your batch file, how many passports are valid? {}", p2(&data));
    Ok(())
}

#[test]
fn data_read() {
    println!(
        "{:?}",
        read_data_space_saperator::<String>("./data/data4").unwrap()[0]
    );
}

#[test]
fn calc() {
    let data: Vec<String> = vec![
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"
            .to_string(),
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929".to_string(),
        "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm".to_string(),
        "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in".to_string(),
    ];
    // part 1
    assert_eq!(is_valid(&data[0]), true);
    assert_eq!(is_valid(&data[1]), false);
    assert_eq!(is_valid(&data[2]), true);
    assert_eq!(is_valid(&data[3]), false);

    assert_eq!(p1(&data), 2);
    // part 2
    assert_eq!(is_byr(2002), true);
    assert_eq!(is_byr(2003), false);

    assert_eq!(is_hgt("60in"), true);
    assert_eq!(is_hgt("190cm"), true);
    assert_eq!(is_hgt("190in"), false);
    assert_eq!(is_hgt("190"), false);

    assert_eq!(is_hcl("#123abc"), true);
    assert_eq!(is_hcl("#123abz"), false);
    assert_eq!(is_hcl("123abc"), false);

    assert_eq!(is_ecl("brn"), true);
    assert_eq!(is_ecl("wat"), false);

    assert_eq!(is_pid("000000001"), true);
    assert_eq!(is_pid("0123456789"), false);

    let data: Vec<String> = vec![
        // invalid:
        "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string(),
        "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946".to_string(),
        "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".to_string(),
        "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007".to_string(),
        // valid:
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f".to_string(),
        "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
            .to_string(),
        "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022".to_string(),
        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string(),
    ];

    assert_eq!(data_validation(&data[0]), false);
    assert_eq!(data_validation(&data[1]), false);
    assert_eq!(data_validation(&data[2]), false);
    assert_eq!(data_validation(&data[3]), false);
    assert_eq!(data_validation(&data[4]), true);
    assert_eq!(data_validation(&data[5]), true);
    assert_eq!(data_validation(&data[6]), true);
    assert_eq!(data_validation(&data[7]), true);

    assert_eq!(p2(&data), 4);
}
