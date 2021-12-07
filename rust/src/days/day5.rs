use aoc_2021::{Day, Solution1, Solution2};

#[derive(Default)]
pub struct Day5;

impl Day for Day5 {}

use anyhow::anyhow;

impl Solution1 for Day5 {

    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {

        Ok(0.to_string())
    }
}

impl Solution2 for Day5 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        Ok(0.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() {
        let lines: Vec<String> = collect_file(Part1, "Day5").unwrap();
        let _ = dbg!(Day5::default().run_solution1(lines));
    }

    #[test]
    fn test_solution1() {
        let lines: Vec<String> = collect_file(Test, "Day5").unwrap();
        let _ = dbg!(Day5::default().run_solution1(lines));
    }

    #[test]
    fn solution2() {
        let lines: Vec<String> = collect_file(Part2, "Day5").unwrap();
        let _ = dbg!(Day5::default().run_solution2(lines));
    }

    #[test]
    fn test_solution2() {
        let lines: Vec<String> = collect_file(Test, "Day5").unwrap();
        let _ = dbg!(Day5::default().run_solution2(lines));
    }
}
