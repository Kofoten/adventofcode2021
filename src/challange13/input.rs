use crate::challange13::data::{Dot, FoldInstruction};
use crate::core_challange::ChallangeInput;

pub struct ChallangeInput13 {
    instruction_section: bool,
    pub fold_instructions: Vec<FoldInstruction>,
    pub dots: Vec<Dot>,
}

impl Default for ChallangeInput13 {
    fn default() -> Self {
        ChallangeInput13 {
            instruction_section: false,
            fold_instructions: Vec::new(),
            dots: Vec::new(),
        }
    }
}

impl ChallangeInput for ChallangeInput13 {
    fn parse_line(&mut self, line: String) {
        if line.is_empty() {
            self.instruction_section = true;
        } else if self.instruction_section {
            self.fold_instructions
                .push(FoldInstruction::from(&line).ok().unwrap());
        } else {
            self.dots.push(Dot::from(&line))
        }
    }
}
