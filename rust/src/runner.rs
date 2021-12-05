use crate::days::*;
use aoc_2021::Part::{Part1, Part2};
use aoc_2021::{collect_file, Solution1, Solution2, valid_day, handle_solution, Day};



mod days;

fn match_day(x: usize) -> Box<dyn Day> {
    match x {
        1 => Box::from(Day1),
        2 => Box::from(Day2),
        _ => unimplemented!("Missing day")
    }
}


fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    println!("Choose day to execute");
    loop {
        stdin.read_line(&mut buffer)?;
        let day = buffer.trim();
        match valid_day(day) {
            Ok(x) => {
                println!("Running day {}", x);
                println!("---------------------------------");
                let runner = match_day(x);
                println!("Running Part 1");
                let lines: Vec<String> = collect_file(Part1, day)?;
                let time = std::time::Instant::now();
                let r = runner.run_solution1(lines);
                let time = std::time::Instant::now().duration_since(time).as_micros();
                handle_solution(r);
                println!("Took {}us", time);
                println!("---------------------------------");

                println!("Running Part 2");
                let lines: Vec<String> = collect_file(Part2, day)?;
                let time = std::time::Instant::now();
                let r = runner.run_solution2(lines);
                let time = std::time::Instant::now().duration_since(time).as_micros();
                handle_solution(r);
                println!("Took {}us", time);
                println!("---------------------------------");
                break;
            }
            Err(err) => {
                eprintln!("You didn't input a valid day, try again : {}", err);
                buffer.clear();
            }
        }
    }
    Ok(())
}
