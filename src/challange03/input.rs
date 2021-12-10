use crate::core_challange::ChallangeInput;

const BIT_COUNT: usize = 12;

pub type DiagnosticData = Vec<bool>;

pub struct ChallangeInput03 {
    pub bit_count: usize,
    pub values: Vec<DiagnosticData>
}

impl Default for ChallangeInput03 {
    fn default() -> Self { ChallangeInput03 {
        values: Vec::new(),
        bit_count: BIT_COUNT
    } }
}

impl ChallangeInput for ChallangeInput03 {
    fn parse_line(&mut self, line: String) {
        let mut bits: DiagnosticData = Vec::new();
        for byte in line.as_bytes() {
            bits.push(*byte - 48 > 0)
        }
        self.values.push(bits);
    }
}