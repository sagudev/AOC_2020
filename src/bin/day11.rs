use aoc::read_data;
use std::cmp::min;
use std::convert::Infallible;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Floor,
    /// free seat
    L,
    /// taken seat
    Seat,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Floor => write!(f, "."),
            Cell::L => write!(f, "L"),
            Cell::Seat => write!(f, "#"),
        }
    }
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Cell::Floor,
            'L' => Cell::L,
            '#' => Cell::Seat,
            _ => panic!("wrong Cell"),
        }
    }
}

#[derive(Debug, Clone)]
struct Row {
    row: Vec<Cell>,
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for d in &self.row {
            s = format!("{}{}", s, d);
        }
        write!(f, "{}", s)
    }
}

impl Row {
    /// returns number of L and S
    fn check_row(&self, i: usize) -> (usize, usize) {
        let mut l = 0;
        let mut s = 0;
        if i != 0 && self.row.len() > i - 1 {
            match self.row[i - 1] {
                Cell::Floor => (),
                Cell::L => l += 1,
                Cell::Seat => s += 1,
            }
        }
        if self.row.len() > i + 1 {
            match self.row[i + 1] {
                Cell::Floor => (),
                Cell::L => l += 1,
                Cell::Seat => s += 1,
            }
        }
        match self.row[i] {
            Cell::Floor => (),
            Cell::L => l += 1,
            Cell::Seat => s += 1,
        }
        (l, s)
    }
    fn check_my_row(&self, i: usize) -> (usize, usize) {
        let mut l = 0;
        let mut s = 0;
        if i != 0 && self.row.len() > i - 1 {
            match self.row[i - 1] {
                Cell::Floor => (),
                Cell::L => l += 1,
                Cell::Seat => s += 1,
            }
        }
        if self.row.len() > i + 1 {
            match self.row[i + 1] {
                Cell::Floor => (),
                Cell::L => l += 1,
                Cell::Seat => s += 1,
            }
        }
        (l, s)
    }
}

impl FromStr for Row {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            row: s.trim().chars().map(Cell::from_char).collect(),
        })
    }
}

fn cikel(data: &mut Vec<Row>) -> bool {
    let mut changed = false;
    let mut res = data.clone();
    for i in 0..data.len() {
        for j in 0..data[0].row.len() {
            let c = &data[i].row[j];
            if *c != Cell::Floor {
                let mut s = 0;
                if data.len() > i + 1 {
                    let t = data[i + 1].check_row(j);
                    s += t.1;
                }
                if i != 0 && data.len() > i - 1 {
                    let t = data[i - 1].check_row(j);
                    s += t.1;
                }
                let t = data[i].check_my_row(j);
                s += t.1;
                //println!("||{} {}", l, s);
                if *c == Cell::L && s == 0 {
                    res[i].row[j] = Cell::Seat;
                    changed = true;
                } else if *c == Cell::Seat && s >= 4 {
                    res[i].row[j] = Cell::L;
                    changed = true;
                }
            }
        }
    }
    *data = res;
    changed
}

fn p1(data: &[Row]) -> usize {
    let mut data = data.to_vec();
    let mut count = 0;
    loop {
        if !cikel(&mut data) {
            break;
        }
    }
    for i in 0..data.len() {
        for j in 0..data[0].row.len() {
            if data[i].row[j] == Cell::Seat {
                count += 1;
            }
        }
    }
    count
}

