use num_enum::TryFromPrimitive;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum PacketType {
    Unknown = 0,
    LiteralValue = 4,
}
