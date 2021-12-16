use super::data::{LengthType, Packet, PacketType};

#[derive(PartialEq, Eq)]
enum DecoderState {
    Version,
    Type,
    LengthType,
    Length,
    Values,
    End,
}

pub struct Decoder<'a> {
    position: usize,
    bits: &'a Vec<bool>,
}

impl<'a> Decoder<'a> {
    pub fn from(bits: &'a Vec<bool>) -> Self {
        Decoder { position: 0, bits }
    }

    pub fn parse_packet(&mut self) -> Option<Packet> {
        let mut packet: Packet = Packet::new();
        let mut state: DecoderState = DecoderState::Version;
        let mut current_bits: Vec<bool> = Vec::new();
        let mut length_limit: usize = 0;
        let mut parse_inner_stop: usize;

        while state != DecoderState::End {
            current_bits.push(self.bits[self.position]);

            match state {
                DecoderState::Version => {
                    if current_bits.len() == 3 {
                        packet.version = bits_to_u8(&current_bits);
                        current_bits.clear();
                        state = DecoderState::Type;
                    }
                }
                DecoderState::Type => {
                    if let Some(packet_type) = self.decode_packet_type(&mut current_bits) {
                        current_bits.clear();
                        packet.packet_type = packet_type;
                        if packet.packet_type == PacketType::LiteralValue {
                            state = DecoderState::Values;
                        } else {
                            state = DecoderState::LengthType;
                        }
                    }
                }
                DecoderState::LengthType => {
                    if current_bits[0] {
                        packet.length_type = LengthType::Commands;
                        length_limit = 11;
                    } else {
                        packet.length_type = LengthType::BitCount;
                        length_limit = 15;
                    }
                    current_bits.clear();
                    state = DecoderState::Length;
                }
                DecoderState::Length => {
                    if current_bits.len() == length_limit {
                        packet.length = bits_to_u64(&current_bits) as usize;
                        current_bits.clear();
                        state = DecoderState::Values;
                    }
                }
                DecoderState::Values => {
                    if packet.packet_type == PacketType::LiteralValue {
                        if current_bits.len() == 5 {
                            if !current_bits[0] {
                                state = DecoderState::End;
                            }
                            current_bits.remove(0);
                            packet.value <<= 4;
                            packet.value += bits_to_u64(&current_bits);
                            current_bits.clear();
                        }
                    } else if packet.length_type == LengthType::BitCount {
                        parse_inner_stop = self.position + packet.length;
                        while parse_inner_stop > self.position {
                            if let Some(inner_packet) = self.parse_packet() {
                                packet.inner.push(inner_packet);
                            }
                        }
                        self.position -= 1;
                        state = DecoderState::End;
                    } else if packet.length_type == LengthType::Commands {
                        while packet.inner.len() < packet.length {
                            if let Some(inner_packet) = self.parse_packet() {
                                packet.inner.push(inner_packet);
                            }
                        }
                        self.position -= 1;
                        state = DecoderState::End;
                    }
                }
                _ => return None,
            }

            self.position += 1;
        }

        Some(packet)
    }

    fn decode_packet_type(&mut self, current_bits: &mut Vec<bool>) -> Option<PacketType> {
        if current_bits.len() == 3 {
            if let Ok(packet_type) = PacketType::try_from(bits_to_u8(&current_bits)) {
                Some(packet_type)
            } else {
                Some(PacketType::None)
            }
        } else {
            None
        }
    }
}

fn bits_to_u8(bits: &Vec<bool>) -> u8 {
    let mut result: u8 = bits[0] as u8;
    for i in 1..bits.len() {
        result <<= 1;
        result += bits[i] as u8;
    }
    result
}

fn bits_to_u64(bits: &Vec<bool>) -> u64 {
    let mut result: u64 = bits[0] as u64;
    for i in 1..bits.len() {
        result <<= 1;
        result += bits[i] as u64;
    }
    result
}
