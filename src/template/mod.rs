use crate::core_challange::{Challange, ChallangeInput};

mod input;
mod part1;
mod part2;

const NAME: &str = "template";

pub struct ChallangeTemplate {}

impl Challange for ChallangeTemplate {
    fn run_part_1(&self, filename: &str) -> String {
        part1::run(input::ChallangeInputTemplate::read(filename, NAME))
    }

    fn run_part_2(&self, filename: &str) -> String {
        part2::run(input::ChallangeInputTemplate::read(filename, NAME))
    }
}