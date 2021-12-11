use crate::challange07::input::ChallangeInput07;

pub fn run(input: ChallangeInput07) -> String {
    let max: usize = *input.positions.iter().max().unwrap() as usize;
    let mut weighted_positions: Vec<u32> = vec![0; 1 + max];

    for position in input.positions {
        for i in 0..weighted_positions.len() {
            weighted_positions[i] += calculate_fuel_consumption(position, i as i32);
        }
    }

    (*weighted_positions.iter().min().unwrap()).to_string()
}

fn calculate_fuel_consumption(from: i32, to: i32) -> u32 {
    let steps: u32 = (to - from).abs() as u32;
    let half: u32 = steps / 2;

    if steps == 0 || half == 0 {
        steps
    } else {
        let add: u32 = steps % half;
        if add == 1 {
            steps * (half + add)
        } else {
            steps * half + half
        }
    }
}