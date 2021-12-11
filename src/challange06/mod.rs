use crate::core_challange::{Challange, ChallangeInput};

pub mod util;

mod input;
mod part1;
mod part2;

const NAME: &str = "challange06";
const DAYS_PART_ONE: usize = 80;
const DAYS_PART_TWO: usize = 256;

pub struct Challange06 {}

impl Challange for Challange06 {
    fn run_part_1(&self, filename: &str) -> String {
        let mut input = input::ChallangeInput06::read(filename, NAME);
        input.days = DAYS_PART_ONE;
        part1::run(input)
    }

    fn run_part_2(&self, filename: &str) -> String {
        let mut input = input::ChallangeInput06::read(filename, NAME);
        input.days = DAYS_PART_TWO;
        part2::run(input)
    }
}