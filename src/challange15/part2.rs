use crate::challange15::input::ChallangeInput15;

pub fn run(input: ChallangeInput15) -> String {
    "input.text".to_string()
}

fn enlarge_input(input: &ChallangeInput15, multiplier: usize) -> ChallangeInput15 {
    let width = input.width * multiplier;
    let mut new_input: Vec<Node> = vec![0; nodes.len() * multiplier * multiplier];

    for i in 0..new_input.len() {
        let mult_1 = i / input.width;
    }
}
