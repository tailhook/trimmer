/// An internal representation of something that can be compared
///
/// Only numbers and strings can be compared for now.
///
/// Use `into()` conversion to make the value.
#[derive(Debug)]  // TODO(tailhook) make normal debug
pub struct Comparable<'a>(ComparableInner<'a>);

#[derive(Debug)]  // TODO(tailhook) make normal debug
enum ComparableInner<'a> {
    I64(i64),
    U64(u64),
    F64(f64),
    Str(&'a str),
    String(String),
}

impl<'a> From<i8> for Comparable<'a> {
    fn from(x: i8) -> Comparable<'a> {
        Comparable(ComparableInner::I64(x as i64))
    }
}

impl<'a> From<i16> for Comparable<'a> {
    fn from(x: i16) -> Comparable<'a> {
        Comparable(ComparableInner::I64(x as i64))
    }
}

impl<'a> From<i32> for Comparable<'a> {
    fn from(x: i32) -> Comparable<'a> {
        Comparable(ComparableInner::I64(x as i64))
    }
}

impl<'a> From<i64> for Comparable<'a> {
    fn from(x: i64) -> Comparable<'a> {
        Comparable(ComparableInner::I64(x))
    }
}

impl<'a> From<u8> for Comparable<'a> {
    fn from(x: u8) -> Comparable<'a> {
        Comparable(ComparableInner::U64(x as u64))
    }
}

impl<'a> From<u16> for Comparable<'a> {
    fn from(x: u16) -> Comparable<'a> {
        Comparable(ComparableInner::U64(x as u64))
    }
}

impl<'a> From<u32> for Comparable<'a> {
    fn from(x: u32) -> Comparable<'a> {
        Comparable(ComparableInner::U64(x as u64))
    }
}

impl<'a> From<u64> for Comparable<'a> {
    fn from(x: u64) -> Comparable<'a> {
        Comparable(ComparableInner::U64(x))
    }
}

impl<'a> From<f64> for Comparable<'a> {
    fn from(x: f64) -> Comparable<'a> {
        Comparable(ComparableInner::F64(x))
    }
}

impl<'a> From<f32> for Comparable<'a> {
    fn from(x: f32) -> Comparable<'a> {
        Comparable(ComparableInner::F64(x as f64))
    }
}

impl<'a> From<&'a str> for Comparable<'a> {
    fn from(x: &'a str) -> Comparable<'a> {
        Comparable(ComparableInner::Str(x))
    }
}

impl<'a> From<String> for Comparable<'a> {
    fn from(x: String) -> Comparable<'a> {
        Comparable(ComparableInner::String(x))
    }
}
