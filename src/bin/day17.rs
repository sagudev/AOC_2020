use aoc::read_data;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::convert::Infallible;
use std::error::Error;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Active,
    /// free seat
    Inactive,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Inactive => write!(f, "."),
            Cell::Active => write!(f, "#"),
        }
    }
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Cell::Inactive,
            '#' => Cell::Active,
            _ => panic!("wrong Cell"),
        }
    }
    fn is_active(&self) -> bool {
        *self == Cell::Active
    }
}

#[derive(Debug, Clone)]
struct Row {
    row: VecDeque<Cell>,
}

impl Row {
    // returns number of A
    fn check_row(&self, i: usize) -> usize {
        let mut a = self.check_my_row(i);
        if i < self.row.len() && self.row[i].is_active() {
            a += 1
        }
        a
    }
    fn check_my_row(&self, i: usize) -> usize {
        let mut a = 0;
        if i != 0 && self.row.len() > (i - 1) && self.row[i - 1].is_active() {
            a += 1
        }
        if self.row.len() > (i + 1) && self.row[i + 1].is_active() {
            a += 1
        }
        a
    }
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

impl FromStr for Row {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("{}", s);
        Ok(Self {
            row: s.trim().chars().map(Cell::from_char).collect(),
        })
    }
}

#[derive(Debug, Clone)]
struct D2 {
    yx: VecDeque<Row>,
}

impl std::fmt::Display for D2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for d in &self.yx {
            s = format!("{}{}\n", s, d);
        }
        write!(f, "{}", s)
    }
}

impl D2 {
    fn check(&self, i: usize, j: usize) -> usize {
        let mut a = 0;
        // check in this z
        a += self.yx[i].check_my_row(j);
        if self.yx.len() > i + 1 {
            a += self.yx[i + 1].check_row(j);
        }
        if i != 0 && self.yx.len() > i - 1 {
            a += self.yx[i - 1].check_row(j);
        }
        a
    }
    /// check in other z (no my)
    fn checkz(&self, i: usize, j: usize) -> usize {
        let mut a = 0;
        // check in this z
        a += self.yx[i].check_row(j);
        if self.yx.len() > i + 1 {
            a += self.yx[i + 1].check_row(j);
        }
        if i != 0 && self.yx.len() > i - 1 {
            a += self.yx[i - 1].check_row(j);
        }
        a
    }
}

#[derive(Debug, Clone)]
struct D3 {
    rows: usize,
    cols: usize,
    z: HashMap<i32, D2>,
}

impl D3 {
    fn insert0(&mut self, z: i32) {
        let r0 = VecDeque::from_iter(vec![Cell::Inactive; self.rows]);
        self.z.insert(
            z,
            D2 {
                yx: VecDeque::from_iter(vec![Row { row: r0 }; self.cols]),
            },
        );
    }
    fn extend(&mut self, min: i32, max: i32) {
        // extend every
        for d2 in self.z.values_mut() {
            for c in d2.yx.iter_mut() {
                c.row.push_front(Cell::Inactive);
                c.row.push_back(Cell::Inactive);
            }
            d2.yx.push_front(Row {
                row: VecDeque::from_iter(vec![Cell::Inactive; self.rows + 2]),
            });
            d2.yx.push_back(Row {
                row: VecDeque::from_iter(vec![Cell::Inactive; self.rows + 2]),
            });
        }
        self.rows += 2;
        self.cols += 2;
        // extend z
        self.insert0(min - 1);
        self.insert0(max + 1);
    }
}

fn trans(yx: Vec<Row>) -> D3 {
    let mut z = HashMap::new();
    let rows = yx[0].row.len();
    let cols = yx.len();
    z.insert(
        0,
        D2 {
            yx: VecDeque::from_iter(yx.iter().cloned()),
        },
    );
    D3 { rows, cols, z }
}

impl std::fmt::Display for D3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (k, v) in &self.z {
            s = format!("{}|| {} ||\n{}\n", s, k, v);
        }
        write!(f, "{}", s)
    }
}

fn cikel(data: &mut D3) {
    let (min, max) = (*data.z.keys().min().unwrap(), *data.z.keys().max().unwrap());
    // extend place
    data.extend(min, max);
    let mut res = data.clone();
    for (z, d2) in &data.z {
        //println!("||");
        //println!("{}", d2);
        for i in 0..d2.yx.len() {
            let d = &d2.yx[i];
            for j in 0..d.row.len() {
                let cell = &d.row[j];
                // check cell
                // check in this z
                let mut a = d2.check(i, j);

                // check in other z
                if let Some(x) = data.z.get(&(z - 1)) {
                    a += x.checkz(i, j);
                }
                if let Some(x) = data.z.get(&(z + 1)) {
                    a += x.checkz(i, j);
                }
                /*
                If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
                If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
                */
                let c = &mut res.z.get_mut(z).unwrap().yx[i].row[j];
                match cell {
                    Cell::Active => {
                        if !(a == 3 || a == 2) {
                            *c = Cell::Inactive;
                        }
                    }
                    Cell::Inactive => {
                        if a == 3 {
                            *c = Cell::Active;
                        }
                    }
                }
            }
            //println!();
        }
    }
    *data = res;
}

fn p1(data: &D3) -> usize {
    let mut data = data.clone();
    let mut count = 0;
    // 6 ciklov
    for _ in 1..=6 {
        cikel(&mut data)
    }
    for v in data.z.values() {
        for d in &v.yx {
            for r in &d.row {
                if *r == Cell::Active {
                    count += 1;
                }
            }
        }
    }
    count
}

