use crate::challange15::node::Node;
use crate::core_challange::ChallangeInput;

pub struct ChallangeInput15 {
    is_first_line: bool,
    pub width: usize,
    pub nodes: Vec<Node>,
}

impl Default for ChallangeInput15 {
    fn default() -> Self {
        ChallangeInput15 {
            is_first_line: true,
            width: 0,
            nodes: Vec::new(),
        }
    }
}

impl ChallangeInput for ChallangeInput15 {
    fn parse_line(&mut self, line: String) {
        let numbers: Vec<Node> = line
            .chars()
            .map(|c| Node::from(c.to_digit(10).unwrap()))
            .collect();

        if self.is_first_line {
            self.is_first_line = false;
            self.width = numbers.len();
        }

        for number in numbers {
            self.nodes.push(number);
        }
    }
}
