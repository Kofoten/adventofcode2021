use crate::challange10::input::ChallangeInput10;
use crate::challange10::util;

struct ParseError {}

pub fn run(input: ChallangeInput10) -> String {
    let mut scores: Vec<u64> = Vec::new();

    for line in input.lines {
        match get_missing_end(line) {
            Ok(ending) => scores.push(calculate_score(ending)),
            Err(_) => (),
        };
    }

    scores.sort();
    scores[scores.len() / 2].to_string()
}

fn get_missing_end(line: Vec<char>) -> Result<Vec<char>, ParseError> {
    let mut open_chunks: Vec<char> = Vec::new();
    for character in line.clone() {
        if util::is_opening(character) {
            open_chunks.push(character);
        } else if util::is_closing(character) {
            if open_chunks.len() == 0 {
                return Err(ParseError {});
            }

            let index = open_chunks.len() - 1;
            if util::is_chunk_type(open_chunks[index], character) {
                open_chunks.remove(index);
            } else {
                return Err(ParseError {});
            }
        }
    }

    let length = open_chunks.len();
    let mut ending: Vec<char> = Vec::new();
    for i in 0..length {
        ending.push(match open_chunks[length - i - 1] {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => return Err(ParseError {}),
        });
    }

    Ok(ending)
}

fn calculate_score(ending: Vec<char>) -> u64 {
    let mut score: u64 = 0;

    for character in ending {
        let character_value = match character {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        };

        score = score * 5 + character_value;
    }

    score
}
