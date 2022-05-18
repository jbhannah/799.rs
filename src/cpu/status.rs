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
    pub fn set_negative(&mut self, result: u8) {
        *self = if result & 0b1000_0000 != 0 {
            self.or(Status::Negative)
        } else {
            self.and(Status::Negative.not())
        }
    }

    pub fn set_zero(&mut self, result: u8) {
        *self = if result == 0 {
            self.or(Status::Zero)
        } else {
            self.and(Status::Zero.not())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
