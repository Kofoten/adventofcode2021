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
    let up_allowed = index >= width;
    let left_allowed = index % width > 0;

    if up_allowed && left_allowed {
        if nodes[index - width].path_risk < nodes[index - 1].path_risk {
            index - width
        } else {
            index - 1
        }
    } else if up_allowed {
        index - width
    } else if left_allowed {
        index - 1
    } else {
        0
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
