use aoc_2021::{Day, Solution1, Solution2};
use std::collections::HashSet;

#[derive(Default)]
pub struct Day9;

impl Day for Day9 {}


impl Solution1 for Day9 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let length = lines.first().ok_or(anyhow::Error::msg("Missing a line"))?.len();
        let height = lines.len();
        let mut matrix = Vec::with_capacity(height);
        for line in lines {
            let mut l = Vec::with_capacity(length);
            for digit in line.chars() {
                l.push(digit.to_digit(10).ok_or(anyhow::Error::msg("Not a number"))?)
            }
            matrix.push(l)
        }
        let mut ctr = 0;
        for y in 0..height {
            for x in 0..length {
                let current = matrix[y][x];
                let adjacent_positions = get_adjacent_positions(x, y, height, length);
                if adjacent_positions.iter().all(|&(ax, ay)| matrix[ay][ax] > current) {
                    ctr += current + 1;
                }
            }
        }
        Ok(ctr.to_string())
    }
}

fn get_adjacent_positions(x: usize, y: usize, height: usize, length: usize) -> Vec<(usize, usize)> {
    let (x, y) = (x as isize, y as isize);
    let (length, height) = (length as isize, height as isize);
    let mut v = Vec::with_capacity(4);
    for (x1, y1) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        if !(x1 >= length || y1 >= height || y1 < 0 || x1 < 0) {
            v.push((x1 as usize, y1 as usize))
        }
    }
    v
}

impl Solution2 for Day9 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let length = lines.first().ok_or(anyhow::Error::msg("Missing a line"))?.len();
        let height = lines.len();
        let mut matrix = Vec::with_capacity(height);
        for line in lines {
            let mut l = Vec::with_capacity(length);
            for digit in line.chars() {
                l.push(digit.to_digit(10).ok_or(anyhow::Error::msg("Not a number"))?)
            }
            matrix.push(l)
        }
        let mut bassins_length = vec![];
        for y in 0..height {
            for x in 0..length {
                let current = matrix[y][x];
                let adjacent_positions = get_adjacent_positions(x, y, height, length);
                if adjacent_positions.iter().all(|&(ax, ay)| matrix[ay][ax] > current) {
                    let mut set = HashSet::with_capacity(100);
                    recursive_flow(x, y, height, length, &mut matrix, &mut set);
                    bassins_length.push(set.len());
                }
            }
        }
        bassins_length.sort();
        Ok(bassins_length.iter().rev().take(3).fold(1usize,|x,&y| x*y).to_string())
    }
}

fn recursive_flow(x: usize, y: usize, height: usize, length: usize, matrix: &Vec<Vec<u32>>, acc: &mut HashSet<(usize, usize)>) {
    let value = matrix[y][x];
    if value==9 || acc.contains(&(x, y)){
        return;
    } else {
        acc.insert((x, y));
    }
    for (ax, ay) in get_adjacent_positions(x, y, height, length) {
        recursive_flow(ax, ay, height, length, matrix, acc);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() {
        let lines: Vec<String> = collect_file(Part1, "Day9").unwrap();
        let _ = dbg!(Day9::default().run_solution1(lines));
    }

    #[test]
    fn test_solution1() {
        let lines: Vec<String> = collect_file(Test, "Day9").unwrap();
        let _ = dbg!(Day9::default().run_solution1(lines));
    }

    #[test]
    fn solution2() {
        let lines: Vec<String> = collect_file(Part2, "Day9").unwrap();
        let _ = dbg!(Day9::default().run_solution2(lines));
    }

    #[test]
    fn test_solution2() {
        let lines: Vec<String> = collect_file(Test, "Day9").unwrap();
        let _ = dbg!(Day9::default().run_solution2(lines));
    }
}
