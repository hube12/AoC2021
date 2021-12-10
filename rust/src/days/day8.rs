use aoc_2021::{Day, Solution1, Solution2};
use std::ops::Sub;

#[derive(Default)]
pub struct Day8;

impl Day for Day8 {}



impl Solution1 for Day8 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let crabs: Vec<Vec<f64>> = lines
            .first()
            .ok_or(anyhow::Error::msg("Missing a line"))?
            .split(',')
            .map(|x| usize::from_str_radix(x, 10).map(|x| vec![x as f64]))
            .collect::<Result<_, _>>()?;
        let r = geometric_median(&crabs, 0.001);
        let first=r[0].round(); // we don't consider geometric median that are not integer
        let dist=calculate_dist(&crabs, &[first as f64]);
        Ok(dist.to_string())
    }
}

impl Solution2 for Day8 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let crabs: Vec<usize> = lines
            .first()
            .ok_or(anyhow::Error::msg("Missing a line"))?
            .split(',')
            .map(|x| usize::from_str_radix(x, 10))
            .collect::<Result<_, _>>()?;
        let min=*crabs.iter().min().ok_or(anyhow::Error::msg("Missing an element"))?;
        let max=*crabs.iter().max().ok_or(anyhow::Error::msg("Missing an element"))?;
        let mut min_dist=usize::MAX;
        let mut _pos_min=0usize;
        for pos in min..max {
            let dist=dist_arithmetic_suite(&crabs,pos);
            if dist<min_dist{
                min_dist=dist;
                _pos_min=pos;
            }
        }
        Ok(min_dist.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() {
        let lines: Vec<String> = collect_file(Part1, "Day8").unwrap();
        let _ = dbg!(Day8::default().run_solution1(lines));
    }

    #[test]
    fn test_solution1() {
        let lines: Vec<String> = collect_file(Test, "Day8").unwrap();
        let _ = dbg!(Day8::default().run_solution1(lines));
    }

    #[test]
    fn solution2() {
        let lines: Vec<String> = collect_file(Part2, "Day8").unwrap();
        let _ = dbg!(Day8::default().run_solution2(lines));
    }

    #[test]
    fn test_solution2() {
        let lines: Vec<String> = collect_file(Test, "Day8").unwrap();
        let _ = dbg!(Day8::default().run_solution2(lines));
    }
}
