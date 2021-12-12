use aoc_2021::{Day, Solution1, Solution2};
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Default)]
pub struct Day8;

impl Day for Day8 {}

fn find_easy_segments(segments: &Vec<&str>) -> anyhow::Result<HashMap<usize, String>> {
    let mut map = HashMap::with_capacity(10);

    let one = segments
        .iter()
        .find(|x| x.len() == 2)
        .ok_or(anyhow::Error::msg("Missing 7"))?;
    insert_map(&mut map, 1, *one);
    let seven = segments
        .iter()
        .find(|x| x.len() == 3)
        .ok_or(anyhow::Error::msg("Missing 7"))?;
    insert_map(&mut map, 7, *seven);
    let four = segments
        .iter()
        .find(|x| x.len() == 4)
        .ok_or(anyhow::Error::msg("Missing 7"))?;
    insert_map(&mut map, 4, *four);
    let eight = segments
        .iter()
        .find(|x| x.len() == 7)
        .ok_or(anyhow::Error::msg("Missing 7"))?;
    insert_map(&mut map, 8, *eight);

    Ok(map)
}

//      we use the following representation
//              bbbbbbb
//             a       c
//             a       c
//             a       c
//              ddddddd
//             e       g
//             e       g
//             e       g
//              fffffff
// (1).len()=2
// (4).len()=4
// (7).len()=3
// (8).len()=8
// (0,6,9).len()=6
// (2,3,5).len()=5

// (0,9,6).contains(any(4))=9
// !(0,9,6).contains(all(1))=6
// zero is the last one
fn find_0_6_9(segments: &Vec<&str>, map: &mut HashMap<usize, String>) -> anyhow::Result<()> {
    let zero_nine_six: Vec<&&str> = segments.iter().filter(|&x| x.len() == 6).collect();
    if zero_nine_six.len() != 3 {
        return Err(anyhow::Error::msg("Not correct segments"));
    }
    let four = map.get(&4).ok_or(anyhow::Error::msg("Missing 4"))?;
    let nine = zero_nine_six
        .iter()
        .find(|&unknown| four.chars().all(|x| unknown.contains(x)))
        .ok_or(anyhow::Error::msg("Missing 9"))?;
    insert_map(map, 9, nine);
    let one = map.get(&1).ok_or(anyhow::Error::msg("Missing 1"))?;
    let six = zero_nine_six
        .iter()
        .find(|&unknown| !one.chars().all(|x| unknown.contains(x)))
        .ok_or(anyhow::Error::msg("Missing 6"))?;
    insert_map(map, 6, six);
    let zero = zero_nine_six
        .iter()
        .filter(|&x| x != six && x != nine)
        .next()
        .ok_or(anyhow::Error::msg("Missing 0"))?;
    insert_map(map, 0, zero);
    if six == nine {
        return Err(anyhow::Error::msg("6==9"));
    }
    if zero == nine {
        return Err(anyhow::Error::msg("0==9"));
    }
    if zero == six {
        return Err(anyhow::Error::msg("0==6"));
    }
    Ok(())
}

// (2,3,5).contains(all(1))=3
// !(0,9,6).contains(all(1))=6
// zero is the last one
fn find_2_3_5(segments: &Vec<&str>, map: &mut HashMap<usize, String>) -> anyhow::Result<()> {
    let two_three_five: Vec<&&str> = segments.iter().filter(|&x| x.len() == 5).collect();
    if two_three_five.len() != 3 {
        return Err(anyhow::Error::msg("Not correct segments"));
    }
    let one = map.get(&1).ok_or(anyhow::Error::msg("Missing 1"))?;
    let three = two_three_five
        .iter()
        .find(|&unknown| one.chars().all(|x| unknown.contains(x)))
        .ok_or(anyhow::Error::msg("Missing 3"))?;
    insert_map(map, 3, three);
    let six = map.get(&6).ok_or(anyhow::Error::msg("Missing 6"))?;
    let five = two_three_five
        .iter()
        .filter(|&x| x != three)
        .find(|&x| x.chars().filter(|&x| six.contains(x)).count() == 5)
        .ok_or(anyhow::Error::msg("Missing 5"))?;
    insert_map(map, 5, five);
    let two = two_three_five
        .iter()
        .filter(|&x| x != three && x != five)
        .next()
        .ok_or(anyhow::Error::msg("Missing 2"))?;
    insert_map(map, 2, two);
    if two == five {
        return Err(anyhow::Error::msg("2==5"));
    }
    if three == five {
        return Err(anyhow::Error::msg("3==5"));
    }
    if two == three {
        return Err(anyhow::Error::msg("2==3"));
    }
    Ok(())
}

