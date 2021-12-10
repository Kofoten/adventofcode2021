use crate::challange03::input::{ChallangeInput03, DiagnosticData};

pub fn run(input: ChallangeInput03) -> String {
    let mut oxygen_generator_ratings: Vec<DiagnosticData> = input.values.clone();
    let mut co2_scrubber_ratings: Vec<DiagnosticData> = input.values.clone();

    for i in 0..input.bit_count {
        if oxygen_generator_ratings.len() > 1 {
            oxygen_generator_ratings = filter_by_commonality(oxygen_generator_ratings, i, true, 1);
        }

        if co2_scrubber_ratings.len() > 1 {
            co2_scrubber_ratings = filter_by_commonality(co2_scrubber_ratings, i, false, 0);
        }
    }

    let oxygen_generator_rating = diagnostic_data_to_i32(oxygen_generator_ratings[0].clone(), input.bit_count);
    let co2_scrubber_rating = diagnostic_data_to_i32(co2_scrubber_ratings[0].clone(), input.bit_count);

    (oxygen_generator_rating * co2_scrubber_rating).to_string()
}

fn filter_by_commonality(values: Vec<DiagnosticData>, index: usize, use_most_common: bool, equality_decider: usize) -> Vec<DiagnosticData> {
    let mut filtered: Vec<Vec<DiagnosticData>> = vec![Vec::new(); 2];

    for i in 0..values.len() {
        if values[i][index] == true {
            filtered[1].push(values[i].clone());
        } else {
            filtered[0].push(values[i].clone());
        }
    }

    let most_common: usize;
    if filtered[0].len() == filtered[1].len() {
        most_common = equality_decider;
    } else if use_most_common {
        most_common = (filtered[1].len() > filtered[0].len()) as usize;
    } else {
        most_common = (filtered[1].len() < filtered[0].len()) as usize;
    }

    return filtered[most_common].clone();
}

fn diagnostic_data_to_i32(data: DiagnosticData, bit_count: usize) -> i32 {
    let mut result: i32 = 0;

    for i in 0..bit_count {
        result += (data[i] as i32) << (bit_count - i - 1);
    }

    return result;
}