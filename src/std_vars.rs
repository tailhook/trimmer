use std::collections::{HashMap, BTreeMap, HashSet, BTreeSet};
use std::hash::Hash;
use std::net::{SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr};

use render_error::DataError;
use vars::{Variable};
use {Var, Output, Number, Comparable};

const TRUE: &&str = &"true";
const FALSE: &&str = &"false";


impl<'a, 'render: 'a> Variable<'render> for &'a str {
    fn typename(&self) -> &'static str {
        "str"
    }
    fn output(&self) -> Result<Output, DataError> {
        Ok(self.into())
    }
    fn as_str_key(&self) -> Result<&str, DataError> {
        Ok(self)
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
    fn as_comparable(&self) -> Result<Comparable, DataError> {
        Ok((*self).into())
    }
}

impl<'x> Variable<'x> for String {
    fn typename(&self) -> &'static str {
        "String"
    }
    fn as_str_key(&self) -> Result<&str, DataError> {
        Ok(&self[..])
    }
    fn output(&self) -> Result<Output, DataError> {
        Ok(self.into())
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
    fn as_comparable(&self) -> Result<Comparable, DataError> {
        Ok(self[..].into())
    }
}

impl<'x> Variable<'x> for IpAddr {
    fn typename(&self) -> &'static str {
        "IpAddr"
    }
    fn output(&self) -> Result<Output, DataError> {
        Ok(self.into())
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(true)
    }
}

impl<'x> Variable<'x> for Ipv4Addr {
    fn typename(&self) -> &'static str {
        "Ipv4Addr"
    }
    fn output(&self) -> Result<Output, DataError> {
        Ok(self.into())
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(true)
    }
}

impl<'x> Variable<'x> for Ipv6Addr {
    fn typename(&self) -> &'static str {
        "Ipv4Addr"
    }
    fn output(&self) -> Result<Output, DataError> {
        Ok(self.into())
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(true)
    }
}

impl<'x> Variable<'x> for SocketAddr {
    fn typename(&self) -> &'static str {
        "IpAddr"
    }
    fn output(&self) -> Result<Output, DataError> {
        Ok(self.into())
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(true)
    }
}

impl<'x> Variable<'x> for Option<&'x str> {
    fn typename(&self) -> &'static str {
        "Option<str>"
    }
    fn output(&self) -> Result<Output, DataError> {
        Ok(match self.as_ref() {
            Some(x) => x.into(),
            None => Output::empty(),
        })
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(true)
    }
    fn as_comparable(&self) -> Result<Comparable, DataError> {
        match *self {
            Some(ref x) => Ok(x[..].into()),
            None => Ok("".into()),
        }
    }
}

macro_rules! impl_number {
    ($typ: ident, $zero: expr) => {
        impl<'x> Variable<'x> for $typ {
            fn typename(&self) -> &'static str {
                stringify!($typ)
            }
            fn as_int_key(&self) -> Result<usize, DataError> {
                Ok(*self as usize)
            }
            fn output(&self) -> Result<Output, DataError> {
                Ok(self.into())
            }
            fn as_bool(&self) -> Result<bool, DataError> {
                Ok(*self != $zero)
            }
            fn as_number(&self) -> Result<Number, DataError> {
                Ok((*self).into())
            }
            fn as_comparable(&self) -> Result<Comparable, DataError> {
                Ok((*self).into())
            }
        }
    }
}

impl_number!(u8, 0);
impl_number!(i8, 0);
impl_number!(u16, 0);
impl_number!(i16, 0);
impl_number!(u32, 0);
impl_number!(i32, 0);
impl_number!(u64, 0);
impl_number!(i64, 0);
impl_number!(f32, 0.);
impl_number!(f64, 0.);
impl_number!(usize, 0);
impl_number!(isize, 0);

impl<'render, V> Variable<'render> for HashMap<String, V>
    where V: Variable<'render> + 'render
{
    fn attr<'x>(&'x self, attr: &str)
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        self.get(attr)
        .map(|x| Var::borrow(x))
        .ok_or_else(|| DataError::AttrNotFound)
    }
    fn index<'x>(&'x self, index: &(Variable<'render>+'render))
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        self.get(index.as_str_key()?)
        .map(|x| Var::borrow(x))
        .ok_or(DataError::IndexNotFound)
    }
    fn typename(&self) -> &'static str {
        "HashMap"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
    fn iterate_pairs<'x>(&'x self)
        -> Result<Box<Iterator<Item=(Var<'x, 'render>, Var<'x, 'render>)>+'x>,
                  DataError>
        where 'render: 'x
    {
        Ok(Box::new(self.iter()
            .map(|(x, y)| (Var::borrow(x), Var::borrow(y)))))
    }
}

