use crate::core_challange::ChallangeInput;

pub type DiagnosticData = Vec<bool>;

pub struct ChallangeInput03 {
    is_first_line: bool,
    pub bit_count: usize,
    pub values: Vec<DiagnosticData>
}

impl Default for ChallangeInput03 {
    fn default() -> Self { ChallangeInput03 {
        is_first_line: true,
        bit_count: 0,
        values: Vec::new()
    } }
}

impl ChallangeInput for ChallangeInput03 {
    fn parse_line(&mut self, line: String) {
        let mut bits: DiagnosticData = Vec::new();
        let bytes = line.as_bytes();

        if self.is_first_line {
            self.bit_count = bytes.len();
        }

        for byte in bytes {
            bits.push(*byte - 48 > 0)
        }
        self.values.push(bits);
    }
}