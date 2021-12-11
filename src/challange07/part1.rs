use crate::challange07::input::ChallangeInput07;

pub fn run(input: ChallangeInput07) -> String {
    let max: usize = *input.positions.iter().max().unwrap() as usize;
    let mut weighted_positions: Vec<u32> = vec![0; 1 + max];

    for position in input.positions {
        for i in 0..weighted_positions.len() {
            weighted_positions[i] += (i as i32 - position).abs() as u32;
        }
    }

    (*weighted_positions.iter().min().unwrap()).to_string()
}