impl<'a: 'render, 'render, V> Variable<'render> for HashMap<&'a str, V>
    where V: Variable<'render> + 'render
{
    fn attr<'x>(&'x self, attr: &str)
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        self.get(attr)
        .map(|x| Var::borrow(x))
        .ok_or_else(|| DataError::AttrNotFound)
    }
    fn index<'x>(&'x self, index: &(Variable<'render>+'render))
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        self.get(index.as_str_key()?)
        .map(|x| Var::borrow(x))
        .ok_or(DataError::IndexNotFound)
    }
    fn typename(&self) -> &'static str {
        "HashMap"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
    fn iterate_pairs<'x>(&'x self)
        -> Result<Box<Iterator<Item=(Var<'x, 'render>, Var<'x, 'render>)>+'x>,
                  DataError>
        where 'render: 'x
    {
        Ok(Box::new(self.iter()
            .map(|(x, y)| (Var::borrow(x), Var::borrow(y)))))
    }
}

impl<'render, V> Variable<'render> for BTreeMap<String, V>
    where V: Variable<'render> + 'render
{
    fn attr<'x>(&'x self, attr: &str)
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        self.get(attr)
        .map(|x| Var::borrow(x))
        .ok_or_else(|| DataError::AttrNotFound)
    }
    fn index<'x>(&'x self, index: &(Variable<'render>+'render))
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        self.get(index.as_str_key()?)
        .map(|x| Var::borrow(x))
        .ok_or(DataError::IndexNotFound)
    }
    fn typename(&self) -> &'static str {
        "BTreeMap"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
    fn iterate_pairs<'x>(&'x self)
        -> Result<Box<Iterator<Item=(Var<'x, 'render>, Var<'x, 'render>)>+'x>,
                  DataError>
        where 'render: 'x
    {
        Ok(Box::new(self.iter()
            .map(|(x, y)| (Var::borrow(x), Var::borrow(y)))))
    }
}

impl<'a: 'render, 'render, V> Variable<'render> for BTreeMap<&'a str, V>
    where V: Variable<'render> + 'render
{
    fn attr<'x>(&'x self, attr: &str)
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        self.get(attr)
        .map(|x| Var::borrow(x))
        .ok_or_else(|| DataError::AttrNotFound)
    }
    fn index<'x>(&'x self, index: &(Variable<'render>+'render))
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        self.get(index.as_str_key()?)
        .map(|x| Var::borrow(x))
        .ok_or(DataError::IndexNotFound)
    }
    fn typename(&self) -> &'static str {
        "BTreeMap"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
    fn iterate_pairs<'x>(&'x self)
        -> Result<Box<Iterator<Item=(Var<'x, 'render>, Var<'x, 'render>)>+'x>,
                  DataError>
        where 'render: 'x
    {
        Ok(Box::new(self.iter()
            .map(|(x, y)| (Var::borrow(x), Var::borrow(y)))))
    }
}

impl<'a, 'render, T: Variable<'render> + 'render> Variable<'render> for Vec<T> {
    fn typename(&self) -> &'static str {
        "Vec"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
    fn iterate<'x>(&'x self)
        -> Result<Box<Iterator<Item=Var<'x, 'render>>+'x>, DataError>
        where 'render: 'x
    {
        Ok(Box::new(self.iter().map(|x| Var::borrow(x))))
    }
    fn index<'x>(&'x self, index: &(Variable<'render>+'render))
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        self.get(index.as_int_key()?)
        .map(|x| Var::borrow(x))
        .ok_or(DataError::IndexNotFound)
    }
}

impl<'a, 'render, T> Variable<'render> for HashSet<T>
    where T: Variable<'render> + Hash + Eq + 'render
{
    fn typename(&self) -> &'static str {
        "HashSet"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
    fn iterate<'x>(&'x self)
        -> Result<Box<Iterator<Item=Var<'x, 'render>>+'x>, DataError>
        where 'render: 'x
    {
        Ok(Box::new(self.iter().map(|x| Var::borrow(x))))
    }
}

impl<'a, 'render, T> Variable<'render> for BTreeSet<T>
    where T: Variable<'render> + Ord + Eq + 'render
{
    fn typename(&self) -> &'static str {
        "BTreeSet"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
    fn iterate<'x>(&'x self)
        -> Result<Box<Iterator<Item=Var<'x, 'render>>+'x>, DataError>
        where 'render: 'x
    {
        Ok(Box::new(self.iter().map(|x| Var::borrow(x))))
    }
}

impl<'x> Variable<'x> for bool {
    fn typename(&self) -> &'static str {
        "bool"
    }
    fn output(&self) -> Result<Output, DataError> {
        Ok(match *self {
            true => TRUE.into(),
            false => FALSE.into(),
        })
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(*self)
    }
    fn as_comparable(&self) -> Result<Comparable, DataError> {
        Ok((*self).into())
    }
}
