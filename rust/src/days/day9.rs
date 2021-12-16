use aoc_2021::{get_adjacent_positions, Day, Pos, Solution1, Solution2, CROSS};
use std::collections::HashSet;

#[derive(Default)]
pub struct Day9;

impl Day for Day9 {}

impl Solution1 for Day9 {
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
        let mut ctr = 0;
        for y in 0..height {
            for x in 0..length {
                let current = matrix[y][x];
                let adjacent_positions = get_adjacent_positions::<CROSS>(x, y, height, length)?;
                if adjacent_positions
                    .iter()
                    .all(|pos| matrix[pos.y()][pos.x()] > current)
                {
                    ctr += current + 1;
                }
            }
        }
        Ok(ctr.to_string())
    }
}

impl Solution2 for Day9 {
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
        let mut bassins_length = vec![];
        for y in 0..height {
            for x in 0..length {
                let current = matrix[y][x];
                let adjacent_positions = get_adjacent_positions::<CROSS>(x, y, height, length)?;
                if adjacent_positions
                    .iter()
                    .all(|pos| matrix[pos.y()][pos.x()] > current)
                {
                    let mut set = HashSet::with_capacity(100);
                    recursive_flow(Pos::new(x, y), height, length, &mut matrix, &mut set)?;
                    bassins_length.push(set.len());
                }
            }
        }
        bassins_length.sort();
        Ok(bassins_length
            .iter()
            .rev()
            .take(3)
            .fold(1usize, |x, &y| x * y)
            .to_string())
    }
}

fn recursive_flow(
    pos: Pos,
    height: usize,
    length: usize,
    matrix: &Vec<Vec<u32>>,
    acc: &mut HashSet<Pos>,
) -> anyhow::Result<()> {
    let value = matrix[pos.y()][pos.x()];
    if value == 9 || acc.contains(&pos) {
        return Ok(());
    } else {
        acc.insert(pos);
    }
    for adj_pos in get_adjacent_positions::<CROSS>(pos.x(), pos.y(), height, length)? {
        recursive_flow(adj_pos, height, length, matrix, acc)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day9").unwrap();
        Ok(assert_eq!(
            Day9::default().run_solution1(lines)?,
            String::from("478")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day9").unwrap();
        Ok(assert_eq!(
            Day9::default().run_solution1(lines)?,
            String::from("15")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day9").unwrap();
        Ok(assert_eq!(
            Day9::default().run_solution2(lines)?,
            String::from("1327014")
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day9").unwrap();
        Ok(assert_eq!(
            Day9::default().run_solution2(lines)?,
            String::from("1134")
        ))
    }
}
