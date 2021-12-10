use crate::core_challange::ChallangeInput;

pub struct SubmarineCommand {
    pub direction: String,
    pub step: u32
}

pub struct ChallangeInput02 {
    pub commands: Vec<SubmarineCommand>
}

impl Default for ChallangeInput02 {
    fn default() -> Self { ChallangeInput02 { commands: Vec::new() } }
}

impl ChallangeInput for ChallangeInput02 {
    fn parse_line(&mut self, line: String) {
        let value = line.split(" ").collect::<Vec<&str>>();
        self.commands.push(SubmarineCommand {
            direction: value[0].to_string(),
            step: value[1].parse::<u32>().unwrap()
        });
    }
}