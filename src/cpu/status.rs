use bitmask_enum::bitmask;

#[bitmask(u8)]
pub enum Status {
    Carry,
    Zero,
    InterruptDisable,
    Decimal,
    Break,
    Empty,
    Overflow,
    Negative,
}

impl Default for Status {
    fn default() -> Self {
        Status::none()
    }
}
