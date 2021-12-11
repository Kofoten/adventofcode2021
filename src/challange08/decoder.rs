pub struct Decoder {
    top: char,
    center_left: Vec<char>,
    right: Vec<char>,
    bottm_left: Vec<char>
}

impl Decoder {
    pub fn new(one: String, four: String, seven: String, eight: String) -> Self {
        let right: Vec<char> = one.chars().collect();
        let center_left: Vec<char> = four.chars().filter(|c| !right.contains(c)).collect();
        let top: char = seven.chars().filter(|c| !right.contains(c)).last().unwrap();
        let bottm_left: Vec<char> = eight.chars().filter(|c| !right.contains(c) && !center_left.contains(c) && top != *c).collect();

        Decoder { top, center_left, right, bottm_left }
    }

    pub fn decode(&self, value: &String) -> u32 {
        let chars: Vec<char> = value.chars().collect();
        let letter_count = chars.len();
        if letter_count == 2 {
            1
        } else if letter_count == 3 {
            7
        } else if letter_count == 4 {
            4
        } else if letter_count == 7 {
            8
        } else {
            let top_match = chars.contains(&self.top);

            let mut center_left_matches = 0;
            for letter in &self.center_left {
                if chars.contains(letter) {
                    center_left_matches += 1;
                }
            }

            let mut right_matches = 0;
            for letter in &self.right {
                if chars.contains(&letter) {
                    right_matches += 1;
                }
            }

            let mut bottm_left_matches = 0;
            for letter in &self.bottm_left {
                if chars.contains(&letter) {
                    bottm_left_matches += 1;
                }
            }

            if top_match && center_left_matches == 1 && bottm_left_matches == 2 && right_matches == 2 {
                0
            } else if top_match && center_left_matches == 1 && bottm_left_matches == 2 && right_matches == 1 {
                2
            } else if top_match && center_left_matches == 1 && bottm_left_matches == 1 && right_matches == 2 {
                3
            } else if top_match && center_left_matches == 2 && bottm_left_matches == 1 && right_matches == 1 {
                5
            } else if top_match && center_left_matches == 2 && bottm_left_matches == 2 && right_matches == 1 {
                6
            } else if top_match && center_left_matches == 2 && bottm_left_matches == 1 && right_matches == 2 {
                9
            } else {
                panic!("Invalid pattern");
            }
        }
    }
}