use aoc_2021::{get_adjacent_positions, Day, UPos, Solution1, Solution2, FULL};
use std::collections::HashSet;

#[derive(Default)]
pub struct Day11;

impl Day for Day11 {}

impl Solution1 for Day11 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let length = lines
            .first()
            .ok_or(anyhow::Error::msg("Missing a line"))?
            .len();
        let height = lines.len();
        let mut matrix = Vec::with_capacity(height);
        for line in lines {
            let mut l = Vec::with_capacity(length);
            for digit in line.chars() {
                l.push(
                    digit
                        .to_digit(10)
                        .ok_or(anyhow::Error::msg("Not a number"))?,
                )
            }
            matrix.push(l)
        }
        let mut points = 0usize;
        for _ in 0..100 {
            points += step(&mut matrix, height, length)?;
        }
        Ok(points.to_string())
    }
}

fn step(matrix: &mut Vec<Vec<u32>>, height: usize, length: usize) -> anyhow::Result<usize> {
    // update the grid
    let mut should_flash = Vec::new();
    for (y, row) in matrix.iter_mut().enumerate() {
        for (x, squid) in row.iter_mut().enumerate() {
            *squid += 1;
            if *squid > 9 {
                should_flash.push(UPos::new(x, y));
            }
        }
    }
    let mut has_flashed = HashSet::with_capacity(100);
    for pos in should_flash {
        recursive_flash(pos, height, length, matrix, &mut has_flashed)?;
    }
    for pos in &has_flashed {
        if matrix[pos.y()][pos.x()] < 9 {
            panic!("Shouldn't happen")
        }
        matrix[pos.y()][pos.x()] = 0;
    }
    Ok(has_flashed.len())
}

fn recursive_flash(
    pos: UPos,
    height: usize,
    length: usize,
    matrix: &mut Vec<Vec<u32>>,
    has_flashed: &mut HashSet<UPos>,
) -> anyhow::Result<()> {
    let value = matrix[pos.y()][pos.x()];
    if value <= 9 || has_flashed.contains(&pos) {
        return Ok(());
    } else {
        has_flashed.insert(pos);
    }
    for pos in get_adjacent_positions::<FULL>(pos.x(), pos.y(), height, length)? {
        matrix[pos.y()][pos.x()] += 1;
        recursive_flash(pos, height, length, matrix, has_flashed)?;
    }
    Ok(())
}

impl Solution2 for Day11 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let length = lines
            .first()
            .ok_or(anyhow::Error::msg("Missing a line"))?
            .len();
        let height = lines.len();
        let mut matrix = Vec::with_capacity(height);
        for line in lines {
            let mut l = Vec::with_capacity(length);
            for digit in line.chars() {
                l.push(
                    digit
                        .to_digit(10)
                        .ok_or(anyhow::Error::msg("Not a number"))?,
                )
            }
            matrix.push(l)
        }
        let mut step_i = 1;
        loop {
            let flashes = step(&mut matrix, height, length)?;
            if flashes == height * length {
                break;
            }
            step_i += 1;
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
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day11").unwrap();
        Ok(assert_eq!(
            Day11::default().run_solution1(lines)?,
            String::from("1681")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day11").unwrap();
        Ok(assert_eq!(
            Day11::default().run_solution1(lines)?,
            String::from("1656")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day11").unwrap();
        Ok(assert_eq!(
            Day11::default().run_solution2(lines)?,
            String::from("276")
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day11").unwrap();
        Ok(assert_eq!(
            Day11::default().run_solution2(lines)?,
            String::from("195")
        ))
    }
}
