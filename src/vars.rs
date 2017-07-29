use std::rc::Rc;
use std::fmt::{Display, Debug};

use render_error::DataError;
use owning_ref::{ErasedRcRef, OwningRef};

pub enum Var<'a> {
    #[doc(hidden)]
    Ref(&'a (Variable + 'static)),
    #[doc(hidden)]
    Rc(ErasedRcRef<Variable>),
}

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
        -> Result<Var<'x>, DataError>
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
        -> Result<Var<'x>, DataError>
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
    fn as_str_key<'x>(&'x self) -> Result<&'x str, DataError> {
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
    fn iterate<'x>(&'x self)
        -> Result<Box<Iterator<Item=Var<'x>>+'x>, DataError>
    {
        Err(DataError::IterationUnsupported(self.typename()))
    }
}

impl Variable for Undefined {
    fn attr<'x>(&'x self, _attr: &str)
        -> Result<Var<'x>, DataError>
    {
        Ok(Var::Ref(UNDEFINED))
    }
    fn index<'x>(&'x self,  _key: &Variable)
        -> Result<Var<'x>, DataError>
    {
        Ok(Var::Ref(UNDEFINED))
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

impl<'a> Var<'a> {
    pub fn owned<T: Variable + 'static>(x: T) -> Var<'static> {
        Var::Rc(OwningRef::new(Rc::new(x))
                .map(|x| x as &Variable).erase_owner())
    }
    pub fn str(x: &'static str) -> Var<'static> {
        // This is a limitation of a rust type system
        Var::Rc(OwningRef::new(Rc::new(x))
                .map(|x| x as &Variable)
                .erase_owner())
    }
}