#[derive(Debug, Clone)]
struct D4 {
    w: HashMap<i32, D3>,
}

impl D4 {
    fn insert0(&mut self, w: i32) {
        let z0 = self.w.get(&0).unwrap().clone();
        let (min, max) = (*z0.z.keys().min().unwrap(), *z0.z.keys().max().unwrap());
        let r0 = VecDeque::from_iter(vec![Cell::Inactive; z0.rows]);
        let mut z = HashMap::new();
        for i in min..=max {
            z.insert(
                i,
                D2 {
                    yx: VecDeque::from_iter(vec![Row { row: r0.clone() }; z0.cols]),
                },
            );
        }
        self.w.insert(
            w,
            D3 {
                rows: z0.rows,
                cols: z0.rows,
                z,
            },
        );
    }
    fn extend(&mut self, min: i32, max: i32) {
        let z0 = self.w.get(&0).unwrap();
        let (minn, maxx) = (*z0.z.keys().min().unwrap(), *z0.z.keys().max().unwrap());
        // extend every
        for d3 in self.w.values_mut() {
            d3.extend(minn, maxx);
        }
        // extend z
        self.insert0(min - 1);
        self.insert0(max + 1);
    }
}

fn trans4(d3: D3) -> D4 {
    let mut w = HashMap::new();
    w.insert(0, d3);
    D4 { w }
}

impl std::fmt::Display for D4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (k, v) in &self.w {
            s = format!("{}|||| {} ||||\n{}\n", s, k, v);
        }
        write!(f, "{}", s)
    }
}

fn cikel4(data: &mut D4) {
    let (min, max) = (*data.w.keys().min().unwrap(), *data.w.keys().max().unwrap());
    // extend place
    data.extend(min, max);
    let mut res = data.clone();
    for (w, d3) in &data.w {
        for (z, d2) in &d3.z {
            //println!("||");
            //println!("{}", d2);
            for i in 0..d2.yx.len() {
                let d = &d2.yx[i];
                for j in 0..d.row.len() {
                    let cell = &d.row[j];
                    // check in this w
                    //  check in this z
                    let mut a = d2.check(i, j);

                    //  check in others z
                    if let Some(x) = d3.z.get(&(z - 1)) {
                        a += x.checkz(i, j);
                    }
                    if let Some(x) = d3.z.get(&(z + 1)) {
                        a += x.checkz(i, j);
                    }

                    if let Some(xw) = data.w.get(&(w + 1)) {
                        //  check in this z
                        a += xw.z[z].checkz(i, j);

                        //  check in others z
                        if let Some(x) = xw.z.get(&(z - 1)) {
                            a += x.checkz(i, j);
                        }
                        if let Some(x) = xw.z.get(&(z + 1)) {
                            a += x.checkz(i, j);
                        }
                    }

                    if let Some(xw) = data.w.get(&(w - 1)) {
                        //  check in this z
                        a += xw.z[z].checkz(i, j);

                        //  check in others z
                        if let Some(x) = xw.z.get(&(z - 1)) {
                            a += x.checkz(i, j);
                        }
                        if let Some(x) = xw.z.get(&(z + 1)) {
                            a += x.checkz(i, j);
                        }
                    }

                    /*
                    If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
                    If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
                    */
                    let c = &mut res.w.get_mut(w).unwrap().z.get_mut(z).unwrap().yx[i].row[j];
                    match cell {
                        Cell::Active => {
                            if !(a == 3 || a == 2) {
                                *c = Cell::Inactive;
                            }
                        }
                        Cell::Inactive => {
                            if a == 3 {
                                *c = Cell::Active;
                            }
                        }
                    }
                }
                //println!();
            }
        }
    }
    *data = res;
}

fn p2(data: &D4) -> usize {
    let mut data = data.clone();
    let mut count = 0;
    // 6 ciklov
    for _ in 1..=6 {
        cikel4(&mut data)
    }
    for w in data.w.values() {
        for v in w.z.values() {
            for d in &v.yx {
                for r in &d.row {
                    if *r == Cell::Active {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<Row> = read_data("./data/data17").unwrap();
    let data = trans(data);
    // part 1
    println!(
        "How many cubes are left in the active state after the sixth cycle? {}",
        p1(&data)
    );
    let data4 = trans4(data);
    // part 2
    println!(
        "How many cubes are left in the active state after the sixth cycle in 4-dimensional space? {}",
        p2(&data4)
    );
    Ok(())
}

#[test]
fn data_read17() {
    println!("{:?}", read_data::<Row>("./data/data17").unwrap());
}

#[test]
fn calc17() {
    let data = vec![".#.", "..#", "###"];
    let data = trans(
        data.iter()
            .map(|line| line.trim().parse().unwrap())
            .collect(),
    );
    let mut mdata = data.clone();
    // part 1
    println!("{}", mdata);
    cikel(&mut mdata);
    println!("{}", mdata);
    cikel(&mut mdata);
    println!("{}", mdata);
    assert_eq!(p1(&data), 112);
    // part 2
    let data4 = trans4(data);
    let mut mdata4 = data4.clone();
    // part 1
    println!("{}", mdata4);
    cikel4(&mut mdata4);
    println!("{}", mdata4);
    //cikel(&mut mdata4);
    //println!("{}", mdata);
    assert_eq!(p2(&data4), 848);
}