fn insert_map(map: &mut HashMap<usize, String>, number: usize, s: &str) {
    map.insert(number, sort_str(s));
}

fn sort_str(s: &str) -> String {
    let mut ss: Vec<char> = s.chars().collect();
    ss.sort();
    String::from_iter(ss)
}

fn match_sequence(seq1: &&str, seq2: &&str) -> bool {
    seq1.len() == seq2.len() && seq1.chars().all(|x| seq2.contains(x))
}

fn match_digits_with_representation<'a, I: IntoIterator<Item = &'a String>>(
    segments: I,
    digit: &&str,
) -> bool {
    segments
        .into_iter()
        .any(|x: &String| match_sequence(&x.as_str(), digit))
}

fn reverse_map(map: HashMap<usize, String>) -> HashMap<String, usize> {
    let mut res = HashMap::with_capacity(map.len());
    for (k, v) in map {
        res.insert(v, k);
    }
    res
}

impl Solution1 for Day8 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let puzzles: Vec<(Vec<&str>, Vec<&str>)> = lines
            .iter()
            .map(|x| {
                x.split_once(" | ")
                    .ok_or(anyhow::Error::msg("Can not divide into two parts"))
            })
            .map(|x| {
                x.and_then(|(segments, digits)| {
                    Ok((
                        segments.split_whitespace().collect(),
                        digits.split_whitespace().collect(),
                    ))
                })
            })
            .collect::<Result<_, _>>()?;
        let mut counter = 0usize;
        for (segments, digits) in &puzzles {
            let easy_segments = find_easy_segments(segments)?;
            for digit in digits {
                if match_digits_with_representation(easy_segments.values(), digit) {
                    counter += 1;
                }
            }
        }
        Ok(counter.to_string())
    }
}

impl Solution2 for Day8 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let puzzles: Vec<(Vec<&str>, Vec<&str>)> = lines
            .iter()
            .map(|x| {
                x.split_once(" | ")
                    .ok_or(anyhow::Error::msg("Can not divide into two parts"))
            })
            .map(|x| {
                x.and_then(|(segments, digits)| {
                    Ok((
                        segments.split_whitespace().collect(),
                        digits.split_whitespace().collect(),
                    ))
                })
            })
            .collect::<Result<_, _>>()?;
        let mut res = 0;
        for (segments, digits) in &puzzles {
            let mut map = find_easy_segments(segments)?;
            find_0_6_9(segments, &mut map)?;
            find_2_3_5(segments, &mut map)?;
            let map = reverse_map(map);
            let mut number = 0usize;
            let mut multiplier = 1000usize;
            for digit in digits {
                let digit = *map
                    .get(&*sort_str(*digit))
                    .ok_or(anyhow::Error::msg("Missing digit"))?;
                number += multiplier * digit;
                multiplier /= 10;
            }
            res += number;
        }
        Ok(res.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() {
        let lines: Vec<String> = collect_file(Part1, "Day8").unwrap();
        let _ = dbg!(Day8::default().run_solution1(lines));
    }

    #[test]
    fn test_solution1() {
        let lines: Vec<String> = collect_file(Test, "Day8").unwrap();
        let _ = dbg!(Day8::default().run_solution1(lines));
    }

    #[test]
    fn solution2() {
        let lines: Vec<String> = collect_file(Part2, "Day8").unwrap();
        let _ = dbg!(Day8::default().run_solution2(lines));
    }

    #[test]
    fn test_solution2() {
        let lines: Vec<String> = collect_file(Test, "Day8").unwrap();
        let _ = dbg!(Day8::default().run_solution2(lines));
    }
}
