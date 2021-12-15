use aoc_2021::{Day, Solution1, Solution2};

#[derive(Default)]
pub struct Day3;

impl Day for Day3 {}

use std::cmp::min;

fn get_list_frequencies(lines: &Vec<String>, max_char: usize) -> anyhow::Result<Vec<usize>> {
    let len = min(
        lines
            .first()
            .ok_or(anyhow::Error::msg("Missing a line"))?
            .len(),
        max_char,
    );
    let mut v = vec![0usize; len];
    for line in lines {
        for (idx, c) in line.chars().enumerate() {
            if idx > max_char {
                break;
            }
            match c {
                '0' => {
                    v[idx] += 1;
                }
                '1' => {}
                _ => {
                    return Err(anyhow::Error::msg("Not 0 or 1"));
                }
            }
        }
    }
    Ok(v)
}

impl Solution1 for Day3 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let v = get_list_frequencies(&lines, usize::MAX)?;
        let mut gamma: usize = 0;
        let mut epsilon: usize = 0;
        let mid = lines.len() / 2;
        let mut exponent = 1usize;
        for zero_count in v.iter().rev() {
            if *zero_count > mid {
                epsilon += exponent
            } else {
                gamma += exponent
            }
            exponent <<= 1;
        }
        Ok((gamma * epsilon).to_string())
    }
}

fn split_vec(lines: Vec<String>, nth: usize) -> anyhow::Result<(Vec<String>, Vec<String>)> {
    let mut v0 = Vec::with_capacity(lines.len());
    let mut v1 = Vec::with_capacity(lines.len());
    for line in lines {
        match line.chars().nth(nth) {
            Some('0') => {
                v0.push(line);
            }
            Some('1') => {
                v1.push(line);
            }
            _ => {
                return Err(anyhow::Error::msg("Not a valid pattern"));
            }
        }
    }
    return Ok((v0, v1));
}

impl Solution2 for Day3 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let len = lines
            .first()
            .ok_or(anyhow::Error::msg("Missing a line"))?
            .len();
        let (v0, v1) = split_vec(lines, 0)?;
        let mut co2;
        let mut o2;
        if v0.len() > v1.len() {
            co2 = v0;
            o2 = v1;
        } else {
            co2 = v1;
            o2 = v0;
        }
        let mut n = 1;
        loop {
            if co2.len() == 1 || n > len {
                break;
            }
            let (co2_0, co2_1) = split_vec(co2, n)?;
            if co2_0.len() > co2_1.len() {
                co2 = co2_0
            } else {
                co2 = co2_1
            }
            n += 1;
        }
        let mut n = 1;
        loop {
            if o2.len() == 1 || n > len {
                break;
            }
            let (o2_0, o2_1) = split_vec(o2, n)?;
            if o2_0.len() > o2_1.len() {
                o2 = o2_1
            } else {
                o2 = o2_0
            }
            n += 1;
        }

        Ok(
            (usize::from_str_radix(o2.first().ok_or(anyhow::Error::msg("missing o2"))?, 2)?
                * usize::from_str_radix(co2.first().ok_or(anyhow::Error::msg("missing o2"))?, 2)?)
            .to_string(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day3").unwrap();
        Ok(assert_eq!(
            Day3::default().run_solution1(lines)?,
            String::from("841526")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day3").unwrap();
        Ok(assert_eq!(
            Day3::default().run_solution1(lines)?,
            String::from("198")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day3").unwrap();
        Ok(assert_eq!(
            Day3::default().run_solution2(lines)?,
            String::from("4790390")
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day3").unwrap();
        Ok(assert_eq!(
            Day3::default().run_solution2(lines)?,
            String::from("230")
        ))
    }
}
