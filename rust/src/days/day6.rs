use aoc_2021::{Day, Solution1, Solution2};

#[derive(Default)]
pub struct Day6;

impl Day for Day6 {}

impl Solution1 for Day6 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let mut fishes: Vec<usize> = lines
            .first()
            .ok_or(anyhow::Error::msg("Missing a line"))?
            .split(',')
            .map(|x| usize::from_str_radix(x, 10))
            .collect::<Result<_, _>>()?;
        let mut new_fishes = Vec::with_capacity(100);
        for _ in 0..80 {
            new_fishes.clear();
            fishes.iter_mut().for_each(|x| {
                if *x == 0 {
                    new_fishes.push(8);
                    *x = 6;
                } else {
                    *x -= 1;
                }
            });
            fishes.extend(&new_fishes);
        }
        Ok(fishes.len().to_string())
    }
}

impl Solution2 for Day6 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let fishes: Vec<usize> = lines
            .first()
            .ok_or(anyhow::Error::msg("Missing a line"))?
            .split(',')
            .map(|x| usize::from_str_radix(x, 10))
            .collect::<Result<_, _>>()?;
        const NUMBER_DAYS: usize = 9;
        const OFFSET_DAYS: usize = 7;
        let mut days = [0u128; NUMBER_DAYS];
        for fish in fishes {
            days[fish] += 1;
        }
        for day in 0..256 {
            let today = day % NUMBER_DAYS;
            days[(today + OFFSET_DAYS) % NUMBER_DAYS] += days[today]
        }
        Ok(days.iter().sum::<u128>().to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day6").unwrap();
        Ok(assert_eq!(
            Day6::default().run_solution1(lines)?,
            String::from("393019")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day6").unwrap();
        Ok(assert_eq!(
            Day6::default().run_solution1(lines)?,
            String::from("5934")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day6").unwrap();
        Ok(assert_eq!(
            Day6::default().run_solution2(lines)?,
            String::from("1757714216975")
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day6").unwrap();
        Ok(assert_eq!(
            Day6::default().run_solution2(lines)?,
            String::from("26984457539")
        ))
    }
}
