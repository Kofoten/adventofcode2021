use super::{LengthType, PacketType};

pub struct Packet {
    pub version: u8,
    pub packet_type: PacketType,
    pub length_type: LengthType,
    pub length: usize,
    pub value: u32,
    pub inner: Vec<Packet>,
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            version: 0,
            packet_type: PacketType::Unknown,
            length_type: LengthType::None,
            length: 0,
            value: 0,
            inner: Vec::new(),
        }
    }
}
