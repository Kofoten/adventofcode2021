use crate::core_challange::{Challange, ChallangeInput};

pub mod decoder;

mod input;
mod part1;
mod part2;

const NAME: &str = "challange08";

pub struct Challange08 {}

impl Challange for Challange08 {
    fn run_part_1(&self, filename: &str) -> String {
        part1::run(input::ChallangeInput08::read(filename, NAME))
    }

    fn run_part_2(&self, filename: &str) -> String {
        part2::run(input::ChallangeInput08::read(filename, NAME))
    }
}