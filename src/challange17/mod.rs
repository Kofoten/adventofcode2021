use crate::core_challange::{Challange, ChallangeInput};

pub mod data;
pub mod util;

mod input;
mod part1;
mod part2;

const NAME: &str = "challange17";

pub struct Challange17 {}

impl Challange for Challange17 {
    fn run_part_1(&self, filename: &str) -> String {
        part1::run(input::ChallangeInput17::read(filename, NAME))
    }

    fn run_part_2(&self, filename: &str) -> String {
        part2::run(input::ChallangeInput17::read(filename, NAME))
    }
}
