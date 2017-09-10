use std::rc::Rc;
use std::i64;

use owning_ref::OwningRef;

use vars::{VarRef, Variable};

/// An internal representation of a number that may be integer of real
///
/// Use `into()` conversion to make the value.
#[derive(Debug)]  // TODO(tailhook) make normal debug
pub struct Number(NumberInner);

#[derive(Clone, Copy, Debug)]
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

fn val<'x, T: Variable<'x>+'x>(v: T) -> VarRef<'x> {
    OwningRef::new(Rc::new(v)).map(|x| x as &Variable).erase_owner()
}

fn norm(n: NumberInner) -> NumberInner {
    use self::NumberInner::*;
    match n {
        I64(a) if a > 0 => U64(a as u64),
        n => n,
    }
}

pub fn add<'x>(a: Number, b: Number) -> VarRef<'x> {
    use self::NumberInner::*;
    match (norm(a.0), norm(b.0)) {
        (I64(a), I64(b)) => {
            a.checked_add(b).map(val)
            .unwrap_or_else(|| val((a as f64) + (b as f64)))
        }
        (U64(a), U64(b)) => {
            a.checked_add(b).map(val)
            .unwrap_or_else(|| val((a as f64) + (b as f64)))
        }
        (F64(a), F64(b)) => val(a + b),
        (I64(a), F64(b)) => val(a as f64 + b),
        (F64(a), I64(b)) => val(a + b as f64),
        (U64(a), F64(b)) => val(a as f64 + b),
        (F64(a), U64(b)) => val(a + b as f64),
        (I64(a), U64(b)) | (U64(b), I64(a)) => {
            b.checked_sub((-a as u64)).map(val)
            .or_else(|| (-a as u64).checked_sub(b).map(|x| val(-(x as i64))))
            .unwrap_or_else(|| val((a as f64) + (b as f64)))
        }
    }
}

pub fn sub<'x>(a: Number, b: Number) -> VarRef<'x> {
    use self::NumberInner::*;
    match (norm(a.0), norm(b.0)) {
        (I64(a), I64(b)) => {
            a.checked_sub(b).map(val)
            .unwrap_or_else(|| val((a as f64) - (b as f64)))
        }
        (U64(a), U64(b)) => {
            a.checked_sub(b).map(val)
            .unwrap_or_else(|| val((a as f64) - (b as f64)))
        }
        (F64(a), F64(b)) => val(a - b),
        (I64(a), F64(b)) => val(a as f64 - b),
        (F64(a), I64(b)) => val(a - b as f64),
        (U64(a), F64(b)) => val(a as f64 - b),
        (F64(a), U64(b)) => val(a - b as f64),
        (U64(a), I64(b)) => {
            a.checked_add((-b as u64)).map(val)
            .unwrap_or_else(|| val((a as f64) - (b as f64)))
        }
        (I64(a), U64(b)) if b < i64::MAX as u64 => {
            a.checked_sub(b as i64).map(val)
            .unwrap_or_else(|| val((a as f64) - (b as f64)))
        }
        (I64(a), U64(b)) => val((a as f64) - (b as f64)),
    }
}

pub fn mul<'x>(a: Number, b: Number) -> VarRef<'x> {
    use self::NumberInner::*;
    match (norm(a.0), norm(b.0)) {
        (I64(a), I64(b)) => {
            a.checked_mul(b).map(val)
            .unwrap_or_else(|| val((a as f64) * (b as f64)))
        }
        (U64(a), U64(b)) => {
            a.checked_mul(b).map(val)
            .unwrap_or_else(|| val((a as f64) * (b as f64)))
        }
        (F64(a), F64(b)) => val(a * b),
        (I64(a), F64(b)) => val(a as f64 * b),
        (F64(a), I64(b)) => val(a * b as f64),
        (U64(a), F64(b)) => val(a as f64 * b),
        (F64(a), U64(b)) => val(a * b as f64),
        (U64(a), I64(b)) => val((a as f64) * (b as f64)),
        (I64(a), U64(b)) => val((a as f64) * (b as f64)),
    }
}

pub fn div<'x>(a: Number, b: Number) -> VarRef<'x> {
    use self::NumberInner::*;
    match (norm(a.0), norm(b.0)) {
        (I64(a), I64(b)) => {
            if a % b == 0 {
                val(a / b)
            } else {
                val((a as f64) / (b as f64))
            }
        }
        (U64(a), U64(b)) => {
            if a % b == 0 {
                val(a / b)
            } else {
                val((a as f64) / (b as f64))
            }
        }
        (F64(a), F64(b)) => val(a / b),
        (I64(a), F64(b)) => val(a as f64 / b),
        (F64(a), I64(b)) => val(a / b as f64),
        (U64(a), F64(b)) => val(a as f64 / b),
        (F64(a), U64(b)) => val(a / b as f64),
        (U64(a), I64(b)) => val((a as f64) / (b as f64)),
        (I64(a), U64(b)) => val((a as f64) / (b as f64)),
    }
}

pub fn modulo<'x>(a: Number, b: Number) -> VarRef<'x> {
    use self::NumberInner::*;
    match (norm(a.0), norm(b.0)) {
        (I64(a), I64(b)) => val(a % b),
        (U64(a), U64(b)) => val(a % b),
        (F64(a), F64(b)) => val(a % b),
        (I64(a), F64(b)) => val(a as f64 % b),
        (F64(a), I64(b)) => val(a % b as f64),
        (U64(a), F64(b)) => val(a as f64 % b),
        (F64(a), U64(b)) => val(a % b as f64),
        (U64(a), I64(b)) => val((a as f64) % (b as f64)),
        (I64(a), U64(b)) => val((a as f64) % (b as f64)),
    }
}
