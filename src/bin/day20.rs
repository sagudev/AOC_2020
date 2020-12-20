use aoc::read_data_space_saperator2;
use std::collections::HashMap;
use std::convert::Infallible;
use std::error::Error;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
enum D {
    /// .
    D,
    /// #
    H,
}

impl std::fmt::Display for D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            D::D => write!(f, "."),
            D::H => write!(f, "#"),
        }
    }
}

impl D {
    fn from_char(c: char) -> Self {
        match c {
            '.' => D::D,
            '#' => D::H,
            _ => panic!("wc"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Row {
    row: Vec<D>,
}

impl Row {
    fn rotate(&mut self) {
        self.row.reverse();
    }
    fn rot(&self) -> Self {
        let mut c = self.clone();
        c.row.reverse();
        c
    }
}

impl FromStr for Row {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            row: s.trim().chars().map(D::from_char).collect(),
        })
    }
}

impl std::iter::FromIterator<D> for Row {
    fn from_iter<I: IntoIterator<Item = D>>(iter: I) -> Self {
        let mut row = Vec::new();

        for i in iter {
            row.push(i);
        }

        Self { row }
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for r in &self.row {
            s = format!("{}{}", s, r);
        }
        writeln!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
struct Tile {
    /// id
    id: usize,
    /// all data
    data: Vec<Row>,
    // edges
    /// long edge 1
    le1: Row,
    /// long edge 1
    le2: Row,
    /// short edge 1
    se1: Row,
    /// short edge 1
    se2: Row,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for r in &self.data {
            s = format!("{}{}", s, r);
        }
        writeln!(f, "Tile {}:\n{}", self.id, s)
    }
}

impl Tile {
    fn flip(&self) -> Self {
        let mut res = self.clone();
        res.data = Vec::new();
        for i in 0..self.data[0].row.len() {
            let mut row = Vec::new();
            for d in &self.data {
                row.push(d.row[i].clone());
            }
            res.data.push(Row { row });
        }
        res.se1 = res.data.get(0).unwrap().clone();
        res.se2 = res.data.last().unwrap().clone();
        res.le1 = res.data.iter().map(|x| x.row[0].clone()).collect();
        res.le2 = res
            .data
            .iter()
            .map(|x| x.row.last().unwrap().clone())
            .collect();
        res
    }
}

fn trans(dataa: Vec<String>) -> Vec<Tile> {
    let mut res = Vec::new();
    for d in dataa {
        let mut id = 0;
        let mut data: Vec<Row> = Vec::new();
        for s in d.split('|') {
            if s == "" {
                continue;
            } else if s.contains("Tile") {
                id = s[5..9].parse().unwrap();
            } else {
                data.push(s.parse().unwrap());
            }
        }
        res.push(Tile {
            id,
            data: data.clone(),
            se1: data.get(0).unwrap().clone(),
            se2: data.last().unwrap().clone(),
            le1: data.iter().map(|x| x.row[0].clone()).collect(),
            le2: data.iter().map(|x| x.row.last().unwrap().clone()).collect(),
        });
    }
    res
}

#[derive(Debug)]
/// sosedi
struct Ne {
    le1: Option<usize>,
    /// long edge 1
    le2: Option<usize>,
    /// short edge 1
    se1: Option<usize>,
    /// short edge 1
    se2: Option<usize>,
}

impl Ne {
    fn len_some(&self) -> usize {
        let mut c = 0;
        if self.le1.is_some() {
            c += 1;
        }
        if self.le2.is_some() {
            c += 1;
        }
        if self.se1.is_some() {
            c += 1;
        }
        if self.se2.is_some() {
            c += 1;
        }
        c
    }
}

fn check(ne: &mut Ne, d: &Tile, e: &Tile) {
    if e.le1 == d.le1 || e.le2 == d.le1 {
        ne.le1 = Some(e.id);
    } else if e.le1 == d.le2 || e.le2 == d.le2 {
        ne.le2 = Some(e.id);
    } else if e.se1 == d.se1 || e.se2 == d.se1 {
        ne.se1 = Some(e.id);
    } else if e.se1 == d.se2 || e.se2 == d.se2 {
        ne.se2 = Some(e.id);
    } else if e.le1.rot() == d.le1 || e.le2.rot() == d.le1 {
        ne.le1 = Some(e.id);
    } else if e.le1.rot() == d.le2 || e.le2.rot() == d.le2 {
        ne.le2 = Some(e.id);
    } else if e.se1.rot() == d.se1 || e.se2.rot() == d.se1 {
        ne.se1 = Some(e.id);
    } else if e.se1.rot() == d.se2 || e.se2.rot() == d.se2 {
        ne.se2 = Some(e.id);
    }
}

fn p1(data: &[Tile]) -> usize {
    let mut h: HashMap<usize, Ne> = HashMap::new();
    for d in data {
        let mut ne = Ne {
            le1: None,
            le2: None,
            se1: None,
            se2: None,
        };
        for e in data {
            if d.id == e.id {
                continue;
            };
            check(&mut ne, d, e);
            let e = e.flip();
            check(&mut ne, d, &e);
        }
        h.insert(d.id, ne);
    }
    let mut m = 1;
    for (k, v) in h {
        if v.len_some() == 2 {
            m *= k
        }
    }
    m
}

fn p2(data: &[Tile]) -> i32 {
    5
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Advent Of Code 2020!");
    let data: Vec<String> = read_data_space_saperator2::<String>("./data/data20", '|').unwrap();
    let data = trans(data);
    // part 1
    println!(
        "What do you get if you multiply together the IDs of the four corner tiles? {}",
        p1(&data)
    );
    // part 2
    println!("Fuel needed (part 2): {}", p2(&data));
    Ok(())
}

#[test]
fn data_read20() {
    println!(
        "{:?}",
        read_data_space_saperator2::<String>("./data/data20", '|').unwrap()
    );
}

#[test]
fn calc20() {
    let data: Vec<String> = vec![
        "|Tile 2311:|..##.#..#.|##..#.....|#...##..#.|####.#...#|##.##.###.|##...#.###|.#.#.#..##|..#....#..|###...#.#.|..###..###".to_string(),
        "|Tile 1951:|#.##...##.|#.####...#|.....#..##|#...######|.##.#....#|.###.#####|###.##.##.|.###....#.|..#.#..#.#|#...##.#..".to_string(),
        "|Tile 1171:|####...##.|#..##.#..#|##.#..#.#.|.###.####.|..###.####|.##....##.|.#...####.|#.##.####.|####..#...|.....##...".to_string(),
        "|Tile 1427:|###.##.#..|.#..#.##..|.#.##.#..#|#.#.#.##.#|....#...##|...##..##.|...#.#####|.#.####.#.|..#..###.#|..##.#..#.".to_string(),
        "|Tile 1489:|##.#.#....|..##...#..|.##..##...|..#...#...|#####...#.|#..#.#.#.#|...#.#.#..|##.#...##.|..##.##.##|###.##.#..".to_string(),
        "|Tile 2473:|#....####.|#..#.##...|#.##..#...|######.#.#|.#...#.#.#|.#########|.###.#..#.|########.#|##...##.#.|..###.#.#.".to_string(),
        "|Tile 2971:|..#.#....#|#...###...|#.#.###...|##.##..#..|.#####..##|.#..####.#|#..#.#..#.|..####.###|..#.#.###.|...#.#.#.#".to_string(),
        "|Tile 2729:|...#.#.#.#|####.#....|..#.#.....|....#..#.#|.##..##.#.|.#.####...|####.#.#..|##.####...|##..#.##..|#.##...##.".to_string(),
        "|Tile 3079:|#.#.#####.|.#..######|..#.......|######....|####.#..#.|.#...#.##.|#.#####.##|..#.###...|..#.......|..#.###...".to_string()
    ];
    let data = trans(data);
    println!("{:?}", data);
    // part 1
    assert_eq!(p1(&data), 20899048083289);
}
