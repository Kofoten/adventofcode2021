use crate::challange09::input::ChallangeInput09;

pub fn run(input: ChallangeInput09) -> String {
    let mut risk_level: u32 = 0;

    for i in 0..input.values.len() {
        let current = input.values[i];
        let up_index: i32 = i as i32 - input.width as i32;
        let down_index: usize = i + input.width;
        let left_index: i32 = if i % input.width == 0 { -1 } else { i as i32 -1 };
        let right_index: usize = if i % input.width == input.width - 1 { 0 } else { i + 1 };

        let up: u8 = if up_index < 0 { 9 } else { input.values[up_index as usize] };
        let down: u8 = if down_index < input.values.len() { input.values[down_index] } else { 9 };
        let left: u8 = if left_index < 0 { 9 } else { input.values[left_index as usize] };
        let right: u8 = if right_index < input.values.len() { input.values[right_index] } else { 9 };

        if up > current && down > current && left > current && right > current {
            risk_level += 1 + current as u32;
        }
    }

    risk_level.to_string()
}