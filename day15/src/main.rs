use itertools::*;

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    up: Option<Rc<RefCell<Node>>>,
    down: Option<Rc<RefCell<Node>>>,
    risk_level: u8,
    id: [u16; 2],
}

impl Node {
    fn new(risk_level: u8, id: [u16; 2]) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            left: None,
            right: None,
            up: None,
            down: None,
            risk_level,
            id,
        }))
    }
}

struct Graph {
    start: Rc<RefCell<Node>>,
}

impl Graph {
    fn from_grid(grid: &Vec<Vec<u8>>) -> Graph {
        // first node
        let graph = Graph {
            start: Node::new(grid[0][0], [0, 0]),
        };

        // first line of nodes
        let mut left_node = graph.start.clone();
        for j in 1..grid[0].len() {
            let risk_level = grid[0][j];
            let new_node = Node::new(risk_level, [0, j as u16]);

            left_node.borrow_mut().right = Some(new_node.clone());
            new_node.borrow_mut().left = Some(left_node);

            left_node = new_node;
        }

        // remaining lines
        let mut start_of_top_line = graph.start.clone();
        let mut top_node = start_of_top_line.clone();
        for i in 1..grid.len() {
            let line = &grid[i];

            // first node of line
            {
                let new_node = Node::new(line[0], [i as u16, 0]);

                top_node.borrow_mut().down = Some(new_node.clone());
                new_node.borrow_mut().up = Some(top_node.clone());

                left_node = new_node;
                let new_top_node = top_node
                    .try_borrow()
                    .unwrap()
                    .right
                    .as_ref()
                    .unwrap()
                    .clone();
                top_node = new_top_node;
            }

            // rest of line
            for j in 1..line.len() {
                let risk_level = line[j];
                let new_node = Node::new(risk_level, [i as u16, j as u16]);

                left_node.borrow_mut().right = Some(new_node.clone());
                new_node.borrow_mut().left = Some(left_node.clone());
                top_node.borrow_mut().down = Some(new_node.clone());
                new_node.borrow_mut().up = Some(top_node.clone());

                left_node = new_node;
                let maybe_new_top_node = top_node.try_borrow().unwrap().right.clone();
                if let Some(new_top_node) = maybe_new_top_node {
                    top_node = new_top_node;
                }
            }

            let new_start_of_top_line = start_of_top_line
                .try_borrow()
                .unwrap()
                .down
                .as_ref()
                .unwrap()
                .clone();
            start_of_top_line = new_start_of_top_line;
            top_node = start_of_top_line.clone();
        }

        graph
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ReachableNode {
    node: Rc<RefCell<Node>>,
    sum: usize,
}

impl PartialOrd for ReachableNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Reverse(self.sum).cmp(&Reverse(other.sum)))
    }
}

impl Ord for ReachableNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn process_neighbor(
    sum: usize,
    maybe_neighbor: Option<Rc<RefCell<Node>>>,
    end: [u16; 2],
    reachable: &mut BinaryHeap<ReachableNode>,
    visited: &mut HashSet<[u16; 2]>,
) -> Option<usize> {
    if let Some(neighbor) = maybe_neighbor {
        let new_sum = sum + neighbor.try_borrow().unwrap().risk_level as usize;
        let id = neighbor.try_borrow().unwrap().id;
        if id == end {
            return Some(new_sum);
        }
        if !visited.contains(&id) {
            reachable.push(ReachableNode {
                node: neighbor,
                sum: new_sum,
            });
            visited.insert(id);
        }
    }
    None
}

fn dykstra(graph: &Graph, end: [u16; 2]) -> usize {
    let mut reachable = BinaryHeap::new();
    reachable.push(ReachableNode {
        node: graph.start.clone(),
        sum: 0,
    });
    let mut visited = HashSet::new();
    visited.insert(graph.start.try_borrow().unwrap().id);
    loop {
        let next = reachable.pop().unwrap();

        let neighbors = [
            next.node.try_borrow().unwrap().left.clone(),
            next.node.try_borrow().unwrap().right.clone(),
            next.node.try_borrow().unwrap().up.clone(),
            next.node.try_borrow().unwrap().down.clone(),
        ];
        for maybe_neighbor in neighbors {
            if let Some(result) =
                process_neighbor(next.sum, maybe_neighbor, end, &mut reachable, &mut visited)
            {
                return result;
            }
        }
    }
}

fn part1(grid: &Vec<Vec<u8>>) {
    let end = [grid.len() as u16 - 1, grid[0].len() as u16 - 1];
    let graph = Graph::from_grid(grid);
    let risk_level = dykstra(&graph, end);
    println!("{:?}", risk_level)
}

fn part2(mut grid: Vec<Vec<u8>>) {
    let num_new_cols = grid[0].len() * 4;
    for i in 0..grid.len() {
        for j in 0..num_new_cols {
            let next = grid[i][j] % 9 + 1;
            grid[i].push(next);
        }
    }

    let num_new_lines = grid.len() * 4;
    for i in 0..num_new_lines {
        grid.push(grid[i].iter().map(|&r| r % 9 + 1).collect_vec());
    }

    let end = [grid.len() as u16 - 1, grid[0].len() as u16 - 1];
    let graph = Graph::from_grid(&grid);
    let risk_level = dykstra(&graph, end);
    println!("{:?}", risk_level)
}

fn main() {
    let grid = include_str!("../input/input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    part1(&grid);

    part2(grid);
}
