use aoc_2021::{Day, Pos, Solution1, Solution2};

#[derive(Default)]
pub struct Day5;

impl Day for Day5 {}

use num_traits::abs;
use std::borrow::BorrowMut;
use std::cmp::{max, min};
use std::collections::HashMap;

fn compute_on_map(
    lines: Vec<String>,
    compute: impl for<'a> Fn((Pos, Pos), Box<dyn FnMut(Pos) + 'a>) -> anyhow::Result<()>,
) -> anyhow::Result<String> {
    let mut map: HashMap<Pos, usize> = HashMap::with_capacity(lines.len());
    lines
        .iter()
        .map(|x| {
            x.split_once(" -> ")
                .ok_or(anyhow::Error::msg("Missing delimiter ->"))
        })
        .map(|x| {
            x.and_then(|(a, b)| {
                let a = a
                    .split_once(",")
                    .ok_or(anyhow::Error::msg("Missing delimiter ,"));
                let b = b
                    .split_once(",")
                    .ok_or(anyhow::Error::msg("Missing delimiter ,"));
                a.and_then(|x| b.and_then(|y| Ok((x, y))))
            })
        })
        .map(|r| {
            r.and_then(|((start_x, start_y), (end_x, end_y))| {
                let a = usize::from_str_radix(start_x, 10).and_then(|start_x| {
                    usize::from_str_radix(start_y, 10)
                        .and_then(|start_y| Ok(Pos::new(start_x, start_y)))
                });
                let b = usize::from_str_radix(end_x, 10).and_then(|end_x| {
                    usize::from_str_radix(end_y, 10).and_then(|end_y| Ok(Pos::new(end_x, end_y)))
                });
                Ok(a.and_then(|x| b.and_then(|y| Ok((x, y)))))
            })
        })
        .try_for_each::<_, Result<_, anyhow::Error>>(|r| {
            compute(r??, Box::new(update_map(map.borrow_mut())))
        })?;
    // Debug
    // for x in 0..10 {
    //     for y in 0..10 {
    //         print!("{}", map.get(&(y, x)).map(|x| x.to_string()).unwrap_or(String::from('.')));
    //     }
    //     println!()
    // }
    Ok(map.values().filter(|&x| *x > 1).count().to_string())
}

fn update_map(map: &mut HashMap<Pos, usize>) -> impl FnMut(Pos) + '_ {
    move |pos| {
        map.entry(pos).and_modify(|v| *v += 1).or_insert(1);
    }
}

impl Solution1 for Day5 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        compute_on_map(lines, |(min_pos, max_pos), mut map| {
            if min_pos.x() == max_pos.x() || min_pos.y() == max_pos.y() {
                for x in min(min_pos.x(), max_pos.x())..=max(min_pos.x(), max_pos.x()) {
                    for y in min(min_pos.y(), max_pos.y())..=max(min_pos.y(), max_pos.y()) {
                        map(Pos::new(x, y));
                    }
                }
            }
            Ok(())
        })
    }
}

impl Solution2 for Day5 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        compute_on_map(lines, |(min_pos, max_pos), mut map| {
            if min_pos.x() == max_pos.x() || min_pos.y() == max_pos.y() {
                //dbg!(((min_pos.x(), min_pos.y()), (max_pos.x(), max_pos.y())));
                for x in min(min_pos.x(), max_pos.x())..=max(min_pos.x(), max_pos.x()) {
                    for y in min(min_pos.y(), max_pos.y())..=max(min_pos.y(), max_pos.y()) {
                        map(Pos::new(x, y))
                    }
                }
            } else if abs(min_pos.x() as isize - max_pos.x() as isize)
                == abs(min_pos.y() as isize - max_pos.y() as isize)
            {
                let mut x_range: Box<dyn DoubleEndedIterator<Item = usize>> =
                    Box::new(min_pos.x()..=max_pos.x());
                if min_pos.x() > max_pos.x() {
                    x_range = Box::new((max_pos.x()..=min_pos.x()).rev());
                }
                let mut y_range: Box<dyn DoubleEndedIterator<Item = usize>> =
                    Box::new(min_pos.y()..=max_pos.y());
                if min_pos.y() > max_pos.y() {
                    y_range = Box::new((max_pos.y()..=min_pos.y()).rev());
                }
                while let (Some(x), Some(y)) = (x_range.next(), y_range.next()) {
                    map(Pos::new(x, y))
                }
                if x_range.next().is_some() || y_range.next().is_some() {
                    return Err(anyhow::Error::msg(format!(
                        "Diagonal was not correct for {}:{} {}:{}",
                        min_pos.x(),
                        max_pos.x(),
                        min_pos.y(),
                        max_pos.y()
                    )));
                }
            }
            Ok(())
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day5").unwrap();
        Ok(assert_eq!(
            Day5::default().run_solution1(lines)?,
            String::from("5442")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day5").unwrap();
        Ok(assert_eq!(
            Day5::default().run_solution1(lines)?,
            String::from("5")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day5").unwrap();
        Ok(assert_eq!(
            Day5::default().run_solution2(lines)?,
            String::from("19571")
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day5").unwrap();
        Ok(assert_eq!(
            Day5::default().run_solution2(lines)?,
            String::from("12")
        ))
    }
}
