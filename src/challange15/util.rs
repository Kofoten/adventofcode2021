use crate::challange15::node::Node;
use colored::*;
use std::collections::HashSet;

pub fn calculate_min_path_risk(nodes: &mut Vec<Node>, width: usize) -> Option<u32> {
    let mut nexts: Vec<HashSet<usize>> = vec![HashSet::from([1, width])];

    while let Some(indicies) = nexts.pop() {
        let mut next: HashSet<usize> = HashSet::new();

        for index in indicies {
            let previous_index = get_min_risk_preceding_neighbour(&nodes, width, index);
            nodes[index].previous = previous_index;
            nodes[index].path_risk = nodes[index].risk_level + nodes[previous_index].path_risk;
            insert_next_neighbours(&mut next, nodes.len(), width, index);
        }

        if next.len() > 0 {
            nexts.push(next);
        }
    }

    print_path(nodes, width);

    if let Some(last) = nodes.iter().last() {
        Some(last.path_risk)
    } else {
        None
    }
}

pub fn get_min_risk_preceding_neighbour(nodes: &Vec<Node>, width: usize, index: usize) -> usize {
    let mut neighbours: Vec<usize> = Vec::new();

    let up_allowed: bool = index >= width;
    let down_allowed: bool = index + width < length;
    let left_allowed: bool = index % width > 0;
    let right_allowed: bool = index % width < width - 1;

    if up_allowed {
        neighbours.push(index - width);

        if left_allowed {
            neighbours.push(index - width - 1);
        }

        if right_allowed {
            neighbours.push(index - width + 1);
        }
    }

    if down_allowed {
        neighbours.push(index + width);

        if left_allowed {
            neighbours.push(index + width - 1);
        }

        if right_allowed {
            neighbours.push(index + width + 1);
        }
    }

    if left_allowed {
        neighbours.push(index - 1);
    }

    if right_allowed {
        neighbours.push(index + 1);
    }

    for i in neighbours {
        let new_risk = nodes[index].path_risk + nodes[i].risk_level;
        if new_risk < nodes[i].path_risk {
            nodes[i].path_risk = new_risk;
            nodes[i].previous = index;
            next.push(i)
        }
    }
}

pub fn insert_next_neighbours(
    next: &mut HashSet<usize>,
    length: usize,
    width: usize,
    index: usize,
) {
    if index + width < length {
        next.insert(index + width);
    }

    if index % width < width - 1 {
        next.insert(index + 1);
    }
}

fn print_path(nodes: &Vec<Node>, width: usize) {
    let mut path: HashSet<usize> = HashSet::from([nodes.len() - 1, 0]);
    let mut previous = nodes.iter().last().unwrap();

    while previous.previous > 0 {
        path.insert(previous.previous);
        previous = &nodes[previous.previous];
    }

    for i in 0..nodes.len() {
        if i % width == 0 {
            println!();
        }

        if path.contains(&i) {
            print!("{}", nodes[i].risk_level.to_string().cyan());
        } else {
            print!("{}", nodes[i].risk_level.to_string());
        }
    }

    println!();
}

pub fn calculate_min_path_risk_old(nodes: &mut Vec<Node>, width: usize) -> Option<u32> {
    let mut next: Vec<usize> = vec![0];

    while let Some(index) = next.pop() {
        let neighbours = get_neighbours(nodes.len(), width, index);

        for i in neighbours {
            let new_risk = nodes[index].path_risk + nodes[i].risk_level;
            if new_risk < nodes[i].path_risk {
                nodes[i].path_risk = new_risk;
                nodes[i].previous = index;
                next.push(i)
            }
        }
    }

    print_path(nodes, width);

    if let Some(last) = nodes.iter().last() {
        Some(last.path_risk)
    } else {
        None
    }
}

pub fn get_neighbours(length: usize, width: usize, index: usize) -> Vec<usize> {
    let mut neighbours: Vec<usize> = Vec::new();

    if index >= width {
        neighbours.push(index - width);
    }

    if index + width < length {
        neighbours.push(index + width);
    }

    if index % width > 0 {
        neighbours.push(index - 1);
    }

    if index % width < width - 1 {
        neighbours.push(index + 1);
    }

    neighbours
}
