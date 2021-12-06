use aoc_2021::{parse_uint, Day, Solution1, Solution2};

#[derive(Default)]
pub struct Day1;

impl Day for Day1 {}

impl Solution1 for Day1 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let transpose: Vec<usize> = parse_uint(lines)?;
        let mut iter = transpose.iter();
        let mut last = iter
            .next()
            .ok_or(anyhow::Error::msg("Missing at least an element"))?;
        let mut counter = 0;
        while let Some(next) = iter.next() {
            if last < next {
                counter += 1;
            }
            last = next
        }
        Ok(counter.to_string())
    }
}

impl Solution2 for Day1 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let transpose: Vec<usize> = lines
            .iter()
            .map(|x| usize::from_str_radix(x.as_ref(), 10))
            .collect::<Result<_, _>>()?;
        let mut iter = transpose.iter();
        let (mut slide1, mut slide2, mut slide3) = (0usize, 0usize, 0usize);
        let next = iter
            .next()
            .ok_or(anyhow::Error::msg("Missing at least 1 element"))?;
        slide1 += *next;
        let next = iter
            .next()
            .ok_or(anyhow::Error::msg("Missing at least 2 elements"))?;
        slide1 += *next;
        slide2 += *next;
        let next = iter
            .next()
            .ok_or(anyhow::Error::msg("Missing at least 3 elements"))?;
        slide1 += *next;
        slide2 += *next;
        slide3 += *next;
        let mut last = slide1;
        let mut counter = 0;
        while let Some(next) = iter.next() {
            slide1 = slide2 + *next;
            slide2 = slide3 + *next;
            slide3 = *next;
            if last < slide1 {
                counter += 1;
            }
            // update rolling buffer
            last = slide1;
        }
        Ok(counter.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() {
        let lines: Vec<String> = collect_file(Part1, "Day1").unwrap();
        let _ = dbg!(Day1::default().run_solution1(lines));
    }

    #[test]
    fn test_solution1() {
        let lines: Vec<String> = collect_file(Test, "Day1").unwrap();
        let _ = dbg!(Day1::default().run_solution1(lines));
    }

    #[test]
    fn solution2() {
        let lines: Vec<String> = collect_file(Part2, "Day1").unwrap();
        let _ = dbg!(Day1::default().run_solution2(lines));
    }

    #[test]
    fn test_solution2() {
        let lines: Vec<String> = collect_file(Test, "Day1").unwrap();
        let _ = dbg!(Day1::default().run_solution2(lines));
    }
}
