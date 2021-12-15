use aoc_2021::{Day, Pos, Solution1, Solution2};
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
pub struct Day14;

impl Day for Day14 {}

fn parse(lines: Vec<String>) -> anyhow::Result<(String, HashMap<(char, char), char>)> {
    let mut iter = lines.iter();
    let polymer = iter
        .next()
        .ok_or(anyhow::Error::msg("Missing polymer"))?
        .clone();
    iter.next();
    let pairs = iter
        .map(|x| {
            x.split_once(" -> ")
                .ok_or(anyhow::Error::msg("Missing delimiter"))
        })
        .map(|x| {
            x.and_then(|(a, b)| match b.chars().count() {
                1 => b
                    .chars()
                    .nth(0)
                    .ok_or(anyhow::Error::msg("Not a valid char"))
                    .and_then(|b| Ok((a, b))),
                _ => Err(anyhow::Error::msg("Not valid length for mapping")),
            })
        })
        .map(|x| {
            x.and_then(|(a, b)| match a.chars().count() {
                2 => a
                    .chars()
                    .nth(0)
                    .ok_or(anyhow::Error::msg("Not a valid char"))
                    .and_then(|fa| {
                        a.chars()
                            .nth(1)
                            .ok_or(anyhow::Error::msg("Not a valid char"))
                            .and_then(|sa| Ok(((fa, sa), b)))
                    }),
                _ => Err(anyhow::Error::msg("Not valid length for mapping")),
            })
        })
        .collect::<Result<_, _>>()?;
    Ok((polymer, pairs))
}

fn dumb_process_polymer(
    polymer: Vec<char>,
    pairs: &HashMap<(char, char), char>,
) -> anyhow::Result<Vec<char>> {
    let mut new = Vec::with_capacity(polymer.len() * 2);
    let mut iter = polymer.iter();
    let mut current_char = *iter
        .next()
        .ok_or(anyhow::Error::msg("Missing at least a char in polymer"))?;
    new.push(current_char);
    while let Some(&next_char) = iter.next() {
        let current = (current_char, next_char);
        if let Some(&x) = pairs.get(&current) {
            new.push(x);
        }
        new.push(next_char);
        current_char = next_char;
    }
    Ok(new)
}

fn min_max(mut v: Vec<char>) -> anyhow::Result<Pos> {
    v.sort();
    let mut maxi = 0;
    let mut mini = usize::MAX;
    let mut ctr = 1;
    let mut current = v
        .pop()
        .ok_or(anyhow::Error::msg("Missing at least a char in polymer"))?;
    loop {
        if let Some(c) = v.pop() {
            if c == current {
                ctr += 1;
            } else {
                maxi = std::cmp::max(ctr, maxi);
                mini = std::cmp::min(ctr, mini);
                ctr = 1;
                current = c;
            }
        } else {
            maxi = std::cmp::max(ctr, maxi);
            mini = std::cmp::min(ctr, mini);
            break;
        }
    }
    Ok(Pos::new(mini, maxi))
}

impl Solution1 for Day14 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let (polymer, pairs) = parse(lines)?;
        let mut chars = polymer.chars().collect();
        for _ in 0..10 {
            chars = dumb_process_polymer(chars, &pairs)?;
        }
        let pos = min_max(chars)?;
        Ok((pos.y() - pos.x()).to_string())
    }
}

// acc is better for allocation but recursive looks nice
fn process_pair(
    pair: (char, char),
    pairs: &HashMap<(char, char), char>,
    it: usize,
) -> VecDeque<char> {
    if it == 0 {
        return VecDeque::from([pair.1]);
    }
    if let Some(&next) = pairs.get(&pair) {
        let mut a = process_pair((pair.0, next), pairs, it - 1);
        a.push_back(pair.1);
        return a;
    }
    return VecDeque::from([pair.1]);
}

