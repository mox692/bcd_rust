#![feature(core_intrinsics)]

use std::{borrow::Borrow, u64,ops};

pub struct BCD(u64);

impl BCD {
    // TODO impl
    pub fn new<S: Borrow<str>>(input:S) -> Self {
        BCD(0)
    }
    // TODO impl
    fn is_compression_format(&self) -> bool {
        false
    }
}

impl ops::Add<BCD> for BCD {
    type Output = BCD;
    fn add(self, _rhs: BCD) -> BCD {
        // TODO impl
        BCD(self.0 + _rhs.0)
    }
}
impl ops::Sub<BCD> for BCD {
    type Output = BCD;
    fn sub(self, _rhs: BCD) -> BCD {
        // TODO impl
        BCD(self.0 - _rhs.0)
    }
}
impl ops::Mul<BCD> for BCD {
    type Output = BCD;
    fn mul(self, _rhs: BCD) -> BCD {
        // TODO impl
        BCD(self.0 * _rhs.0)
    }
}
impl ops::Div<BCD> for BCD {
    type Output = BCD;
    fn div(self, _rhs: BCD) -> BCD {
        // TODO impl
        BCD(self.0 / _rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use std::intrinsics::size_of;
    use super::*;
    #[test]
    fn t()  {
    }
}
