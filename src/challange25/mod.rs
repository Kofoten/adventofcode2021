use crate::core_challange::{Challange, ChallangeInput};

mod input;
mod part1;
mod part2;

const NAME: &str = "challange25";

pub struct Challange25 {}

impl Challange for Challange25 {
    fn run_part_1(&self, filename: &str) -> String {
        part1::run(input::ChallangeInput25::read(filename, NAME))
    }

    fn run_part_2(&self, filename: &str) -> String {
        part2::run(input::ChallangeInput25::read(filename, NAME))
    }
}