use aoc_2021::{Day, Solution1, Solution2};

#[derive(Default)]
pub struct Day2;

impl Day for Day2 {}

use anyhow::anyhow;

impl Solution1 for Day2 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let res = lines
            .iter()
            .map(|x| {
                let (order, distance) = x
                    .split_once(" ")
                    .ok_or(anyhow::Error::msg("Not splittable"))?;
                let distance = isize::from_str_radix(distance, 10).map_err(|x| anyhow!(x))?;
                match order {
                    "forward" => Ok((distance, 0)),
                    "down" => Ok((0, distance)),
                    "up" => Ok((0, -distance)),
                    _ => Err(anyhow::Error::msg("Not a valid order")),
                }
            })
            .fold(Ok((0isize, 0isize)), |acc: anyhow::Result<_>, x| {
                let x = x?;
                acc.map(|(mut horizontal, mut depth)| {
                    horizontal += x.0;
                    depth += x.1;
                    (horizontal, depth)
                })
            })?;

        Ok((res.0 * res.1).to_string())
    }
}

impl Solution2 for Day2 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let res = lines
            .iter()
            .map(|x| {
                let (order, distance) = x
                    .split_once(" ")
                    .ok_or(anyhow::Error::msg("Not splittable"))?;
                let distance = isize::from_str_radix(distance, 10).map_err(|x| anyhow!(x))?;
                match order {
                    "forward" => Ok((distance, 0)),
                    "down" => Ok((0, distance)),
                    "up" => Ok((0, -distance)),
                    _ => Err(anyhow::Error::msg("Not a valid order")),
                }
            })
            .fold(Ok((0isize, 0isize, 0isize)), |acc: anyhow::Result<_>, x| {
                let x = x?;
                acc.map(|(mut horizontal, mut depth, mut aim)| {
                    horizontal += x.0;
                    aim += x.1;
                    depth += aim * x.0;
                    (horizontal, depth, aim)
                })
            })?;
        Ok((res.0 * res.1).to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day2").unwrap();
        Ok(assert_eq!(
            Day2::default().run_solution1(lines)?,
            String::from("1989014")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day2").unwrap();
        Ok(assert_eq!(
            Day2::default().run_solution1(lines)?,
            String::from("150")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day2").unwrap();
        Ok(assert_eq!(
            Day2::default().run_solution2(lines)?,
            String::from("2006917119")
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day2").unwrap();
        Ok(assert_eq!(
            Day2::default().run_solution2(lines)?,
            String::from("900")
        ))
    }
}
