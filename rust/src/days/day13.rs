use aoc_2021::{Day, Solution1, Solution2};
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

fn parse(lines: Vec<String>) -> anyhow::Result<(Vec<(usize, usize)>, Vec<Fold>)> {
    let mut iter = lines.iter();
    let points: Vec<(usize, usize)> = iter
        .by_ref()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            x.split_once(',')
                .ok_or(anyhow::Error::msg("Missing delimiter"))
        })
        .map(|x| {
            x.and_then(|(x, y)| {
                usize::from_str_radix(x, 10)
                    .and_then(|x| usize::from_str_radix(y, 10).and_then(|y| Ok((x, y))))
                    .map_err(|x| anyhow!(x))
            })
        })
        .collect::<Result<_, _>>()?;
    let folds: Vec<Fold> = iter.map(Fold::try_from).collect::<Result<_, _>>()?;
    Ok((points, folds))
}

fn min_max(points: &Vec<(usize, usize)>) -> ((usize, usize), (usize, usize)) {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (usize::MAX, usize::MAX, 0usize, 0usize);
    for (x, y) in points {
        if *x < min_x {
            min_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
    }
    ((min_x, min_y), (max_x, max_y))
}

impl Fold {
    fn fold(&self, points: &mut Vec<(usize, usize)>, (min, mut max): ((usize, usize), (usize, usize))) -> anyhow::Result<(usize, usize)> {
        match self {
            Fold::XAxis(fold) => {
                if *fold < (max.0 - min.0) / 2 {
                    return Err(anyhow::Error::msg(
                        "This fold will bring more elements onto the smaller grid.",
                    ));
                }
                for (x, _) in points {
                    if *x > *fold {
                        *x = 2 * (*fold) - *x;
                    } else if *x == *fold {
                        return Err(anyhow::Error::msg(
                            "You can not fold onto itself",
                        ));
                    }
                }
                max.0 = *fold - 1;
            }
            Fold::YAxis(fold) => {
                if *fold < (max.1 - min.1) / 2 {
                    return Err(anyhow::Error::msg(
                        "This fold will bring more elements onto the smaller grid.",
                    ));
                }
                for (_, y) in points {
                    if *y > *fold {
                        *y = 2 * (*fold) - *y;
                    } else if *y == *fold {
                        return Err(anyhow::Error::msg(
                            "You can not fold onto itself",
                        ));
                    }
                }
                max.1 = *fold - 1;
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
        for fold in &folds{
            max = fold.fold(&mut points, (min, max))?;
            points.sort();
            points.dedup();
        }
        let mut s =String::with_capacity((max.1-min.1)*((max.0-min.0)+1));
        for y in min.1..=max.1 {
            for x in min.0..=max.0 {
                if points.contains(&(x,y)){
                    s.push('#');
                }else{
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
    fn solution1() {
        let lines: Vec<String> = collect_file(Part1, "Day13").unwrap();
        let _ = dbg!(Day13::default().run_solution1(lines));
    }

    #[test]
    fn test_solution1() {
        let lines: Vec<String> = collect_file(Test, "Day13").unwrap();
        let _ = dbg!(Day13::default().run_solution1(lines));
    }

    #[test]
    fn solution2() {
        let lines: Vec<String> = collect_file(Part2, "Day13").unwrap();
        let _ = dbg!(Day13::default().run_solution2(lines));
    }

    #[test]
    fn test_solution2() {
        let lines: Vec<String> = collect_file(Test, "Day13").unwrap();
        let _ = dbg!(Day13::default().run_solution2(lines));
    }
}
