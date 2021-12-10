use crate::challange03::input;

pub fn run(input: input::ChallangeInput03) -> String {
    let threshold = (input.values.len() / 2) as u32;
    let mut counters: Vec<u32> = vec![0; input.bit_count];

    for bits in input.values {
        for i in 0..input.bit_count {
            if bits[i] == true {
                counters[i] += 1
            }
        }
    }

    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;
    for i in 0..input.bit_count {
        let ones_is_common = counters[i] > threshold;
        gamma_rate += (ones_is_common as u32) << (input.bit_count - i - 1);
        epsilon_rate += (!ones_is_common as u32) << (input.bit_count - i - 1);
    }

    (gamma_rate * epsilon_rate).to_string()
}