use crate::core_challange::{Challange, ChallangeInput};

pub mod util;

mod input;
mod part1;
mod part2;

const NAME: &str = "challange11";

pub struct Challange11 {}

impl Challange for Challange11 {
    fn run_part_1(&self, filename: &str) -> String {
        part1::run(input::ChallangeInput11::read(filename, NAME))
    }

    fn run_part_2(&self, filename: &str) -> String {
        part2::run(input::ChallangeInput11::read(filename, NAME))
    }
}
