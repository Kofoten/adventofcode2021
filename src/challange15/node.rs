pub struct Node {
    pub risk_level: u32,
    pub previous: usize,
    pub path_risk: u32,
}

impl Node {
    pub fn from(risk_level: u32) -> Self {
        Node {
            risk_level,
            path_risk: u32::MAX,
            previous: 0,
        }
    }
}
