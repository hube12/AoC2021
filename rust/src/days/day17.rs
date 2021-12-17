use aoc_2021::{Day, UPos, Solution1, Solution2, IPos};
use std::cmp::max;
use std::convert::TryFrom;
use std::collections::HashSet;

#[derive(Default)]
pub struct Day17;

impl Day for Day17 {}

fn dumb_step(pos: &mut IPos, velocity: &mut IPos) {
    pos.set_x(pos.x() + velocity.x());
    pos.set_y(pos.y() + velocity.y());
    velocity.set_x(if velocity.x() < 0 {
        velocity.x() + 1
    } else {
        max(velocity.x() - 1, 0)
    });
    velocity.set_y(velocity.y() - 1);
}

fn parse(line: &String) -> anyhow::Result<(IPos, IPos)> {
    line.trim_start_matches("target area: x=")
        .split_once(", y=")
        .ok_or(anyhow::Error::msg("Missing delimiter: , y="))
        .and_then(|(x, y)| {
            x.split_once("..")
                .ok_or(anyhow::Error::msg("Missing delimiter .."))
                .and_then(|(x1, x2)| {
                    y.split_once("..")
                        .ok_or(anyhow::Error::msg("Missing delimiter .."))
                        .and_then(|(y1, y2)| {
                            IPos::try_from((x1, y1))
                                .and_then(|pos1| IPos::try_from((x2, y2)).map(|pos2| (pos1, pos2)))
                        })
                })
        })
}

fn inside(area: (&IPos, &IPos), pos: &IPos) -> bool {
    // x is normal, y is inverted for humans
    area.0.x() <= pos.x() && pos.x() <= area.1.x() && area.0.y() <= pos.y() && pos.y() <= area.1.y()
}

// out of the bounds of the 0,0 to area limits
fn wrong_side(area: (&IPos, &IPos), pos: &IPos) -> bool {
    pos.x() > area.1.x() || pos.y() < area.0.y() || pos.x() < 0
}

impl Solution1 for Day17 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let area = parse(lines.first().ok_or(anyhow::Error::msg("Missing line"))?)?;
        let p_area = (&area.0, &area.1);
        let mut possible = HashSet::new();
        let mut mv = 0;
        for vx in 0..1000 {
            for vy in -100..1000 {
                let mut velocity = IPos::new(vx, vy);
                let mut pos = IPos::new(0, 0);
                let mut max_pos = 0;
                let mut ctr = 10;
                for n in 0..1000 {
                    dumb_step(&mut pos, &mut velocity);
                    max_pos = max(max_pos, pos.y());
                    if inside(p_area, &pos) {
                        possible.insert(IPos::new(vx, vy));
                        if max_pos >= mv {
                            mv = max_pos;
                        }
                        break;
                    }
                    if wrong_side(p_area, &pos) {
                        break;
                    }
                    if ctr == 0 {
                        break;
                    }
                    if pos.x() == 0 || velocity.x() == 0 && (area.0.x() - pos.x()) > 0 {
                        ctr -= 1;
                    }
                }
            }
        }
        Ok(mv.to_string())
    }
}

impl Solution2 for Day17 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let area = parse(lines.first().ok_or(anyhow::Error::msg("Missing line"))?)?;
        let p_area = (&area.0, &area.1);
        let mut possible = HashSet::new();
        let mut mv = 0;
        for vx in 0..1000 {
            for vy in -500..1000 {
                let mut velocity = IPos::new(vx, vy);
                let mut pos = IPos::new(0, 0);
                let mut max_pos = 0;
                let mut ctr = 1000;
                for n in 0..10000 {
                    dumb_step(&mut pos, &mut velocity);
                    max_pos = max(max_pos, pos.y());
                    if inside(p_area, &pos) {
                        possible.insert(IPos::new(vx, vy));
                        if max_pos >= mv {
                            mv = max_pos;
                        }
                        break;
                    }
                    if wrong_side(p_area, &pos) {
                        break;
                    }
                    if ctr == 0 {
                        break;
                    }
                    if pos.x() == 0 || velocity.x() == 0 && (area.0.x() - pos.x()) > 0 {
                        ctr -= 1;
                    }
                }
            }
        }
        Ok(possible.len().to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day17").unwrap();
        Ok(assert_eq!(
            Day17::default().run_solution1(lines)?,
            String::from("5565")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day17").unwrap();
        Ok(assert_eq!(
            Day17::default().run_solution1(lines)?,
            String::from("45")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day17").unwrap();
        Ok(assert_eq!(
            Day17::default().run_solution2(lines)?,
            String::from("2118")
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day17").unwrap();
        Ok(assert_eq!(
            Day17::default().run_solution2(lines)?,
            String::from("112")
        ))
    }
}
