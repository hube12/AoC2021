use num_traits::Num;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::hash::Hash;
use anyhow::anyhow;
pub fn handle_solution<T: std::fmt::Display>(solution: anyhow::Result<T>) {
    match solution {
        Ok(solution) => println!("Solution is \n{}", solution),
        Err(error) => eprintln!("Error solution was not found with error :\n\t{}", error),
    }
}

#[derive(Copy, Clone)]
pub enum Part {
    Part1,
    Part2,
    Test,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Pos<T:Num+Copy> {
    x: T,
    y: T,
}

impl<T:Num+Copy> TryFrom<(&str, &str)> for Pos<T> {
    type Error = anyhow::Error;

    fn try_from((x, y): (&str, &str)) -> Result<Self, Self::Error> {
        Ok(Pos::new(
            T::from_str_radix(x, 10).map_err(|_| anyhow::Error::msg("Not a valid number for bound T"))?,
            T::from_str_radix(y, 10).map_err(|_| anyhow::Error::msg("Not a valid number for bound T"))?,
        ))
    }
}
impl<T:Num+Copy> Pos<T> {
    pub fn x(&self) -> T {
        self.x
    }
    pub fn y(&self) -> T {
        self.y
    }

    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }
}
pub type UPos=Pos<usize>;
pub type IPos=Pos<isize>;

pub type Pattern = u8;

pub const FULL: Pattern = 0;
pub const STAR: Pattern = 1;
pub const CROSS: Pattern = 2;

pub fn get_adjacent_positions<const PATTERN: Pattern>(
    x: usize,
    y: usize,
    height: usize,
    length: usize,
) -> anyhow::Result<Vec<Pos<usize>>> {
    let (x, y) = (x as isize, y as isize);
    let (length, height) = (length as isize, height as isize);
    let bounded = |x1, y1, v: &mut Vec<UPos>| {
        if !(x1 >= length || y1 >= height || y1 < 0 || x1 < 0) {
            v.push(Pos::new(x1 as usize, y1 as usize))
        }
    };
    match PATTERN {
        FULL => {
            let mut v = Vec::with_capacity(8);
            for (x1, y1) in [
                (x - 1, y),
                (x + 1, y),
                (x, y - 1),
                (x, y + 1),
                (x - 1, y - 1),
                (x + 1, y - 1),
                (x - 1, y + 1),
                (x + 1, y + 1),
            ] {
                bounded(x1, y1, &mut v)
            }
            Ok(v)
        }
        STAR => {
            let mut v = Vec::with_capacity(8);
            for (x1, y1) in [
                (x - 1, y - 1),
                (x + 1, y - 1),
                (x - 1, y + 1),
                (x + 1, y + 1),
            ] {
                bounded(x1, y1, &mut v)
            }
            Ok(v)
        }
        CROSS => {
            let mut v = Vec::with_capacity(8);
            for (x1, y1) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                bounded(x1, y1, &mut v)
            }
            Ok(v)
        }
        _ => Err(anyhow::Error::msg("Not a valid pattern")),
    }
}

pub fn valid_day(day: &str) -> anyhow::Result<usize> {
    let number = day.strip_prefix("day").unwrap_or(day);
    if let Ok(number) = usize::from_str_radix(number, 10) {
        return match number {
            1..=25 => Ok(number),
            e => Err(anyhow::Error::msg(format!(
                "Not a valid day number (1-25) : {}",
                e
            ))),
        };
    }
    Err(anyhow::Error::msg(format!(
        "Not a valid day format dayX (X:1-25) : {}",
        day
    )))
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Part1 => write!(f, "input.txt"),
            Part::Part2 => write!(f, "input.txt"),
            Part::Test => write!(f, "test.txt"),
        }
    }
}

pub fn parse_uint<Input: AsRef<str>, T: Num>(lines: Vec<Input>) -> anyhow::Result<Vec<T>>
where
    <T as Num>::FromStrRadixErr: std::error::Error + Send + Sync,
    <T as Num>::FromStrRadixErr: 'static,
{
    Ok(lines
        .iter()
        .map(|x| T::from_str_radix(x.as_ref(), 10))
        .collect::<Result<_, _>>()?)
}

pub fn collect_file<T: Ord + Eq + Clone + std::str::FromStr>(
    part: Part,
    day: &str,
) -> anyhow::Result<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::error::Error + Sync + Send,
{
    let day = day.trim().to_lowercase();
    valid_day(&*day)?;
    let path = format!("input/{}/{}", day, part);
    let input = std::fs::read_to_string(path)?;
    let lines = input.lines();
    let mut res: Vec<T> = vec![];
    for line in lines {
        res.push(
            line.parse::<T>().map_err(|err| {
                anyhow::Error::msg(format!("Could not parse with error {:?}", err))
            })?,
        )
    }
    Ok(res)
}

pub trait Solution1 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String>;
}

pub trait Solution2 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String>;
}

pub trait Day: Solution1 + Solution2 {}
