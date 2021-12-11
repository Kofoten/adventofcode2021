use crate::challange08::input::ChallangeInput08;

pub fn run(input: ChallangeInput08) -> String {
    let mut counter: u32 = 0;
    for data in input.values {
        for part in data.output_value {
            let letter_count = part.chars().count();
            if letter_count == 2 || letter_count == 3 || letter_count == 4 || letter_count == 7 {
                counter += 1;
            }
        }
    }
    counter.to_string()
}