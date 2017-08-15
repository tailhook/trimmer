/// An internal representation of something that can be compared
///
/// Only numbers and strings can be compared for now.
///
/// Use `into()` conversion to make the value.
pub struct Comparable<'a>(ComparableInner<'a>);

enum ComparableInner<'a> {
    I64(i64),
    U64(u64),
    F64(f64),
    Str(&'a str),
    String(String),
}

impl<'a> From<i64> for Comparable<'a> {
    fn from(x: i64) -> Comparable<'a> {
        Comparable(ComparableInner::I64(x))
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
