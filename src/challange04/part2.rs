use crate::challange04::input::ChallangeInput04;

pub fn run(mut input: ChallangeInput04) -> String {
    let mut answer: i32 = -1;

    for draw in input.draws {
        let mut boards_to_remove: Vec<usize> = Vec::new();

        for i in 0..input.boards.len() {
            if input.boards[i].check_number(draw) {
                if input.boards.len() == 1 {
                    answer = (input.boards[0].get_unmarked_sum() * draw) as i32;
                    break;
                }
                boards_to_remove.push(i);
            }
        }

        if answer != -1 {
            break;
        }

        boards_to_remove.sort_by(|a, b| b.cmp(a));
        for index in boards_to_remove {
            input.boards.remove(index);
        }
    }

    answer.to_string()
}