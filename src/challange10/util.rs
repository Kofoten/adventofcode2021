pub fn is_closing(character: char) -> bool {
    match character {
        ')' => true,
        ']' => true,
        '}' => true,
        '>' => true,
        _ => false,
    }
}

pub fn is_opening(character: char) -> bool {
    match character {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false,
    }
}

pub fn is_chunk_type(chunk_type: char, character: char) -> bool {
    match chunk_type {
        '(' => character == '(' || character == ')',
        '[' => character == '[' || character == ']',
        '{' => character == '{' || character == '}',
        '<' => character == '<' || character == '>',
        _ => panic!("Invalid chunk type"),
    }
}
