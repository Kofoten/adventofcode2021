use num_enum::TryFromPrimitive;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum PacketType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    LiteralValue = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
    None = 8,
}
