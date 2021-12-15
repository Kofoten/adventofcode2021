use crate::challange15::node::Node;

pub fn calculate_min_path_risk(nodes: &mut Vec<Node>, width: usize) -> Option<u32> {
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
