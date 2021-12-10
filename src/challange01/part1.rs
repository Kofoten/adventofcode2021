use crate::challange01::input::ChallangeInput01;

pub fn run(input: ChallangeInput01) -> String {
    let mut counter: u32 = 0;
    for i in 1..input.values.len() {
        if input.values[i - 1] < input.values[i] {
            counter += 1;
        }
    }
    counter.to_string()
}