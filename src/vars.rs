use std::rc::Rc;
use std::fmt::{Debug};
use std::iter::empty;

use render_error::DataError;
use owning_ref::{OwningRef, Erased};
use {Var, Output};

pub type VarRef<'render> = OwningRef<Rc<Erased+'render>,
                                     Variable<'render>+'render>;

pub enum Val<'a, 'render: 'a>{
    Ref(&'a (Variable<'render> + 'render)),
    Rc(VarRef<'render>),
}


#[derive(Debug)]
pub struct Undefined;

#[derive(Debug)]
pub struct Empty;

pub const UNDEFINED: &'static Undefined = &Undefined;
pub const EMPTY: &'static Empty = &Empty;
pub const TRUE: &'static bool = &true;
pub const FALSE: &'static bool = &false;

/// A trait that you need to implement to put variable into the rendering
/// context
///
/// Note: by default all operations return unsupported error
pub trait Variable<'render>: Debug {
    /// Evaluates `a.b` operation
    ///
    /// Depending on your domain `a.x` may be equivalent of `a["x"]` or
    /// maybe not. Integer arguments `a.0` are not supported.
    fn attr<'x>(&'x self,  _attr: &str)
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
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
    fn index<'x>(&'x self, _key: &(Variable<'render> + 'render))
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        Err(DataError::IndexUnsupported(self.typename()))
    }
    /// Evaluates `{{ x }}`
    ///
    /// This operation may not be useful for array-, and mapping-like values
    fn output(&self) -> Result<Output, DataError> {
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
        -> Result<Box<Iterator<Item=Var<'x, 'render>>+'x>, DataError>
        where 'render: 'x
    {
        Err(DataError::IterationUnsupported(self.typename()))
    }

    /// Return iterator over pairs if appropriate
    fn iterate_pairs<'x>(&'x self)
        -> Result<Box<Iterator<Item=(Var<'x, 'render>, Var<'x, 'render>)>+'x>,
                  DataError>
        where 'render: 'x
    {
        Err(DataError::PairIterationUnsupported(self.typename()))
    }
}

impl<'a> Variable<'a> for Undefined {
    fn attr<'x>(&'x self, _attr: &str)
        -> Result<Var<'x, 'a>, DataError>
        where 'a: 'x
    {
        Ok(Var::undefined())
    }
    fn index<'x>(&'x self,  _key: &Variable)
        -> Result<Var<'x, 'a>, DataError>
        where 'a: 'x
    {
        Ok(Var::undefined())
    }
    fn output(&self) -> Result<Output, DataError> {
        Ok(Output::empty())
    }
    fn typename(&self) -> &'static str {
        "undefined"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(false)
    }
    fn iterate<'x>(&'x self)
        -> Result<Box<Iterator<Item=Var<'x, 'a>>+'x>, DataError>
        where 'a: 'x
    {
        Ok(Box::new(empty()))
    }
    fn iterate_pairs<'x>(&'x self)
        -> Result<Box<Iterator<Item=(Var<'x, 'a>, Var<'x, 'a>)>+'x>,
                  DataError>
        where 'a: 'x
    {
        Ok(Box::new(empty()))
    }
}

impl<'a> Variable<'a> for Empty {
    fn output(&self) -> Result<Output, DataError> {
        Ok(Output::empty())
    }
    fn typename(&self) -> &'static str {
        "str"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(false)
    }
}

impl<'a, 'render> Var<'a, 'render> {
    /// Embed and owned reference to a value
    pub fn owned<'x, 'y: 'x, T: Variable<'y>+'y>(x: T) -> Var<'x, 'y>
        where 'y: 'x, T: 'y
    {
        Var(Val::Rc(OwningRef::new(Rc::new(x))
                .map(|x| x as &Variable).erase_owner()))
    }
    /// Embed a static string as a variable
    ///
    /// Currently this uses reference counted object that contains pointer,
    /// but we want to figure out better way to reference static strings
    pub fn str(x: &'static str) -> Var<'a, 'render> {
        // This is a limitation of a rust type system
        Var(Val::Rc(OwningRef::new(Rc::new(x))
                .map(|x| x as &Variable)
                .erase_owner()))
    }
    /// Create a borrowed reference to the variable
    pub fn borrow<'x, T: Variable<'render>+'render>(x: &'x T)
        -> Var<'x, 'render>
        where 'render: 'x
    {
        Var(Val::Ref(x))
    }
    /// Create an undefined variable reference
    pub fn undefined<'x, 'y: 'x>() -> Var<'x, 'y> {
        Var(Val::Ref(UNDEFINED))
    }
    /// Create a variable that contains an empty string
    pub fn empty<'x, 'y: 'x>() -> Var<'x, 'y> {
        Var(Val::Ref(EMPTY))
    }
    /// Create a variable that boolean true
    pub fn bool_true<'x, 'y: 'x>() -> Var<'x, 'y> {
        Var(Val::Ref(EMPTY))
    }
    /// Create a variable that boolean false
    pub fn bool_false<'x, 'y: 'x>() -> Var<'x, 'y> {
        Var(Val::Ref(EMPTY))
    }
}
