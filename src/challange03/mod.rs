use crate::core_challange::{Challange, ChallangeInput};

mod input;
mod part1;
mod part2;

const NAME: &str = "challange03";

pub struct Challange03 {}

impl Challange for Challange03 {
    fn run_part_1(&self, filename: &str) -> String {
        part1::run(input::ChallangeInput03::read(filename, NAME))
    }

    fn run_part_2(&self, filename: &str) -> String {
        part2::run(input::ChallangeInput03::read(filename, NAME))
    }
}