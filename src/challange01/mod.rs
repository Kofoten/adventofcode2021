use crate::core_challange::{Challange, ChallangeInput};

mod input;
mod part1;
mod part2;

const NAME: &str = "challange01";

pub struct Challange01 {}

impl Challange for Challange01 {
    fn run_part_1(&self, filename: String) -> String {
        part1::run(input::ChallangeInput01::read(filename, NAME))
    }

    fn run_part_2(&self, filename: String) -> String {
        part2::run(input::ChallangeInput01::read(filename, NAME))
    }
}