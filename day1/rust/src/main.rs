use std::fs;
use std::cmp::min;
use std::time::Instant;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, AddAssign, Add};
use std::path::PathBuf;
use std::str::FromStr;


fn solution_part1<Input: Eq + Ord>(lines: Vec<Input>) -> anyhow::Result<u64> {
    let mut iter = lines.iter();
    let mut last = iter.next().ok_or(anyhow::Error::msg("Missing at least an element"))?;
    let mut counter = 0u64;
    while let Some(next) = iter.next() {
        if last < next {
            counter += 1;
        }
        last = next
    }
    Ok(counter)
}

fn solution_part2(lines: Vec<usize>) -> anyhow::Result<usize> {
    let mut iter = lines.iter();
    let (mut slide1,mut slide2,mut slide3)=(0usize, 0usize, 0usize);
    let next=iter.next().ok_or(anyhow::Error::msg("Missing at least 1 element"))?;
    slide1+=*next;
    let next=iter.next().ok_or(anyhow::Error::msg("Missing at least 2 elements"))?;
    slide1+=*next;
    slide2+=*next;
    let next=iter.next().ok_or(anyhow::Error::msg("Missing at least 3 elements"))?;
    slide1+=*next;
    slide2+=*next;
    slide3+=*next;
    let mut last =slide1;
    let mut counter = 0usize;
    while let Some(next) = iter.next() {
        slide1=slide2+*next;
        slide2=slide3+*next;
        slide3=*next;
        if last < slide1 {
            counter += 1;
        }
        // update rolling buffer
        last=slide1;
    }
    Ok(counter)
}

fn handle_solution<T: Display>(solution: anyhow::Result<T>) {
    match solution {
        Ok(solution) => println!("Solution is {}", solution),
        Err(error) => eprintln!("Error solution was not found with error :\n{}", error)
    }
}

#[derive(Copy, Clone)]
enum Part {
    Part1,
    Part2,
    Test,
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Part1 => write!(f, "input.txt"),
            Part::Part2 => write!(f, "input.txt"),
            Part::Test => write!(f, "test.txt"),
        }
    }
}

fn collect_file<T: Ord + Eq + Clone+ std::str::FromStr>(part: Part)->anyhow::Result<Vec<T>> where <T as FromStr>::Err: std::error::Error, <T as FromStr>::Err: Send, <T as FromStr>::Err: Sync{
    let input = fs::read_to_string(format!("../input/{}", part))?;
    let lines = input.lines();
    let mut res: Vec<T> = vec![];
    for line in lines {
        res.push(line.parse::<T>().map_err(|err| anyhow::Error::msg(format!("Could not parse with error {:?}",err)))?)
    }
    Ok(res)
}

fn main()->anyhow::Result<()> {
    let part=Part::Test;
    let lines=collect_file::<usize>(part)?;
    println!("Running {}",part);
    let now = Instant::now();
    handle_solution(solution_part1(lines));
    println!("Took {}us", now.elapsed().as_micros());

    let part=Part::Part1;
    let lines=collect_file::<usize>(part)?;
    println!("Running {}",part);
    let now = Instant::now();
    handle_solution(solution_part1(lines));
    println!("Took {}us", now.elapsed().as_micros());

    let part=Part::Test;
    let lines=collect_file::<usize>(part)?;
    println!("Running {}",part);
    let now = Instant::now();
    handle_solution(solution_part2(lines));
    println!("Took {}us", now.elapsed().as_micros());

    let part=Part::Part2;
    let lines=collect_file::<usize>(part)?;
    println!("Running {}",part);
    let now = Instant::now();
    handle_solution(solution_part2(lines));
    println!("Took {}us", now.elapsed().as_micros());

    Ok(())
}
