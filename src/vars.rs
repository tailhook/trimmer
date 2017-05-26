use std::fmt::{Display, Debug};

use target::{Target, TargetKind};
use render_error::DataError;

#[derive(Debug)]
pub struct Undefined;

pub const UNDEFINED: &'static Undefined = &Undefined;
pub const EMPTY: &'static &'static str = &"";

/// A trait that you need to implement to put variable into the rendering
/// context
///
/// Note: by default all operations return unsupported error
pub trait Variable: Debug {
    /// Evaluates `a.b` operation
    ///
    /// Depending on your domain `a.x` may be equivalent of `a["x"]` or
    /// maybe not. Integer arguments `a.0` are not supported.
    fn attr<'x>(&'x self,  _attr: &str)
        -> Result<&'x Variable, DataError>
    {
        Err(DataError::AttrUnsupported(self.typename()))
    }
    /// Evaluates `a[b]`
    ///
    /// Depending on your domain `a["x"]` may be equivalent of `a.x` or
    /// maybe not.
    ///
    /// You may exract string value for a key with `key.as_str_key()`
    /// and `key.as_int_key()`.
    ///
    /// Note that actual key may have a (rust) type that is different from
    /// type of self (i.e. may come from different library).
    fn index<'x>(&'x self, _key: &Variable)
        -> Result<&'x Variable, DataError>
    {
        Err(DataError::IndexUnsupported(self.typename()))
    }
    /// Evaluates `{{ x }}`
    ///
    /// This operation may not be useful for array-, and mapping-like values
    fn output(&self) -> Result<&Display, DataError> {
        Err(DataError::OutputUnsupported(self.typename()))
    }
    /// Returns type name to use in error messages
    ///
    /// Note this must return actual type of value from user point of view
    /// not just rust type. For example for `Json` type it should describe
    /// `Json::Object` as a mapping and `Json::Array` as an array, not just
    /// return `Json`
    fn typename(&self) -> &'static str;
    /// Return string value of the variable used as key in index
    ///
    /// String keys are used for indexing dicts
    ///
    /// It's okay not to implement this method for complex variables
    fn as_str_key(&self) -> Result<&str, DataError> {
        Err(DataError::StrKeyUnsupported(self.typename()))
    }
    /// Return intenger value of the variable used as key
    ///
    /// Integer keys are used for indexing arrays
    ///
    /// It's okay not to implement this method for complex variables
    fn as_int_key(&self) -> Result<usize, DataError> {
        Err(DataError::IntKeyUnsupported(self.typename()))
    }
    /// Return boolean value of this object
    ///
    /// This is used in conditions `## if x`
    fn as_bool(&self) -> Result<bool, DataError> {
        Err(DataError::BoolUnsupported(self.typename()))
    }

    /// Return iterator over the value if appropriate
    ///
    /// Iterator should be smart enough to find out whether iterator over
    /// key-value pairs or keys is expected. You can also optimize tuple
    /// unpacking in the iterator itself
    fn iterate<'x>(&'x self, target: TargetKind)
        -> Result<Box<Iterator<'x>+'x>, DataError>
    {
        Err(DataError::IterationUnsupported(self.typename(), target))
    }
}
/// A trait that represents iterator over variable
///
/// Note: currently it contains only `next` item but in future we will add
/// more methods that allow `loop.*` variables to work.
pub trait Iterator<'a> {
    /// Set apropriate variables and return `false` if previous iteration was
    /// the last one
    fn next<'y, 'z>(&mut self, target: &mut Target<'a, 'y, 'z>)
        -> bool
    {
        return false;
    }
}

impl Variable for Undefined {
    fn attr<'x>(&'x self, _attr: &str)
        -> Result<&'x Variable, DataError>
    {
        Ok(UNDEFINED)
    }
    fn index<'x>(&'x self,  _key: &Variable)
        -> Result<&Variable, DataError>
    {
        Ok(UNDEFINED)
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(EMPTY)
    }
    fn typename(&self) -> &'static str {
        "undefined"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(false)
    }
}
