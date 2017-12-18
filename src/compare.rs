use grammar::CmpOperator;
use std::i64;


/// An internal representation of something that can be compared
///
/// Only numbers and strings can be compared for now.
///
/// Use `into()` conversion to make the value.
#[derive(Debug)]  // TODO(tailhook) make normal debug
pub struct Comparable<'a>(ComparableInner<'a>);

#[derive(Debug)]  // TODO(tailhook) make normal debug
enum ComparableInner<'a> {
    Bool(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    Str(&'a str),
    String(String),
}

impl<'a> From<bool> for Comparable<'a> {
    fn from(x: bool) -> Comparable<'a> {
        Comparable(ComparableInner::Bool(x))
    }
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

impl<'a> From<isize> for Comparable<'a> {
    fn from(x: isize) -> Comparable<'a> {
        Comparable(ComparableInner::I64(x as i64))
    }
}

impl<'a> From<usize> for Comparable<'a> {
    fn from(x: usize) -> Comparable<'a> {
        Comparable(ComparableInner::U64(x as u64))
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

fn _compare<T: PartialEq + PartialOrd>(a: &T, b: &T, oper: CmpOperator) -> bool
{
    use grammar::CmpOperator::*;
    match oper {
        Eq => a == b,
        Neq => a != b,
        Less => a < b,
        Greater => a > b,
        LessEq => a <= b,
        GreaterEq => a >= b,
    }
}

fn _less(oper: CmpOperator) -> bool {
    use grammar::CmpOperator::*;
    match oper {
        Eq => false,
        Neq => true,
        Less => true,
        Greater => false,
        LessEq => true,
        GreaterEq => false,
    }
}

pub fn compare(a: &Comparable, b: &Comparable, oper: CmpOperator)
    -> Result<bool, ()>
{
    use self::ComparableInner::*;

    match (&a.0, &b.0)  {
        // Same type
        (&Bool(x), &Bool(y)) => Ok(_compare(&x, &y, oper)),
        (&I64(x), &I64(y)) => Ok(_compare(&x, &y, oper)),
        (&U64(x), &U64(y)) => Ok(_compare(&x, &y, oper)),
        (&F64(x), &F64(y)) => Ok(_compare(&x, &y, oper)),
        (&Str(x), &Str(y)) => Ok(_compare(&x, &y, oper)),
        (&String(ref x), &String(ref y)) => Ok(_compare(x, y, oper)),
        (&Str(x), &String(ref y)) => Ok(_compare(&x, &&y[..], oper)),
        (&String(ref x), &Str(y)) => Ok(_compare(&&x[..], &y, oper)),

        // Cast to float
        (&I64(x), &F64(y)) => Ok(_compare(&(x as f64), &y, oper)),
        (&F64(x), &I64(y)) => Ok(_compare(&x, &(y as f64), oper)),
        (&U64(x), &F64(y)) => Ok(_compare(&(x as f64), &y, oper)),
        (&F64(x), &U64(y)) => Ok(_compare(&x, &(y as f64), oper)),
        // Two ints
        (&I64(x), &U64(_)) if x < 0 => Ok(_less(oper)),
        (&I64(x), &U64(y)) if y < i64::MAX as u64
            => Ok(_compare(&x, &(y as i64), oper)),
        (&I64(_), &U64(_)) => Ok(_less(oper)),
        (&U64(_), &I64(y)) if y < 0 => Ok(!_less(oper)),
        (&U64(x), &I64(y)) if x < i64::MAX as u64
            => Ok(_compare(&(x as i64), &y, oper)),
        (&U64(_), &I64(_)) => Ok(!_less(oper)),

        // Disabled
        (&Str(_), &Bool(_)) => Err(()),
        (&String(_), &Bool(_)) => Err(()),
        (&Bool(_), &Str(_)) => Err(()),
        (&Bool(_), &String(_)) => Err(()),
        (&Str(_), &I64(_)) => Err(()),
        (&String(_), &I64(_)) => Err(()),
        (&I64(_), &Str(_)) => Err(()),
        (&I64(_), &String(_)) => Err(()),
        (&Str(_), &U64(_)) => Err(()),
        (&String(_), &U64(_)) => Err(()),
        (&U64(_), &Str(_)) => Err(()),
        (&U64(_), &String(_)) => Err(()),
        (&Str(_), &F64(_)) => Err(()),
        (&String(_), &F64(_)) => Err(()),
        (&F64(_), &Str(_)) => Err(()),
        (&F64(_), &String(_)) => Err(()),
        (&Bool(_), &I64(_)) => Err(()),
        (&I64(_), &Bool(_)) => Err(()),
        (&Bool(_), &U64(_)) => Err(()),
        (&U64(_), &Bool(_)) => Err(()),
        (&Bool(_), &F64(_)) => Err(()),
        (&F64(_), &Bool(_)) => Err(()),
    }
}

#[test]
#[cfg(target_arch="x64_64")]
fn size() {
    assert_eq!(size_of::<Comparable>(), 32);
}
