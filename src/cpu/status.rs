use bitmask_enum::bitmask;

#[bitmask(u8)]
pub enum Status {
    Carry,
    Zero,
    InterruptDisable,
    Decimal,
    Break,
    Break2,
    Overflow,
    Negative,
}

impl Default for Status {
    fn default() -> Self {
        Status::InterruptDisable | Status::Break | Status::Break2
    }
}

impl Status {
    pub fn set(&mut self, flag: Self, value: bool) {
        *self = if value {
            self.or(flag)
        } else {
            self.and(flag.not())
        };
    }

    pub fn set_carry(&mut self, result: u16) {
        self.set(Status::Carry, result > 0xff);
    }

    pub fn set_negative(&mut self, result: u8) {
        self.set(Status::Negative, result & 0b1000_0000 != 0);
    }

    pub fn set_overflow(&mut self, result: bool) {
        self.set(Status::Overflow, result);
    }

    pub fn set_zero(&mut self, result: u8) {
        self.set(Status::Zero, result == 0);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_carry_off_off() {
        let mut status = Status::default();
        status.set_carry(0x0fe);
        assert!(!status.contains(Status::Carry));
    }

    #[test]
    fn test_set_carry_off_on() {
        let mut status = Status::default();
        status.set_carry(0x100);
        assert!(status.contains(Status::Carry));
    }

    #[test]
    fn test_set_carry_on_off() {
        let mut status = Status::default() | Status::Carry;
        status.set_carry(0x0fe);
        assert!(!status.contains(Status::Carry));
    }

    #[test]
    fn test_set_carry_on_on() {
        let mut status = Status::default() | Status::Carry;
        status.set_carry(0x100);
        assert!(status.contains(Status::Carry));
    }

    #[test]
    fn test_set_negative_off_off() {
        let mut status = Status::default();
        status.set_negative(0b0000_0001);
        assert!(!status.contains(Status::Negative));
    }

    #[test]
    fn test_set_negative_off_on() {
        let mut status = Status::default();
        status.set_negative(0b1000_0000);
        assert!(status.contains(Status::Negative));
    }

    #[test]
    fn test_set_negative_on_off() {
        let mut status = Status::default() | Status::Negative;
        status.set_negative(0b0000_0001);
        assert!(!status.contains(Status::Negative));
    }

    #[test]
    fn test_set_negative_on_on() {
        let mut status = Status::default() | Status::Negative;
        status.set_negative(0b1000_0000);
        assert!(status.contains(Status::Negative));
    }

    #[test]
    fn test_set_overflow_off_off() {
        let mut status = Status::default();
        status.set_overflow(false);
        assert!(!status.contains(Status::Overflow));
    }

    #[test]
    fn test_set_overflow_off_on() {
        let mut status = Status::default();
        status.set_overflow(true);
        assert!(status.contains(Status::Overflow));
    }

    #[test]
    fn test_set_overflow_on_off() {
        let mut status = Status::default() | Status::Overflow;
        status.set_overflow(false);
        assert!(!status.contains(Status::Overflow));
    }

    #[test]
    fn test_set_overflow_on_on() {
        let mut status = Status::default() | Status::Overflow;
        status.set_overflow(true);
        assert!(status.contains(Status::Overflow));
    }

    #[test]
    fn test_set_zero_off_off() {
        let mut status = Status::default();
        status.set_zero(1);
        assert!(!status.contains(Status::Zero));
    }

    #[test]
    fn test_set_zero_off_on() {
        let mut status = Status::default();
        status.set_zero(0);
        assert!(status.contains(Status::Zero));
    }

    #[test]
    fn test_set_zero_on_off() {
        let mut status = Status::default() | Status::Zero;
        status.set_zero(1);
        assert!(!status.contains(Status::Zero));
    }

    #[test]
    fn test_set_zero_on_on() {
        let mut status = Status::default() | Status::Zero;
        status.set_zero(0);
        assert!(status.contains(Status::Zero));
    }
}
