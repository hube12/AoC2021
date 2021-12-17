use aoc_2021::{Day, UPos, Solution1, Solution2};

#[derive(Default)]
pub struct Day4;

impl Day for Day4 {}

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Board {
    /// represents value in (col,row)
    map: HashMap<usize, Vec<UPos>>,
    /// Marked number in each row
    rows: Vec<usize>,
    /// Marked number in each row
    cols: Vec<usize>,
    marked: HashSet<usize>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            map: HashMap::with_capacity(25),
            rows: vec![5; 5],
            cols: vec![5; 5],
            // technically you can not know the lower bound but we assume that a 5 should happen at this point
            marked: HashSet::with_capacity(20),
        }
    }
}

impl Board {
    fn insert_line(&mut self, line: &String, row: usize) -> anyhow::Result<()> {
        for (col, number) in line.split_whitespace().enumerate() {
            let number = usize::from_str_radix(number, 10)?;
            match self.map.entry(number) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(UPos::new(col, row));
                }
                Entry::Vacant(e) => {
                    e.insert(vec![UPos::new(col, row)]);
                }
            }
        }
        Ok(())
    }

    fn mark_number(&mut self, number: usize) {
        if self.marked.contains(&number) {
            return;
        }
        if let Some(positions) = self.map.get(&number) {
            for pos in positions {
                self.rows[pos.x()] -= 1;
                self.cols[pos.y()] -= 1;
            }
            self.marked.insert(number);
        }
    }

    fn has_finished(&self) -> bool {
        self.rows.iter().any(|x| *x == 0) || self.cols.iter().any(|x| *x == 0)
    }

    fn score(&self) -> usize {
        self.map.keys().filter(|&x| !self.marked.contains(x)).sum()
    }
}

impl Solution1 for Day4 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let mut iter = lines.iter();
        let numbers: Vec<usize> = iter
            .next()
            .ok_or(anyhow::Error::msg("Missing a line"))?
            .trim()
            .split(",")
            .map(|x| usize::from_str_radix(x, 10))
            .collect::<Result<_, _>>()?;
        let mut boards = vec![];
        loop {
            if let Some(_) = iter.next() {
                let mut board = Board::default();
                for row in 0..5 {
                    let line = iter
                        .next()
                        .ok_or(anyhow::Error::msg("Missing a board line"))?;
                    board.insert_line(line, row)?;
                }
                boards.push(board);
            } else {
                break;
            }
        }
        for number in numbers {
            for board in &mut boards {
                board.mark_number(number);
                if board.has_finished() {
                    return Ok((board.score() * number).to_string());
                }
            }
        }
        Err(anyhow::Error::msg("No board did finish"))
    }
}

impl Solution2 for Day4 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let mut iter = lines.iter();
        let numbers: Vec<usize> = iter
            .next()
            .ok_or(anyhow::Error::msg("Missing a line"))?
            .trim()
            .split(",")
            .map(|x| usize::from_str_radix(x, 10))
            .collect::<Result<_, _>>()?;
        let mut boards = vec![];
        loop {
            if let Some(_) = iter.next() {
                let mut board = Board::default();
                for row in 0..5 {
                    let line = iter
                        .next()
                        .ok_or(anyhow::Error::msg("Missing a board line"))?;
                    board.insert_line(line, row)?;
                }
                boards.push(board);
            } else {
                break;
            }
        }
        for number in numbers {
            let len = boards.len();
            let mut to_keep = Vec::with_capacity(len);
            for mut board in boards {
                board.mark_number(number);
                if !board.has_finished() {
                    to_keep.push(board)
                } else {
                    if len == 1 {
                        return Ok((board.score() * number).to_string());
                    }
                }
            }
            boards = to_keep;
        }
        Err(anyhow::Error::msg("No board did finish"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day4").unwrap();
        Ok(assert_eq!(
            Day4::default().run_solution1(lines)?,
            String::from("16716")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day4").unwrap();
        Ok(assert_eq!(
            Day4::default().run_solution1(lines)?,
            String::from("4512")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day4").unwrap();
        Ok(assert_eq!(
            Day4::default().run_solution2(lines)?,
            String::from("4880")
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day4").unwrap();
        Ok(assert_eq!(
            Day4::default().run_solution2(lines)?,
            String::from("1924")
        ))
    }
}
