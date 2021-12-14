use crate::core_challange::{Challange, ChallangeInput};

mod input;
mod part1;
mod part2;

const NAME: &str = "challange21";

pub struct Challange21 {}

impl Challange for Challange21 {
    fn run_part_1(&self, filename: &str) -> String {
        part1::run(input::ChallangeInput21::read(filename, NAME))
    }

    fn run_part_2(&self, filename: &str) -> String {
        part2::run(input::ChallangeInput21::read(filename, NAME))
    }
}