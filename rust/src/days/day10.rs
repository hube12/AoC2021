use aoc_2021::{Day, Solution1, Solution2};
use std::convert::TryFrom;

#[derive(Default)]
pub struct Day10;

impl Day for Day10 {}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Token {
    LParenthesis,
    RParenthesis,
    LBrackets,
    RBrackets,
    LCurly,
    RCurly,
    LFish,
    RFish,
}

impl Token {
    // get the token that should be before
    fn previous(&self) -> anyhow::Result<Self> {
        Ok(match self {
            Token::RParenthesis => Token::LParenthesis,
            Token::RBrackets => Token::LBrackets,
            Token::RCurly => Token::LCurly,
            Token::RFish => Token::LFish,
            _ => {
                return Err(anyhow::Error::msg("Not a Right token"));
            }
        })
    }
    #[allow(dead_code)]
    fn next(&self) -> anyhow::Result<Self> {
        Ok(match self {
            Token::LParenthesis => Token::RParenthesis,
            Token::LBrackets => Token::RBrackets,
            Token::LCurly => Token::RCurly,
            Token::LFish => Token::RFish,
            _ => {
                return Err(anyhow::Error::msg("Not a Left token"));
            }
        })
    }

    #[allow(dead_code)]
    fn is_left(&self) -> bool {
        match self {
            Token::LParenthesis | Token::LBrackets | Token::LCurly | Token::LFish => true,
            _ => false,
        }
    }

    fn is_right(&self) -> bool {
        match self {
            Token::RParenthesis | Token::RBrackets | Token::RCurly | Token::RFish => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for Token {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '(' => Token::LParenthesis,
            ')' => Token::RParenthesis,
            '[' => Token::LBrackets,
            ']' => Token::RBrackets,
            '{' => Token::LCurly,
            '}' => Token::RCurly,
            '<' => Token::LFish,
            '>' => Token::RFish,
            _ => {
                return Err(anyhow::Error::msg("Not a valid token for parser"));
            }
        })
    }
}

impl From<Token> for usize {
    fn from(token: Token) -> Self {
        match token {
            Token::RParenthesis => 3,
            Token::RBrackets => 57,
            Token::RCurly => 1197,
            Token::RFish => 25137,
            Token::LParenthesis => 1,
            Token::LBrackets => 2,
            Token::LCurly => 3,
            Token::LFish => 4,
        }
    }
}

impl Solution1 for Day10 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let mut points = 0usize;
        for line in lines {
            let mut stack: Vec<Token> = Vec::with_capacity(line.len());
            for c in line.chars() {
                let token = Token::try_from(c)?;
                if token.is_right() {
                    let expected = token.previous()?;
                    if let Some(was) = stack.pop() {
                        if was != expected {
                            points += usize::from(token);
                            break;
                        }
                    }
                } else {
                    stack.push(token);
                }
            }
        }
        Ok(points.to_string())
    }
}

impl Solution2 for Day10 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let mut points = Vec::with_capacity(lines.len());
        for line in lines {
            let mut point = 0usize;
            let mut stack: Vec<Token> = Vec::with_capacity(line.len());
            let mut corrupted = false;
            for c in line.chars() {
                let token = Token::try_from(c)?;
                if token.is_right() {
                    let expected = token.previous()?;
                    if let Some(was) = stack.pop() {
                        if was != expected {
                            corrupted = true;
                            break;
                        }
                    }
                } else {
                    stack.push(token);
                }
            }
            if !corrupted {
                while let Some(missing) = stack.pop() {
                    if missing.is_right() {
                        return Err(anyhow::Error::msg("Should nto be a right token"));
                    }
                    point *= 5;
                    point += usize::from(missing);
                }
                points.push(point);
            }
        }
        points.sort();
        Ok(points[points.len() / 2].to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() {
        let lines: Vec<String> = collect_file(Part1, "Day10").unwrap();
        let _ = dbg!(Day10::default().run_solution1(lines));
    }

    #[test]
    fn test_solution1() {
        let lines: Vec<String> = collect_file(Test, "Day10").unwrap();
        let _ = dbg!(Day10::default().run_solution1(lines));
    }

    #[test]
    fn solution2() {
        let lines: Vec<String> = collect_file(Part2, "Day10").unwrap();
        let _ = dbg!(Day10::default().run_solution2(lines));
    }

    #[test]
    fn test_solution2() {
        let lines: Vec<String> = collect_file(Test, "Day10").unwrap();
        let _ = dbg!(Day10::default().run_solution2(lines));
    }
}
