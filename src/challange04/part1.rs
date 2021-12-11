use crate::challange04::input::ChallangeInput04;

pub fn run(mut input: ChallangeInput04) -> String {
    let mut answer: i32 = -1;

    for draw in input.draws {
        for i in 0..input.boards.len() {
            if input.boards[i].check_number(draw) {
                answer = (input.boards[i].get_unmarked_sum() * draw) as i32;
                break;
            }
        }

        if answer > -1 {
            break;
        }
    }

    answer.to_string()
}