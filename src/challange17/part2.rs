use crate::challange17::input::ChallangeInput17;

pub fn run(input: ChallangeInput17) -> String {
    format!(
        "x={}..{}, y={}..{}",
        input.target_area.x,
        input.target_area.x + input.target_area.width,
        input.target_area.y,
        input.target_area.y + input.target_area.height
    )
}