// this could be more optimized
fn direction(data: &[Row], col: usize, row: usize) -> usize {
    /* let mut c = 0;
    for d in data {
        if d.row[row] == Cell::Seat {
            c += 1;
        }
    }
    for i in 0..data[0].row.len() {
        if data[col].row[i] == Cell::Seat {
            c += 1;
        }
    }
    let mini = min(data[0].row.len() - row, data.len() - col);
    for i in 0..2 * mini {
        if col + i >= mini
            && row + i >= mini
            && data[col + i - mini].row[row + i - mini] == Cell::Seat
        {
            c += 1;
        }
        if col + i >= mini
            && row + mini >= i
            && data[col + i - mini].row[row + mini - i] == Cell::Seat
        {
            c += 1;
        }
    }
    c */
    let mut c = 0;
    //println!("{}", data[col].row[row]);
    let row_len = data[0].row.len();
    let col_len = data.len();
    // col
    for i in 1..col_len {
        //print!("{}", data[col + i].row[row]);
        if col + i < col_len && data[col + i].row[row] != Cell::Floor {
            if data[col + i].row[row] == Cell::Seat {
                c += 1;
            }
            break;
        }
    }
    //println!("c {}", c);
    for i in 1..col_len {
        //print!("{}", data[col - i].row[row]);
        if col >= i && data[col - i].row[row] != Cell::Floor {
            if data[col - i].row[row] == Cell::Seat {
                c += 1;
            }
            break;
        }
    }
    //println!("c {}", c);
    // row
    for i in 1..row_len {
        if row + i < row_len && data[col].row[row + i] != Cell::Floor {
            if data[col].row[row + i] == Cell::Seat {
                c += 1;
            }
            break;
        }
    }
    //println!("r {}", c);
    for i in 1..row_len {
        if row >= i && data[col].row[row - i] != Cell::Floor {
            if data[col].row[row - i] == Cell::Seat {
                c += 1;
            }
            break;
        }
    }
    //println!("r {}", c);
    // col + row
    let mini = min(row_len, col_len);
    for i in 1..mini {
        if row + i < row_len && col + i < col_len && data[col + i].row[row + i] != Cell::Floor {
            if data[col + i].row[row + i] == Cell::Seat {
                c += 1;
            }
            break;
        }
    }
    //println!("cr {}", c);
    for i in 1..mini {
        if row >= i && col >= i && data[col - i].row[row - i] != Cell::Floor {
            if data[col - i].row[row - i] == Cell::Seat {
                c += 1;
            }
            break;
        }
    }
    //println!("cr {}", c);
    // col - row
    for i in 1..mini {
        // row >= i && col + i <= mini &&
        if row >= i && col + i < col_len && data[col + i].row[row - i] != Cell::Floor {
            //println!("{:?}", data[col + i].row[row - i]);
            if data[col + i].row[row - i] == Cell::Seat {
                c += 1;
            }
            break;
        }
    }
    //println!("rc {}", c);
    for i in 1..mini {
        // row + i <= mini && col >= i &&
        if col >= i && row + i < row_len && data[col - i].row[row + i] != Cell::Floor {
            if data[col - i].row[row + i] == Cell::Seat {
                c += 1;
            }
            break;
        }
    }
    //println!("rc {}", c);
    c
}

fn cikel2(data: &mut Vec<Row>) -> bool {
    let mut changed = false;
    let mut res = data.clone();
    for i in 0..data.len() {
        for j in 0..data[0].row.len() {
            let c = &data[i].row[j];
            if *c != Cell::Floor {
                let s = direction(&data, i, j);
                print!("{}", s);
                if *c == Cell::L && s == 0 {
                    res[i].row[j] = Cell::Seat;
                    changed = true;
                } else if *c == Cell::Seat && s >= 5 {
                    res[i].row[j] = Cell::L;
                    changed = true;
                }
            } else {
                print!(" ");
            }
        }
        println!()
    }
    *data = res;
    changed
}

fn p2(data: &[Row]) -> usize {
    let mut data = data.to_vec();
    let mut count = 0;
    loop {
        for d in &data {
            println!("{}", d);
        }
        if !cikel2(&mut data) {
            break;
        }
    }
    for i in 0..data.len() {
        for j in 0..data[0].row.len() {
            if data[i].row[j] == Cell::Seat {
                count += 1;
            }
        }
    }
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<Row> = read_data("./data/data11").unwrap();
    // part 1
    println!("How many seats end up occupied? {}", p1(&data));
    // part 2
    println!("Given the new visibility method and the rule change for occupied seats becoming empty, once equilibrium is reached, how many seats end up occupied? {}", p2(&data));
    Ok(())
}

#[test]
fn data_read() {
    println!("{:?}", read_data::<Row>("./data/data11").unwrap());
}

#[test]
fn calc() {
    let data = vec![
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ];
    let data: Vec<Row> = data
        .iter()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    // part 1
    assert_eq!(p1(&data), 37);
    // part 2
    let dir_data = vec![
        ".......#.",
        "...#.....",
        ".#.......",
        ".........",
        "..#L....#",
        "....#....",
        ".........",
        "#........",
        "...#.....",
    ];
    let dir_data: Vec<Row> = dir_data
        .iter()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    assert_eq!(direction(&dir_data, 4, 3), 8);

    let dir_data = vec![".............", ".L.L.#.#.#.#.", "............."];
    let dir_data: Vec<Row> = dir_data
        .iter()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    assert_eq!(direction(&dir_data, 1, 1), 0);

    let dir_data: Vec<Row> = [
        ".##.##.", "#.#.#.#", "##...##", "...L...", "##...##", "#.#.#.#", ".##.##.",
    ]
    .iter()
    .map(|line| line.trim().parse().unwrap())
    .collect();
    assert_eq!(direction(&dir_data, 3, 3), 0);

    let dir_data: Vec<Row> = ["#.##.##.##", "#######.##", "#.#.#..#..", "####.##.##"]
        .iter()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    assert_eq!(direction(&dir_data, 0, 2), 5);
    assert_eq!(direction(&dir_data, 0, 8), 5);
    assert_eq!(p2(&data), 26);
}
