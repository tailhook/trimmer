use std::fmt::{Display, Debug};

use context::Context;
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
    fn attr<'x>(&'x self, _ctx: &mut Context,  _attr: &str)
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
    fn index(&self, _ctx: &mut Context, _key: &Variable)
        -> Result<&Variable, DataError>
    {
        Err(DataError::IndexUnsupported(self.typename()))
    }
    /// Evaluates `{{ x }}`
    ///
    /// This operation may not be useful for array-, and mapping-like values
    fn output(&self, _ctx: &mut Context) -> Result<&Display, DataError> {
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
    fn as_str_key(&self, _ctx: &mut Context) -> Result<&str, DataError> {
        Err(DataError::StrKeyUnsupported(self.typename()))
    }
    /// Return intenger value of the variable used as key
    ///
    /// Integer keys are used for indexing arrays
    ///
    /// It's okay not to implement this method for complex variables
    fn as_int_key(&self, _ctx: &mut Context) -> Result<usize, DataError> {
        Err(DataError::IntKeyUnsupported(self.typename()))
    }
}

impl Variable for Undefined {
    fn attr<'x>(&'x self, _: &mut Context, _attr: &str)
        -> Result<&'x Variable, DataError>
    {
        Ok(UNDEFINED)
    }
    fn index<'x>(&'x self, _: &mut Context,  _key: &Variable)
        -> Result<&Variable, DataError>
    {
        Ok(UNDEFINED)
    }
    fn output(&self, _: &mut Context) -> Result<&Display, DataError> {
        Ok(EMPTY)
    }
    fn typename(&self) -> &'static str {
        "undefined"
    }
}
