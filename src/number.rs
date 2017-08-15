/// An internal representation of a number that may be integer of real
///
/// Use `into()` conversion to make the value.
pub struct Number(NumberInner);

enum NumberInner {
    I64(i64),
    U64(u64),
    F64(f64),
}

impl From<i64> for Number {
    fn from(x: i64) -> Number {
        Number(NumberInner::I64(x))
    }
}

impl From<u64> for Number {
    fn from(x: u64) -> Number {
        Number(NumberInner::U64(x))
    }
}

impl From<f64> for Number {
    fn from(x: f64) -> Number {
        Number(NumberInner::F64(x))
    }
}
