use std::collections::HashMap;

pub fn calculate_polymer_strength(
    steps: usize,
    template: &Vec<char>,
    rules: &HashMap<String, char>,
) -> usize {
    let last: char = *template.iter().last().unwrap();
    let mut combinations: HashMap<String, usize> = HashMap::new();

    for i in 1..template.len() {
        let key = format!("{}{}", template[i - 1], template[i]);
        *combinations.entry(key).or_insert(0) += 1;
    }

    for _step in 0..steps {
        let mut new_combinations: HashMap<String, usize> = HashMap::new();

        for (key, count) in &combinations {
            if let Some(character) = rules.get(key) {
                if let Ok(new_key) = create_new_key(key, *character, 0) {
                    *new_combinations.entry(new_key).or_insert(0) += *count;
                }

                if let Ok(new_key) = create_new_key(key, *character, 1) {
                    *new_combinations.entry(new_key).or_insert(0) += *count;
                }
            }
        }

        combinations = new_combinations;
    }

    let mut counts: HashMap<char, usize> = HashMap::from([(last, 1)]);
    for (key, count) in combinations {
        if let Some(character) = key.chars().nth(0) {
            *counts.entry(character).or_insert(0) += count;
        }
    }

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn create_new_key(key: &String, character: char, index: usize) -> Result<String, ()> {
    if let Some(val) = key.chars().nth(index) {
        Ok(if index == 0 {
            format!("{}{}", val, character)
        } else {
            format!("{}{}", character, val)
        })
    } else {
        Err(())
    }
}