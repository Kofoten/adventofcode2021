use crate::core_challange::ChallangeInput;

pub struct InputData {
    pub signal_pattern: Vec<String>,
    pub output_value: Vec<String>
}

pub struct ChallangeInput08 {
    pub values: Vec<InputData>
}

impl Default for ChallangeInput08 {
    fn default() -> Self { ChallangeInput08 { values: Vec::new() } }
}

impl ChallangeInput for ChallangeInput08 {
    fn parse_line(&mut self, line: String) {
        let mut delimiter_encontered: bool = false;
        let mut data = InputData {
            signal_pattern: Vec::new(),
            output_value: Vec::new()
        };

        for value in line.split(" ") {
            if value == "|" {
                delimiter_encontered = true;
                continue;
            }

            let text: String = value.to_string();
            if delimiter_encontered {
                data.output_value.push(text);
            } else {
                data.signal_pattern.push(text);
            }
        }

        self.values.push(data);
    }
}