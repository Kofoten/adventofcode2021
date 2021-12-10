use crate::challange04::input;

pub fn run(input: input::ChallangeInput04) -> String {
    let mut answer: i32 = -1;
    let mut boards = input.boards.clone();

    for draw in input.draws {
        for i in 0..boards.len() {
            if boards[i].check_number(draw) {
                answer = (boards[i].get_unmarked_sum() * draw) as i32;
                break;
            }
        }

        if answer > -1 {
            break;
        }
    }

    answer.to_string()
}