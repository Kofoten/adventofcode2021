use crate::challange01::input;

pub fn run(input: input::ChallangeInput01) -> String {
    let mut sums: Vec<u32> = vec![0; input.values.len()];
    let mut counter: u32 = 0;

    for i in 0..input.values.len() {
        let inner_start = if i < 2 { 0 } else { i - 2 };
        for j in inner_start..i + 1 {
            sums[j] += input.values[i];
        }
    }

    for i in 1..sums.len() {
        if sums[i - 1] < sums[i] {
            counter += 1;
        }
    }
    counter.to_string()
}