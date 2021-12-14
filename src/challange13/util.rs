use crate::challange13::data::{Dot, FoldAxis, FoldInstruction};

pub fn fold(dots: &mut Vec<Dot>, instruction: FoldInstruction) {
    let mut modified_dots: Vec<Dot> = Vec::new();
    let mut covered_dots: Vec<usize> = Vec::new();

    for i in 0..dots.len() {
        let mut dot = dots[i];

        match instruction.axis {
            FoldAxis::X => {
                if dot.x > instruction.index {
                    dot.x -= (dot.x - instruction.index) * 2;
                    modified_dots.push(dot);
                    covered_dots.push(i);
                }
            }
            FoldAxis::Y => {
                if dot.y > instruction.index {
                    dot.y -= (dot.y - instruction.index) * 2;
                    modified_dots.push(dot);
                    covered_dots.push(i);
                }
            }
        };
    }

    for dot in modified_dots {
        for i in 0..dots.len() {
            if dots[i] == dot {
                covered_dots.push(i);
            }
        }

        dots.push(dot);
    }

    covered_dots.sort_by(|a, b| b.cmp(a));
    for merged_dot in covered_dots {
        dots.remove(merged_dot);
    }
}
