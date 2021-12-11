use crate::challange06::input::ChallangeInput06;

pub fn calculate_lanternfish_count(input: ChallangeInput06) -> u64 {
    let mut lanternfishes: Vec<u64> = vec![0; 9];

    for lanternfish in input.lanternfishes {
        lanternfishes[lanternfish] += 1;
    }

    for _ in 0..input.days {
        lanternfishes.rotate_left(1);
        lanternfishes[6] += lanternfishes[8];
    }

    lanternfishes.iter().fold(0u64, |sum, val| sum + val)
}