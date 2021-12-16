use crate::challange16::decoder::Decoder;
use crate::challange16::input::ChallangeInput16;

use super::data::{Packet, PacketType};

pub fn run(input: ChallangeInput16) -> String {
    if let Some(packet) = Decoder::from(&input.bits).parse_packet() {
        get_version_sum(&packet).to_string()
    } else {
        String::from("skit")
    }
}

fn get_version_sum(packet: &Packet) -> u32 {
    let mut sum: u32 = packet.version as u32;

    if packet.packet_type != PacketType::LiteralValue {
        for inner_packet in &packet.inner {
            sum += get_version_sum(inner_packet);
        }
    }

    sum
}
