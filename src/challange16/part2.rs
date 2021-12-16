use super::decoder::Decoder;
use crate::challange16::input::ChallangeInput16;

pub fn run(input: ChallangeInput16) -> String {
    if let Some(packet) = Decoder::from(&input.bits).parse_packet() {
        packet.get_result().to_string()
    } else {
        String::from("skit")
    }
}