#[allow(dead_code)]
fn rec_process(
    nth: char,
    mut queue: VecDeque<char>,
    pairs: &HashMap<(char, char), char>,
    stats: &mut HashMap<char, usize>,
) -> anyhow::Result<()> {
    *stats.entry(nth).or_insert(0) += 1;
    // get the last stable derivation
    let stable_pair = queue
        .pop_front()
        .ok_or(anyhow::Error::msg("Missing front in queue is impossible"))?;
    *stats.entry(stable_pair).or_insert(0) += 1;
    // go back N-1
    if let Some(mut last_pair) = queue.pop_front() {
        let mut go_back = 1;
        while let Some(front) = queue.pop_front() {
            // get each sub tree
            let q = process_pair((last_pair, front), pairs, go_back);
            // process back each subtree
            rec_process(last_pair, q, pairs, stats)?;
            go_back += 1;
            last_pair = front;
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn smart_process_polymer(
    polymer: Vec<char>,
    it: usize,
    pairs: &HashMap<(char, char), char>,
) -> anyhow::Result<HashMap<char, usize>> {
    let mut iter = polymer.iter();
    let mut nth = *iter.next().ok_or(anyhow::Error::msg("Need 1 characters"))?;
    let mut stats = HashMap::new();
    while let Some(&nth1) = iter.next() {
        // for each sequence in the initial step
        let queue = process_pair((nth, nth1), pairs, it);
        rec_process(nth, queue, pairs, &mut stats)?;
        nth = nth1;
    }
    *stats.entry(nth).or_insert(0) += 1;
    Ok(stats)
}

impl Solution2 for Day14 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let (polymer, pairs) = parse(lines)?;
        //dbg!(smart_process_polymer(polymer.chars().collect(),10,&pairs));
        // We assume the polymer contains all letter at the beginning for the capacity (which might not be the case)
        let mut char_stats = HashMap::with_capacity(polymer.len());
        // we assume full derivation for the capacity (but it could be higher)
        let mut step_stats = HashMap::with_capacity(pairs.len());
        // Initialize the step stats with the first pattern available in chunks of 2 in the original polymer
        // ABCDEBC
        // __ __
        //  __ __
        //   __ __
        // here BC is twice for instance
        let mut it = polymer.chars();
        let mut last_char = it
            .next()
            .ok_or(anyhow::Error::msg("Need at least a char"))?;
        *char_stats.entry(last_char).or_insert(0) += 1;
        while let Some(c) = it.next() {
            *step_stats.entry((last_char, c)).or_insert(0usize) += 1usize;
            *char_stats.entry(c).or_insert(0usize) += 1usize;
            last_char = c;
        }
        for _ in 0..40 {
            // we should not derive more than the previous on average (but we can)
            let mut new_step_stats = HashMap::with_capacity(step_stats.len());
            for ((start, end), occurrence) in step_stats {
                if let Some(&middle) = pairs.get(&(start, end)) {
                    *new_step_stats.entry((start, middle)).or_insert(0usize) += occurrence;
                    *new_step_stats.entry((middle, end)).or_insert(0usize) += occurrence;
                    *char_stats.entry(middle).or_insert(0usize) += occurrence;
                } else {
                    *new_step_stats.entry((start, end)).or_insert(0usize) += occurrence;
                }
            }
            step_stats = new_step_stats;
        }
        if char_stats.len() == 0 {
            return Err(anyhow::Error::msg("Missing at least a char in the polymer"));
        }
        Ok((char_stats.values().max().unwrap() - char_stats.values().min().unwrap()).to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day14").unwrap();
        Ok(assert_eq!(
            Day14::default().run_solution1(lines)?,
            String::from("2010")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day14").unwrap();
        Ok(assert_eq!(
            Day14::default().run_solution1(lines)?,
            String::from("1588")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day14").unwrap();
        Ok(assert_eq!(
            Day14::default().run_solution2(lines)?,
            String::from("2437698971143")
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day14").unwrap();
        Ok(assert_eq!(
            Day14::default().run_solution2(lines)?,
            String::from("2188189693529")
        ))
    }
}
