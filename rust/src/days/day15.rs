use aoc_2021::{Day, Pos, Solution1, Solution2, get_adjacent_positions, CROSS};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::{ Ordering, max};

#[derive(Default)]
pub struct Day15;

impl Day for Day15 {}

type Cost = u32;
type Adjacents = Vec<(Pos, Cost)>;
type Graph = HashMap<Pos, Adjacents>;

fn make_graph(matrix: Vec<Vec<u32>>, length: usize, height: usize) -> anyhow::Result<Graph> {
    let mut map = HashMap::with_capacity(length * height);
    for y in 0..height {
        for x in 0..length {
            let entry = map.entry(Pos::new(x, y)).or_insert(Vec::with_capacity(4));
            let positions = get_adjacent_positions::<CROSS>(x, y, height, length)?;
            for pos in positions {
                entry.push((pos, matrix[pos.y()][pos.x()]))
            }
        }
    }
    Ok(map)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<'a>(&'a Pos, usize);

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
            .then_with(|| self.0.cmp(&other.0))
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_min_cost<'a>(start: &'a Pos, destination: &Pos, graph: &'a Graph) -> Option<usize> {
    let mut distances = HashMap::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert(start, 0usize);
    to_visit.push(State(start, 0usize));

    while let Some(State(current, cost)) = to_visit.pop() {
        if current == destination {
            return Some(cost);
        }
        if cost > *distances.get(current).unwrap_or(&usize::MAX) {
            continue;
        }
        if let Some(neighbors) = graph.get(current) {
            for (neighbor, jump_cost) in neighbors {
                let new_distance = cost + *jump_cost as usize;
                if new_distance < *distances.get(&neighbor).unwrap_or(&usize::MAX) {
                    distances.insert(neighbor, new_distance);
                    to_visit.push(State(neighbor, new_distance));
                }
            }
        }
    }
    None
}

impl Solution1 for Day15 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let matrix: Vec<Vec<u32>> = lines
            .iter()
            .map(|x| {
                x.chars()
                    .map(|x| {
                        x.to_digit(10)
                            .ok_or(anyhow::Error::msg("Not a valid digit"))
                    })
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;
        let length = matrix
            .first()
            .ok_or(anyhow::Error::msg("Need one line"))?
            .len();
        let height = matrix.len();
        if height < 1 || length < 1 {
            return Err(anyhow::Error::msg("Need at least a Node"));
        }
        let graph = make_graph(matrix, length, height)?;
        let start = Pos::new(0, 0);
        let destination = Pos::new(length - 1, height - 1);

        let min_cost = find_min_cost(&start, &destination, &graph).ok_or(anyhow::Error::msg("Can not compute cost"))?;
        Ok(min_cost.to_string())
    }
}

fn copy_5x5(matrix:Vec<Vec<u32>>) -> anyhow::Result<(Vec<Vec<u32>>,usize,usize)> {
    let length = matrix
        .first()
        .ok_or(anyhow::Error::msg("Need one line"))?
        .len();
    let height = matrix.len();
    let mut full_cave = Vec::with_capacity(5 * height);
    for _ in 0..5 {
        for _ in 0..height {
            full_cave.push(vec![0; length * 5]);
        }
    }

    for y in 0..height {
        for x in 0..length {
            full_cave[y][x] = matrix[y][x];
        }
    }

    for y_copy in 0..5 {
        let y_idx = (if y_copy == 0 { 0 } else { y_copy - 1 }) * height;
        let y_offset = y_copy * height;
        for x_copy in 0..=0 {
            if x_copy == 0 && y_copy == 0 {
                continue;
            }
            let x_offset = x_copy * length;
            for y in 0..height {
                for x in 0..length {
                    full_cave[y + y_offset][x + x_offset] = max(1, (full_cave[y + y_idx][x] + 1) % 10);
                }
            }
        }
    }

    for y_copy in 0..5 {
        let y_idx = y_copy * height;
        let y_offset = y_copy * height;
        for x_copy in 1..5 {
            if x_copy == 0 && y_copy == 0 {
                continue;
            }
            let x_offset = x_copy * length;
            let x_idx = (x_copy - 1) * length;
            for y in 0..height {
                for x in 0..length {
                    full_cave[y + y_offset][x + x_offset] = max(1, (full_cave[y + y_idx][x+x_idx] + 1) % 10);
                }
            }
        }
    }
    Ok((full_cave,height*5,length*5))
}

impl Solution2 for Day15 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let matrix: Vec<Vec<u32>> = lines
            .iter()
            .map(|x| {
                x.chars()
                    .map(|x| {
                        x.to_digit(10)
                            .ok_or(anyhow::Error::msg("Not a valid digit"))
                    })
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;
        let (full_cave,height,length)=copy_5x5(matrix)?;
        let start = Pos::new(0, 0);
        let destination = Pos::new(length - 1, height - 1);
        let graph=make_graph(full_cave,length,height)?;
        let min_cost = find_min_cost(&start, &destination, &graph).ok_or(anyhow::Error::msg("Can not compute cost"))?;
        Ok(min_cost.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day15").unwrap();
        Ok(assert_eq!(
            Day15::default().run_solution1(lines)?,
            String::from("609")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day15").unwrap();
        Ok(assert_eq!(
            Day15::default().run_solution1(lines)?,
            String::from("40")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day15").unwrap();
        Ok(assert_eq!(
            Day15::default().run_solution2(lines)?,
            String::from("2925")
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day15").unwrap();
        Ok(assert_eq!(
            Day15::default().run_solution2(lines)?,
            String::from("315")
        ))
    }
}
