use crate::core_challange::{Challange, ChallangeInput};

pub mod board;

mod input;
mod part1;
mod part2;

const NAME: &str = "challange04";

pub struct Challange04 {}

impl Challange for Challange04 {
    fn run_part_1(&self, filename: &str) -> String {
        part1::run(input::ChallangeInput04::read(filename, NAME))
    }

    fn run_part_2(&self, filename: &str) -> String {
        part2::run(input::ChallangeInput04::read(filename, NAME))
    }
}