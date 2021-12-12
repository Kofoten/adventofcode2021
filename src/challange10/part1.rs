use crate::challange10::input::ChallangeInput10;
use crate::challange10::util;

pub fn run(input: ChallangeInput10) -> String {
    let mut score: u32 = 0;

    for line in input.lines {
        match analyze_line(line) {
            Some(error_score) => score += error_score,
            None => (),
        };
    }

    score.to_string()
}

fn analyze_line(line: Vec<char>) -> Option<u32> {
    let mut chunks: Vec<char> = Vec::new();
    for character in line {
        if util::is_opening(character) {
            chunks.push(character);
        } else if util::is_closing(character) {
            if chunks.len() == 0 {
                return Some(get_error_score(character));
            }

            let index = chunks.len() - 1;
            if util::is_chunk_type(chunks[index], character) {
                chunks.remove(index);
            } else {
                return Some(get_error_score(character));
            }
        }
    }
    None
}

fn get_error_score(invalid_character: char) -> u32 {
    match invalid_character {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}
