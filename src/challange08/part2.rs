use crate::challange08::input::ChallangeInput08;
use crate::challange08::decoder::Decoder;

pub fn run(input: ChallangeInput08) -> String {
    let mut sum: u32 = 0;

    for data in input.values {
        let mut one: String = String::new();
        let mut four: String = String::new();
        let mut seven: String = String::new();
        let mut eight: String = String::new();

        for value in data.signal_pattern {
            let letter_count = value.chars().count();
            match letter_count {
                2 => one = value,
                3 => seven = value,
                4 => four = value,
                7 => eight = value,
                _ => continue,
            };

            if !(one.is_empty() || four.is_empty() || seven.is_empty() || eight.is_empty()) {
                break;
            } 
        }

        let decoder: Decoder = Decoder::new(one, four, seven, eight);
        let out_val_len: usize = data.output_value.len();
        let mut out_value: u32 = 0;
        for i in 0..out_val_len {
            let decoded = decoder.decode(&data.output_value[i]);
            let multiplier = 10u32.pow((out_val_len - i - 1) as u32);
            out_value += decoded * multiplier;
        }

        sum += out_value;
    }

    sum.to_string()
}