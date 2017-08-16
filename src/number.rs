/// An internal representation of a number that may be integer of real
///
/// Use `into()` conversion to make the value.
pub struct Number(NumberInner);

enum NumberInner {
    I64(i64),
    U64(u64),
    F64(f64),
}

impl From<i8> for Number {
    fn from(x: i8) -> Number {
        Number(NumberInner::I64(x as i64))
    }
}

impl From<i16> for Number {
    fn from(x: i16) -> Number {
        Number(NumberInner::I64(x as i64))
    }
}

impl From<i32> for Number {
    fn from(x: i32) -> Number {
        Number(NumberInner::I64(x as i64))
    }
}

impl From<i64> for Number {
    fn from(x: i64) -> Number {
        Number(NumberInner::I64(x))
    }
}

impl From<u8> for Number {
    fn from(x: u8) -> Number {
        Number(NumberInner::U64(x as u64))
    }
}

impl From<u16> for Number {
    fn from(x: u16) -> Number {
        Number(NumberInner::U64(x as u64))
    }
}

impl From<u32> for Number {
    fn from(x: u32) -> Number {
        Number(NumberInner::U64(x as u64))
    }
}

impl From<u64> for Number {
    fn from(x: u64) -> Number {
        Number(NumberInner::U64(x))
    }
}

impl From<f32> for Number {
    fn from(x: f32) -> Number {
        Number(NumberInner::F64(x as f64))
    }
}

impl From<f64> for Number {
    fn from(x: f64) -> Number {
        Number(NumberInner::F64(x))
    }
}
