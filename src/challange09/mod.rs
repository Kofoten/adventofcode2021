use crate::core_challange::{Challange, ChallangeInput};

pub mod util;
pub mod point;

mod input;
mod part1;
mod part2;

const NAME: &str = "challange09";

pub struct Challange09 {}

impl Challange for Challange09 {
    fn run_part_1(&self, filename: &str) -> String {
        part1::run(input::ChallangeInput09::read(filename, NAME))
    }

    fn run_part_2(&self, filename: &str) -> String {
        part2::run(input::ChallangeInput09::read(filename, NAME))
    }
}