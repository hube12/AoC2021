use aoc_2021::{Day, Solution1, Solution2};

#[derive(Default)]
pub struct Day5;

impl Day for Day5 {}

use num_traits::abs;
use std::borrow::BorrowMut;
use std::cmp::{max, min};
use std::collections::HashMap;

fn compute_on_map(
    lines: Vec<String>,
    compute: impl for<'a> Fn(
        ((usize, usize), (usize, usize)),
        Box<dyn FnMut(usize, usize) + 'a>,
    ) -> anyhow::Result<()>,
) -> anyhow::Result<String> {
    let mut map: HashMap<(usize, usize), usize> = HashMap::with_capacity(lines.len());
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
                let a = usize::from_str_radix(start_x, 10).and_then(|min_x| {
                    usize::from_str_radix(start_y, 10).and_then(|min_y| Ok((min_x, min_y)))
                });
                let b = usize::from_str_radix(end_x, 10).and_then(|min_x| {
                    usize::from_str_radix(end_y, 10).and_then(|min_y| Ok((min_x, min_y)))
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

fn update_map(map: &mut HashMap<(usize, usize), usize>) -> impl FnMut(usize, usize) + '_ {
    move |x: usize, y: usize| {
        map.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
    }
}

impl Solution1 for Day5 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        compute_on_map(lines, |((min_x, min_y), (max_x, max_y)), mut map| {
            if min_x == max_x || min_y == max_y {
                for x in min(min_x, max_x)..=max(min_x, max_x) {
                    for y in min(min_y, max_y)..=max(min_y, max_y) {
                        map(x, y);
                    }
                }
            }
            Ok(())
        })
    }
}

impl Solution2 for Day5 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        compute_on_map(lines, |((min_x, min_y), (max_x, max_y)), mut map| {
            if min_x == max_x || min_y == max_y {
                //dbg!(((min_x, min_y), (max_x, max_y)));
                for x in min(min_x, max_x)..=max(min_x, max_x) {
                    for y in min(min_y, max_y)..=max(min_y, max_y) {
                        map(x, y)
                    }
                }
            } else if abs(min_x as isize - max_x as isize) == abs(min_y as isize - max_y as isize) {
                let mut x_range: Box<dyn DoubleEndedIterator<Item = usize>> =
                    Box::new(min_x..=max_x);
                if min_x > max_x {
                    x_range = Box::new((max_x..=min_x).rev());
                }
                let mut y_range: Box<dyn DoubleEndedIterator<Item = usize>> =
                    Box::new(min_y..=max_y);
                if min_y > max_y {
                    y_range = Box::new((max_y..=min_y).rev());
                }
                while let (Some(x), Some(y)) = (x_range.next(), y_range.next()) {
                    map(x, y)
                }
                if x_range.next().is_some() || y_range.next().is_some() {
                    return Err(anyhow::Error::msg(format!(
                        "Diagonal was not correct for {}:{} {}:{}",
                        min_x, max_x, min_y, max_y
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
