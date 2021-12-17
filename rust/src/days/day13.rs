use aoc_2021::{Day, UPos, Solution1, Solution2};
use std::convert::TryFrom;

use anyhow::anyhow;

#[derive(Default)]
pub struct Day13;

impl Day for Day13 {}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Fold {
    XAxis(usize),
    YAxis(usize),
}

impl TryFrom<&String> for Fold {
    type Error = anyhow::Error;
    fn try_from(s: &String) -> Result<Self, Self::Error> {
        if let Some(s) = s.strip_prefix("fold along ") {
            if let Some(s) = s.strip_prefix("x=") {
                return Ok(Fold::XAxis(usize::from_str_radix(s, 10)?));
            } else if let Some(s) = s.strip_prefix("y=") {
                return Ok(Fold::YAxis(usize::from_str_radix(s, 10)?));
            }
        }
        Err(anyhow::Error::msg("Not valid"))
    }
}

fn parse(lines: Vec<String>) -> anyhow::Result<(Vec<UPos>, Vec<Fold>)> {
    let mut iter = lines.iter();
    let points: Vec<UPos> = iter
        .by_ref()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            x.split_once(',')
                .ok_or(anyhow::Error::msg("Missing delimiter"))
        })
        .map(|x| {
            x.and_then(|(x, y)| {
                usize::from_str_radix(x, 10)
                    .and_then(|x| usize::from_str_radix(y, 10).and_then(|y| Ok(UPos::new(x, y))))
                    .map_err(|x| anyhow!(x))
            })
        })
        .collect::<Result<_, _>>()?;
    let folds: Vec<Fold> = iter.map(Fold::try_from).collect::<Result<_, _>>()?;
    Ok((points, folds))
}

fn min_max(points: &Vec<UPos>) -> (UPos, UPos) {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (usize::MAX, usize::MAX, 0usize, 0usize);
    for point in points {
        if point.x() < min_x {
            min_x = point.x();
        }
        if point.y() < min_y {
            min_y = point.y();
        }
        if point.x() > max_x {
            max_x = point.x();
        }
        if point.y() > max_y {
            max_y = point.y();
        }
    }
    (UPos::new(min_x, min_y), UPos::new(max_x, max_y))
}

impl Fold {
    fn fold(&self, points: &mut Vec<UPos>, (min, mut max): (UPos, UPos)) -> anyhow::Result<UPos> {
        match self {
            Fold::XAxis(fold) => {
                if *fold < (max.x() - min.x()) / 2 {
                    return Err(anyhow::Error::msg(
                        "This fold will bring more elements onto the smaller grid.",
                    ));
                }
                for point in points {
                    if point.x() > *fold {
                        point.set_x((*fold << 1) - point.x());
                    } else if point.x() == *fold {
                        return Err(anyhow::Error::msg("You can not fold onto itself"));
                    }
                }
                max.set_x(*fold - 1);
            }
            Fold::YAxis(fold) => {
                if *fold < (max.y() - min.y()) / 2 {
                    return Err(anyhow::Error::msg(
                        "This fold will bring more elements onto the smaller grid.",
                    ));
                }
                for point in points {
                    if point.y() > *fold {
                        point.set_y((*fold << 1) - point.y());
                    } else if point.y() == *fold {
                        return Err(anyhow::Error::msg("You can not fold onto itself"));
                    }
                }
                max.set_y(*fold - 1);
            }
        }
        Ok(max)
    }
}

impl Solution1 for Day13 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let (mut points, folds) = parse(lines)?;
        let (min, max) = min_max(&points);
        let first_fold = folds.first().ok_or(anyhow::Error::msg("Missing a fold"))?;
        let _ = first_fold.fold(&mut points, (min, max))?;
        points.sort();
        points.dedup();
        // Debug
        // for y in min.1..=max.1 {
        //     for x in min.0..=max.0 {
        //         if points.contains(&(x,y)){
        //             print!("#");
        //         }else{
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        // println!();

        Ok(points.len().to_string())
    }
}

impl Solution2 for Day13 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let (mut points, folds) = parse(lines)?;
        let (min, mut max) = min_max(&points);
        for fold in &folds {
            max = fold.fold(&mut points, (min, max))?;
            points.sort();
            points.dedup();
        }
        let mut s = String::with_capacity((max.y() - min.y()) * ((max.x() - min.x()) + 1));
        for y in min.y()..=max.y() {
            for x in min.x()..=max.x() {
                if points.contains(&UPos::new(x, y)) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        // println!("{}",s);
        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day13").unwrap();
        Ok(assert_eq!(
            Day13::default().run_solution1(lines)?,
            String::from("729")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day13").unwrap();
        Ok(assert_eq!(
            Day13::default().run_solution1(lines)?,
            String::from("17")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day13").unwrap();
        Ok(assert_eq!(
            Day13::default().run_solution2(lines)?,
            String::from(
                "\
                ###...##..####.#....###..#..#.####.###..\n\
                #..#.#..#....#.#....#..#.#..#.#....#..#.\n\
                #..#.#......#..#....###..####.###..#..#.\n\
                ###..#.##..#...#....#..#.#..#.#....###..\n\
                #.#..#..#.#....#....#..#.#..#.#....#....\n\
                #..#..###.####.####.###..#..#.#....#....\n\
                "
            )
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day13").unwrap();
        Ok(assert_eq!(
            Day13::default().run_solution2(lines)?,
            String::from(
                "\
            #####\n\
            #...#\n\
            #...#\n\
            #...#\n\
            #####\n\
            .....\n\
            .....\n\
            "
            )
        ))
    }
}
