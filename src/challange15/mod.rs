use crate::core_challange::{Challange, ChallangeInput};

pub mod node;
pub mod util;

mod input;
mod part1;
mod part2;

const NAME: &str = "challange15";

pub struct Challange15 {}

impl Challange for Challange15 {
    fn run_part_1(&self, filename: &str) -> String {
        part1::run(input::ChallangeInput15::read(filename, NAME))
    }

    fn run_part_2(&self, filename: &str) -> String {
        part2::run(input::ChallangeInput15::read(filename, NAME))
    }
}
