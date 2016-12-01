use std::rc::Rc;
use std::fmt::{self, Display, Debug};
use std::collections::HashMap;

use render_error::DataError;

#[derive(Debug)]
pub struct Undefined;

pub const UNDEFINED: &'static Undefined = &Undefined;
const EMPTY: &'static &'static str = &"";

pub struct Var<'a> {
    value: VarImpl<'a>
}

pub enum VarImpl<'a> {
    Str(&'a str),
    Borrowed(&'a Variable<'a>),
    Owned(Rc<Variable<'a>+'a>),
}

/// A trait that you need to implement to put variable into the rendering
/// context
///
/// Note: by default all operations return unsupported error
pub trait Variable<'a>: Debug {
    /// Evaluates `a.b` operation
    ///
    /// Depending on your domain `a.x` may be equivalent of `a["x"]` or
    /// maybe not. Integer arguments `a.0` are not supported.
    fn attr(&self, _attr: &str) -> Result<Var<'a>, DataError> {
        Err(DataError::AttrUnsupported(self.typename()))
    }
    /// Evaluates `a[b]`
    ///
    /// Depending on your domain `a["x"]` may be equivalent of `a.x` or
    /// maybe not.
    ///
    /// You may exract string value for a key with `key.output().to_string()`.
    /// Note that actual key may have a (rust) type that is different from
    /// type of self (i.e. may come from different library).
    fn subscript<'x>(&'a self, _key: &'a Variable<'x>) -> Result<Var<'a>, DataError> {
        Err(DataError::SubscriptUnsupported(self.typename()))
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
}

/// Holds variables passed to a template rendering function
pub struct Context<'var> {
    vars: HashMap<String, Var<'var>>,
}

impl<'var> Context<'var> {
    /// Create a new context
    pub fn new() -> Context<'var> {
        Context {
            vars: HashMap::new(),
        }
    }

    /// Add a variable to context
    pub fn add<V: IntoVariable<'var> + Debug + 'var>(&mut self,
        key: &str, value: V)
    {
        let v= value.into_variable();
        println!("ADD {:?} = {:?}", key, v);
        self.vars.insert(key.to_string(), v);
    }

    pub fn get(&self, key: &str) -> Option<Var<'var>>
    {
        self.vars.get(key).map(|x| x.me())
    }
}

impl<'a> Var<'a> {
    fn me(&self) -> Var<'a> {
        use self::VarImpl::*;
        match self.value {
            Str(x) => Var { value: Str(x) },
            Borrowed(x) => Var { value: Borrowed(x) },
            Owned(ref x) => Var { value: Owned(x.clone()) },
        }
    }
}

impl<'a> Variable<'a> for Undefined {
    fn attr(&self, _attr: &str) -> Result<Var<'a>, DataError> {
        Ok(Var { value: VarImpl::Borrowed(UNDEFINED) })
    }
    fn subscript(&self, _key: &Variable) -> Result<Var<'a>, DataError> {
        Ok(Var { value: VarImpl::Borrowed(UNDEFINED) })
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(EMPTY)
    }
    fn typename(&self) -> &'static str {
        "undefined"
    }
}

impl<'a> Debug for Var<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            VarImpl::Borrowed(x) => write!(f, "b{:?}", x),
            VarImpl::Owned(ref x) => write!(f, "o{:?}", x),
            VarImpl::Str(x) => write!(f, "s{:?}", x),
        }
    }
}

impl<'a> Variable<'a> for Var<'a> {
    fn attr(&self, attr: &str) -> Result<Var<'a>, DataError> {
        match self.value {
            VarImpl::Borrowed(x) => x.attr(attr),
            VarImpl::Owned(ref x) => x.attr(attr),
            VarImpl::Str(_) => Err(DataError::AttrUnsupported("&str")),
        }
    }
    fn subscript<'x>(&'a self, key: &'a Variable<'x>)
        -> Result<Var<'a>, DataError>
    {
        match self.value {
            VarImpl::Borrowed(x) => x.subscript(key),
            VarImpl::Owned(ref x) => x.subscript(key),
            VarImpl::Str(_) => Err(DataError::SubscriptUnsupported("&str")),
        }
    }
    fn output(&self) -> Result<&Display, DataError> {
        match self.value {
            VarImpl::Borrowed(x) => x.output(),
            VarImpl::Owned(ref x) => x.output(),
            VarImpl::Str(ref x) => Ok(x),
        }
    }
    fn typename(&self) -> &'static str {
        match self.value {
            VarImpl::Borrowed(x) => x.typename(),
            VarImpl::Owned(ref x) => x.typename(),
            VarImpl::Str(_) => "&str",
        }
    }
}


pub trait IntoVariable<'a> {
    fn into_variable(self) -> Var<'a>;
}


impl<'a> IntoVariable<'a> for String {
    fn into_variable(self) -> Var<'a> {
        Var { value: VarImpl::Owned(Rc::new(self)) }
    }
}

impl<'a> IntoVariable<'a> for &'a str {
    fn into_variable(self) -> Var<'a> {
        Var { value: VarImpl::Str(self) }
    }
}

impl<'a, T: Variable<'a>> IntoVariable<'a> for &'a T
{
    fn into_variable(self) -> Var<'a> {
        Var { value: VarImpl::Borrowed(self) }
    }
}

