use std::cmp;
use std::collections::HashMap;

pub fn calculate_polymer_strength(
    steps: usize,
    template: &mut Vec<char>,
    rules: &HashMap<String, char>,
) -> usize {
    for _step in 0..steps {
        let mut new_template: Vec<char> = vec![template[0]];

        for i in 1..template.len() {
            let second = template[i];
            let key = format!("{}{}", template[i - 1], second);
            let element = rules.get(&key).unwrap();
            new_template.push(*element);
            new_template.push(second);
        }

        *template = new_template;
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for i in 0..template.len() {
        *counts.entry(template[i]).or_insert(0) += 1;
    }

    let mut max: usize = 0;
    let mut min: usize = template.len();
    for count in counts.values() {
        max = cmp::max(max, *count);
        min = cmp::min(min, *count);
    }

    max - min
}
