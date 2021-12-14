use crate::challange13::data::Dot;
use crate::challange13::input::ChallangeInput13;
use crate::challange13::util;
use std::cmp;

pub fn run(mut input: ChallangeInput13) -> String {
    for instruction in input.fold_instructions {
        util::fold(&mut input.dots, instruction)
    }

    let max: Dot = input.dots.iter().fold(Dot::origin(), |acc, dot| Dot {
        x: cmp::max(acc.x, dot.x),
        y: cmp::max(acc.y, dot.y),
    });

    let mut characters: Vec<Vec<char>> = vec![vec![' '; max.x as usize + 1]; max.y as usize + 1];
    for dot in input.dots {
        characters[dot.y as usize][dot.x as usize] = '#';
    }

    characters
        .iter()
        .map(|row| String::from_iter(row))
        .collect::<Vec<String>>()
        .join("\n")
}
