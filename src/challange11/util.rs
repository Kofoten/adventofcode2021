pub fn get_flashing_octopi(octopi: &mut Vec<u8>, width: usize) -> Vec<usize> {
    let mut charged: Vec<usize> = Vec::new();

    for i in 0..octopi.len() {
        octopi[i] += 1;
        if octopi[i] > 9 {
            charged.push(i);
        }
    }

    let mut index: usize = 0;
    while index < charged.len() {
        let neighbours: Vec<usize> = get_neighbour_indicies(octopi.len(), width, charged[index]);

        for neighbour in neighbours {
            if octopi[neighbour] > 9 {
                continue;
            }

            octopi[neighbour] += 1;
            if octopi[neighbour] > 9 {
                charged.push(neighbour);
            }
        }

        index += 1;
    }

    charged
}

pub fn get_neighbour_indicies(length: usize, width: usize, index: usize) -> Vec<usize> {
    let mut neighbours: Vec<usize> = Vec::new();

    let up_allowed: bool = index >= width;
    let down_allowed: bool = index + width < length;
    let left_allowed: bool = index % width > 0;
    let right_allowed: bool = index % width < width - 1;

    if up_allowed {
        neighbours.push(index - width);

        if left_allowed {
            neighbours.push(index - width - 1);
        }

        if right_allowed {
            neighbours.push(index - width + 1);
        }
    }

    if down_allowed {
        neighbours.push(index + width);

        if left_allowed {
            neighbours.push(index + width - 1);
        }

        if right_allowed {
            neighbours.push(index + width + 1);
        }
    }

    if left_allowed {
        neighbours.push(index - 1);
    }

    if right_allowed {
        neighbours.push(index + 1);
    }

    neighbours
}
