use aoc_2021::{Day, Solution1, Solution2};
use std::collections::HashSet;

#[derive(Default)]
pub struct Day11;

impl Day for Day11 {}


impl Solution1 for Day11 {
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
        let mut points = 0usize;
        for _ in 0..100 {
            points += step(&mut matrix, height, length);
        }
        Ok(points.to_string())
    }
}

fn step(matrix: &mut Vec<Vec<u32>>, height: usize, length: usize) -> usize {
    // update the grid
    let mut should_flash = Vec::new();
    for (y, row) in matrix.iter_mut().enumerate() {
        for (x, squid) in row.iter_mut().enumerate() {
            *squid += 1;
            if *squid> 9 {
                should_flash.push((x, y));
            }
        }
    }
    let mut has_flashed = HashSet::with_capacity(100);
    for (x, y) in should_flash {
        recursive_flash(x, y, height, length, matrix, &mut has_flashed);
    }
    for (x, y) in &has_flashed {
        if matrix[*y][*x] < 9 {
            panic!("Shouldn't happen")
        }
        matrix[*y][*x] = 0;
    }
    has_flashed.len()
}

fn recursive_flash(x: usize, y: usize, height: usize, length: usize, matrix: &mut Vec<Vec<u32>>, has_flashed: &mut HashSet<(usize, usize)>) {

    let value = matrix[y][x];
    if value <= 9 || has_flashed.contains(&(x, y)) {
        return;
    }else{
        has_flashed.insert((x,y));
    }

    for (ax, ay) in get_adjacent_positions(x, y, height, length) {
        matrix[ay][ax] += 1;
        recursive_flash(ax, ay, height, length, matrix, has_flashed);
    }
}

fn get_adjacent_positions(x: usize, y: usize, height: usize, length: usize) -> Vec<(usize, usize)> {
    let (x, y) = (x as isize, y as isize);
    let (length, height) = (length as isize, height as isize);
    let mut v = Vec::with_capacity(4);
    for (x1, y1) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1), (x - 1, y - 1), (x + 1, y - 1), (x - 1, y + 1), (x + 1, y + 1)] {
        if !(x1 >= length || y1 >= height || y1 < 0 || x1 < 0) {
            v.push((x1 as usize, y1 as usize))
        }
    }
    v
}

impl Solution2 for Day11 {
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
        let mut step_i =1;
        loop{
            let flashes=step(&mut matrix, height, length);
            if flashes==height*length{
               break;
            }
            step_i+=1;
        }
        Ok(step_i.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() {
        let lines: Vec<String> = collect_file(Part1, "Day11").unwrap();
        let _ = dbg!(Day11::default().run_solution1(lines));
    }

    #[test]
    fn test_solution1() {
        let lines: Vec<String> = collect_file(Test, "Day11").unwrap();
        let _ = dbg!(Day11::default().run_solution1(lines));
    }

    #[test]
    fn solution2() {
        let lines: Vec<String> = collect_file(Part2, "Day11").unwrap();
        let _ = dbg!(Day11::default().run_solution2(lines));
    }

    #[test]
    fn test_solution2() {
        let lines: Vec<String> = collect_file(Test, "Day11").unwrap();
        let _ = dbg!(Day11::default().run_solution2(lines));
    }
}
