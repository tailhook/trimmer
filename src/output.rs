use std::fmt::{self, Display};

use Output;

pub const EMPTY_STR: &'static &'static str = &"";

pub enum OutImpl<'a> {
    Borrow(&'a (Display + 'a)),
    Owned(Box<Display + 'a>),
}

impl<'a, T: Display + 'a> From<&'a T> for Output<'a> {
    fn from(t: &'a T) -> Output<'a> {
        Output(OutImpl::Borrow(t))
    }
}

impl<'a> Output<'a> {
    /// Create an owned value for output
    ///
    /// Usually you should use a reference like `self.into()`, but sometimes
    /// something more complex may be returned using `Output::owned(value)`
    pub fn owned<T: Display + 'a>(t: T) -> Output<'a> {
        Output(OutImpl::Owned(Box::new(t)))
    }
    /// Returns an empty output
    pub fn empty() -> Output<'a> {
        Output(OutImpl::Borrow(EMPTY_STR))
    }
}

impl<'a> fmt::Display for OutImpl<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OutImpl::Borrow(x) => x.fmt(f),
            OutImpl::Owned(ref x) => x.fmt(f),
        }
    }
}
