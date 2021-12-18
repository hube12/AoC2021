use aoc_2021::{Day, Solution1, Solution2};
use std::borrow::Borrow;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Default)]
pub struct Day18;

impl Day for Day18 {}

impl Solution1 for Day18 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let mut tree = parse(lines[0].as_str())?;
        dbg!(tree);
        // for i in 1..lines.len() {
        //     node = join(node, parse(lines[i].as_str()));
        // }
        // let mag=magnitude(unsafe { &*node });
        // Ok(mag.to_string())
        Ok(0.to_string())
    }
}

impl Solution2 for Day18 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let mut max_mag = 0;
        // for i in 0..lines.len() {
        //     for j in 0..lines.len() {
        //         if i == j {
        //             continue;
        //         }
        //         let node = join(parse(lines[i].as_str()), parse(lines[j].as_str()));
        //         max_mag = std::cmp::max(max_mag, magnitude(unsafe { &*node }));
        //     }
        // }
        Ok(max_mag.to_string())
    }
}

/// A simple man Tree where we store nodes in a map with an id
struct Tree {
    root: usize,
    nodes: HashMap<usize, Node>,
    max_id: usize,
}

impl Debug for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut stack = vec![self.root];
        while !stack.is_empty() {
            writeln!(f, "{}", stack.iter().map(|x| x.to_string() + ",").collect::<String>())?;
            let mut new_stack = vec![];
            for s in stack {
                if let Some(node) = self.get(s) {
                    if let Some(left) = node.left_child {
                        new_stack.push(left);
                    }
                    if let Some(right) = node.right_child {
                        new_stack.push(right);
                    }
                } else {
                    panic!("Impossible");
                }
            }
            stack = new_stack;
        }
        Ok(())
    }
}

impl Tree {
    fn new(root: Node) -> Self {
        let mut map = HashMap::new();
        map.insert(0, root);
        Tree {
            root: 0,
            nodes: map,
            max_id: 1,
        }
    }
    fn insert(&mut self, node: Node) -> usize {
        let id = self.max_id;
        self.max_id += 1;
        self.nodes.insert(id, node);
        id
    }

    fn get(&self, id: usize) -> Option<&Node> {
        self.nodes.get(&id)
    }

    fn get_mut(&mut self, id: usize) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }
}

#[derive(Debug)]
struct Node {
    data: u32,
    parent: Option<usize>,
    is_leaf: bool,
    left_child: Option<usize>,
    right_child: Option<usize>,
}

impl Node {
    fn new(data: u32, parent: Option<usize>, is_leaf: bool, left_child: Option<usize>, right_child: Option<usize>) -> Self {
        Node {
            data,
            parent,
            is_leaf,
            left_child,
            right_child,
        }
    }
}

// fn split(node: &mut Node) -> bool {
//     if node.is_leaf {
//         if node.data < 10 {
//             return false;
//         }
//         let data = node.data;
//         node.is_leaf = false;
//         node.child = [
//             g_node(data / 2, node, true, [None; 2]),
//             g_node(data - data / 2, node, true, [None; 2]),
//         ];
//         true
//     } else {
//         split(node.child[0]) || split(node.child[1])
//     }
// }

fn explode(tree: &mut Tree, current_node:Option<usize>, depth: usize) -> anyhow::Result<bool> {
    if let Some(current_node) = current_node {
        let node = tree.get_mut(starting_node).ok_or(anyhow::Error::msg(format!("Missing Node for id : {}", current_node)))?;
        if node.is_leaf {
            return Ok(false);
        }
        if explode(tree, node.left_child, depth + 1)? || explode(tree,node.right_child, depth + 1)? {
            return Ok(true);
        }
        if depth < 4 {
            return Ok(false);
        }
        // left child
        {
            let left_child=
            let data = node.left_child.data;
            let mut node = node;
            let mut parent = unsafe { &*node }.parent;
            while !parent.is_null() && unsafe { &*parent }.child[i] == node {
                node = parent;
                parent = unsafe { &*node }.parent;
            }
            if !parent.is_null() {
                node = unsafe { &*parent }.child[i];
                while !unsafe { &*node }.is_leaf {
                    node = unsafe { &*node }.child[1 - i];
                }
                unsafe { &mut *node }.data += data;
            }
        }

        for i in [0, 1] {

        }
        node.is_leaf = true;
        node.data = 0;
        true
    }else {
        return Ok(true);
    }
}

fn magnitude(tree: &Tree, starting_node: Option<usize>) -> anyhow::Result<usize> {
    if let Some(starting_node) = starting_node {
        let node = tree.get(starting_node).ok_or(anyhow::Error::msg(format!("Missing Node for id : {}", starting_node)))?;
        if node.is_leaf {
            Ok(node.data as usize)
        } else {
            Ok(3 * magnitude(tree, node.left_child)? + 2 * magnitude(tree, node.right_child)?)
        }
    } else {
        return Ok(0);
    }
}

fn parse(line: &str) -> anyhow::Result<Tree> {
    let mut tree = Tree::new(Node::new(0, None, false, None, None));
    let mut current = tree.root;
    let mut stack = vec![];
    for c in line.chars() {

        match c {
            '[' => {
                let left_child = tree.insert(Node::new(0, Some(current), false, None, None));
                let node = tree.get_mut(current).ok_or(anyhow::Error::msg(format!("Missing Node for id : {}", current)))?;
                node.left_child = Some(left_child);
                stack.push(current);
                current = left_child;
            }
            ',' => {
                let right_child = tree.insert(Node::new(0, Some(current), false, None, None));
                let node = tree.get_mut(current).ok_or(anyhow::Error::msg(format!("Missing Node for id : {}", current)))?;
                let parent_id = node.parent.ok_or(anyhow::Error::msg(format!("Missing parent for id : {}", current)))?;
                let parent = tree.get_mut(parent_id).ok_or(anyhow::Error::msg(format!("Missing Node for id : {}", parent_id)))?;
                parent.right_child = Some(right_child);
                current = right_child;
            }
            ']' => current = stack.pop().unwrap(),
            d => {
                let node = tree.get_mut(current).ok_or(anyhow::Error::msg(format!("Missing Node for id : {}", current)))?;
                node.data = d.to_digit(10).unwrap();
                node.is_leaf = true;
            }
        }
    }
    Ok(tree)
}

// fn join(left: *mut Node, right: *mut Node) -> *mut Node {
//     let node = g_node(0, std::ptr::null_mut(), false, [left, right]);
//     unsafe { &mut *left }.parent = node;
//     unsafe { &mut *right }.parent = node;
//     while explode(node, 0) || split(node) {}
//     node
// }


#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day18").unwrap();
        Ok(assert_eq!(
            Day18::default().run_solution1(lines)?,
            String::from("3793")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day18").unwrap();
        Ok(assert_eq!(
            Day18::default().run_solution1(lines)?,
            String::from("4140")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day18").unwrap();
        Ok(assert_eq!(
            Day18::default().run_solution2(lines)?,
            String::from("4695")
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day18").unwrap();
        Ok(assert_eq!(
            Day18::default().run_solution2(lines)?,
            String::from("3993")
        ))
    }
}