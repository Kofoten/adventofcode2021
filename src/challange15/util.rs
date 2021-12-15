use crate::challange15::node::Node;
use colored::*;
use std::collections::HashSet;

pub fn calculate_min_path_risk(nodes: &mut Vec<Node>, width: usize) -> Option<u32> {
    let mut nexts: Vec<HashSet<usize>> = vec![HashSet::from([1, width])];

    while let Some(indicies) = nexts.pop() {
        let mut next: HashSet<usize> = HashSet::new();

        for index in indicies {
            let mut neighbours: Vec<usize> = Vec::new();

            if index >= width {
                neighbours.push(index - width);
            }

            if index + width < nodes.len() {
                neighbours.push(index + width);
            }

            if index % width > 0 {
                neighbours.push(index - 1);
            }

            if index % width < width - 1 {
                neighbours.push(index + 1);
            }

            let mut min = neighbours[0];
            for i in 1..neighbours.len() {
                if nodes[neighbours[i]].path_risk < nodes[min].path_risk {
                    min = neighbours[i]
                }
            }

            nodes[index].previous = min;
            nodes[index].path_risk = nodes[index].risk_level + nodes[min].path_risk;

            for neighbour in neighbours {
                if nodes[neighbour].path_risk > nodes[index].path_risk {
                    next.insert(neighbour);
                }
            }
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
