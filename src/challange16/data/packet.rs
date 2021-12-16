use super::{LengthType, PacketType};

pub struct Packet {
    pub version: u8,
    pub packet_type: PacketType,
    pub length_type: LengthType,
    pub length: usize,
    pub value: u64,
    pub inner: Vec<Packet>,
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            version: 0,
            packet_type: PacketType::None,
            length_type: LengthType::None,
            length: 0,
            value: 0,
            inner: Vec::new(),
        }
    }

    pub fn get_result(&self) -> u64 {
        match self.packet_type {
            PacketType::Sum => self
                .inner
                .iter()
                .fold(0u64, |acc, packet| acc + packet.get_result()),
            PacketType::Product => self
                .inner
                .iter()
                .fold(1u64, |acc, packet| acc * packet.get_result()),
            PacketType::Minimum => self
                .inner
                .iter()
                .map(|packet| packet.get_result())
                .min()
                .unwrap_or(0),
            PacketType::Maximum => self
                .inner
                .iter()
                .map(|packet| packet.get_result())
                .max()
                .unwrap_or(0),
            PacketType::LiteralValue => self.value,
            PacketType::GreaterThan => {
                (self.inner[0].get_result() > self.inner[1].get_result()) as u64
            }
            PacketType::LessThan => {
                (self.inner[0].get_result() < self.inner[1].get_result()) as u64
            }
            PacketType::EqualTo => {
                (self.inner[0].get_result() == self.inner[1].get_result()) as u64
            }
            _ => 0,
        }
    }
}
