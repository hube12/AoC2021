use num_traits::Num;

pub fn handle_solution<T: std::fmt::Display>(solution: anyhow::Result<T>) {
    match solution {
        Ok(solution) => println!("Solution is {}", solution),
        Err(error) => eprintln!("Error solution was not found with error :\n\t{}", error),
    }
}

#[derive(Copy, Clone)]
pub enum Part {
    Part1,
    Part2,
    Test,
}

pub fn valid_day(day: &str) -> anyhow::Result<usize> {
    if let Some(number) = day.strip_prefix("day") {
        if let Ok(number) = usize::from_str_radix(number, 10) {
            return match number {
                1..=25 => Ok(number),
                e => Err(anyhow::Error::msg(format!(
                    "Not a valid day number (1-25) : {}",
                    e
                ))),
            };
        }
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

#[test]
fn test() {
    let r = valid_day("day1");
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), 1);
}
