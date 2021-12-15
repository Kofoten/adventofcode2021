use crate::challange15::input::ChallangeInput15;
use crate::challange15::node::Node;
use crate::challange15::util;

pub fn run(input: ChallangeInput15) -> String {
    let mut large = enlarge_input(&input, 5);
    large.nodes[0].path_risk = 0;

    if let Some(path_risk) = util::calculate_min_path_risk(&mut large.nodes, large.width) {
        path_risk.to_string()
    } else {
        String::from("shit")
    }
}

fn enlarge_input(input: &ChallangeInput15, multiplier: usize) -> ChallangeInput15 {
    let source_rows = input.nodes.len() / input.width;
    let width = input.width * multiplier;
    let length = input.nodes.len() * multiplier * multiplier;
    let mut nodes: Vec<Node> = Vec::new();

    for i in 0..length {
        let row = i / width;
        let block_row = row / source_rows;
        let source_index = i % input.width + (row % source_rows) * input.width;
        let add: u32 = (i % width / input.width + block_row) as u32;
        let new_node = Node {
            risk_level: calculate_risk_level(&input.nodes[source_index], add),
            previous: 0,
            path_risk: u32::MAX,
        };

        nodes.push(new_node);
    }

    let mut new_input = ChallangeInput15::default();
    new_input.width = width;
    new_input.nodes = nodes;
    new_input
}

fn calculate_risk_level(node: &Node, add: u32) -> u32 {
    let risk_level = node.risk_level + add;
    if risk_level > 9 {
        risk_level - 9
    } else {
        risk_level
    }
}
