use crate::core_challange::{Challange, ChallangeInput};

pub mod util;

mod input;
mod part1;
mod part2;

const NAME: &str = "challange10";

pub struct Challange10 {}

impl Challange for Challange10 {
    fn run_part_1(&self, filename: &str) -> String {
        part1::run(input::ChallangeInput10::read(filename, NAME))
    }

    fn run_part_2(&self, filename: &str) -> String {
        part2::run(input::ChallangeInput10::read(filename, NAME))
    }
}