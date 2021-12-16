use crate::challange16::decoder::Decoder;
use crate::challange16::input::ChallangeInput16;

pub fn run(input: ChallangeInput16) -> String {
    if let Some(packet) = Decoder::from(&input.bits).parse_packet() {
        println!("{}", packet.version)
    }
    "y ".to_string()
}
