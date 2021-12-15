use aoc_2021::{Day, Solution1, Solution2};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default)]
pub struct Day12;

impl Day for Day12 {}

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
enum Caves {
    Big(String),
    Small(String),
    Start,
    End,
}

impl ToString for Caves {
    fn to_string(&self) -> String {
        match self {
            Caves::Big(x) => x.clone(),
            Caves::Small(x) => x.clone(),
            Caves::Start => String::from("start"),
            Caves::End => String::from("end"),
        }
    }
}

impl Caves {
    fn parse(s: &str) -> Caves {
        match s {
            "start" => Caves::Start,
            "end" => Caves::End,
            x => {
                if x.chars().all(|c| c.is_uppercase()) {
                    Caves::Big(x.to_string())
                } else {
                    Caves::Small(x.to_string())
                }
            }
        }
    }
}

impl Solution1 for Day12 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let lines: Vec<(Caves, Caves)> = lines
            .iter()
            .map(|x| {
                x.split_once("-")
                    .ok_or(anyhow::Error::msg("Missing delimiter"))
            })
            .map(|x| x.and_then(|(s, e)| Ok((Caves::parse(s), Caves::parse(e)))))
            .collect::<Result<_, _>>()?;
        let mut links = HashMap::with_capacity(lines.len());
        for (link_a, link_b) in lines {
            links
                .entry(link_a.clone())
                .or_insert(vec![])
                .push(link_b.clone());
            links
                .entry(link_b.clone())
                .or_insert(vec![])
                .push(link_a.clone());
        }
        // current path for each traversal (we assume 100 long path are about as much as it can be)
        let stack = HashMap::with_capacity(100);
        let sum = graph_traversal::<1, false>(&Caves::Start, &links, stack)?;
        Ok(sum.to_string())
    }
}

fn graph_traversal<const LIMIT: u8, const CHECK: bool>(
    current: &Caves,
    links: &HashMap<Caves, Vec<Caves>>,
    mut tracked_path: HashMap<Caves, u8>,
) -> anyhow::Result<usize> {
    let next_ones = links
        .get(current)
        .ok_or(anyhow::Error::msg("Missing current"))?;
    if tracked_path.get(current) == Some(&LIMIT) {
        return Ok(0);
    }
    if CHECK && tracked_path.values().filter(|&&x| x == LIMIT).count() > 2 {
        return Ok(0);
    }
    match current {
        Caves::Small(_) => {
            *tracked_path.entry(current.clone()).or_insert(0) += 1;
        }
        Caves::Start => {
            tracked_path.entry(current.clone()).or_insert(LIMIT);
        }
        Caves::End => {
            return Ok(1);
        }
        _ => {}
    }
    let mut sum = 0;
    for next in next_ones {
        let current = graph_traversal::<LIMIT, CHECK>(next, links, tracked_path.clone())?;
        sum += current;
    }
    return Ok(sum);
}

impl Solution2 for Day12 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let lines: Vec<(Caves, Caves)> = lines
            .iter()
            .map(|x| {
                x.split_once("-")
                    .ok_or(anyhow::Error::msg("Missing delimiter"))
            })
            .map(|x| x.and_then(|(s, e)| Ok((Caves::parse(s), Caves::parse(e)))))
            .collect::<Result<_, _>>()?;
        let mut links = HashMap::with_capacity(lines.len());
        for (link_a, link_b) in lines {
            links
                .entry(link_a.clone())
                .or_insert(vec![])
                .push(link_b.clone());
            links
                .entry(link_b.clone())
                .or_insert(vec![])
                .push(link_a.clone());
        }
        // current path for each traversal (we assume 100 long path are about as much as it can be)
        let stack = HashMap::with_capacity(100);
        let sum = graph_traversal::<2, true>(&Caves::Start, &links, stack)?;
        Ok(sum.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day12").unwrap();
        Ok(assert_eq!(
            Day12::default().run_solution1(lines)?,
            String::from("3708")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day12").unwrap();
        Ok(assert_eq!(
            Day12::default().run_solution1(lines)?,
            String::from("226")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day12").unwrap();
        Ok(assert_eq!(
            Day12::default().run_solution2(lines)?,
            String::from("93858")
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day12").unwrap();
        Ok(assert_eq!(
            Day12::default().run_solution2(lines)?,
            String::from("3509")
        ))
    }
}
