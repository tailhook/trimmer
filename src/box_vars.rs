use std::sync::Arc;
use std::rc::Rc;

use {Variable, Var, DataError, Comparable, Number, Output};


impl<'render, T: Variable<'render>> Variable<'render> for Box<T> {
    fn attr<'x>(&'x self,  attr: &str)
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        (**self).attr(attr)
    }
    fn index<'x>(&'x self,
        key: &(Variable<'render> + 'render))
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        (**self).index(key)
    }
    fn output(&self) -> Result<Output, DataError> {
        (**self).output()
    }
    fn typename(&self) -> &'static str {
        return stringify!(#name);
    }
    fn as_str_key<'x>(&'x self)
        -> Result<&'x str, DataError>
    {
        (**self).as_str_key()
    }
    fn as_int_key(&self) -> Result<usize, DataError> {
        (**self).as_int_key()
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        (**self).as_bool()
    }
    fn as_number(&self) -> Result<Number, DataError> {
        (**self).as_number()
    }
    fn as_comparable(&self) -> Result<Comparable, DataError> {
        (**self).as_comparable()
    }

    fn iterate<'x>(&'x self)
        -> Result<Box<Iterator<Item=
            Var<'x, 'render>>+'x>,
            DataError>
        where 'render: 'x
    {
        (**self).iterate()
    }

    fn iterate_pairs<'x>(&'x self)
        -> Result<Box<Iterator<Item=(Var<'x, 'render>, Var<'x, 'render>)>+'x>,
            DataError>
        where 'render: 'x
    {
        (**self).iterate_pairs()
    }
}

impl<'render, T: Variable<'render>> Variable<'render> for Rc<T> {
    fn attr<'x>(&'x self,  attr: &str)
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        (**self).attr(attr)
    }
    fn index<'x>(&'x self,
        key: &(Variable<'render> + 'render))
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        (**self).index(key)
    }
    fn output(&self) -> Result<Output, DataError> {
        (**self).output()
    }
    fn typename(&self) -> &'static str {
        return stringify!(#name);
    }
    fn as_str_key<'x>(&'x self)
        -> Result<&'x str, DataError>
    {
        (**self).as_str_key()
    }
    fn as_int_key(&self) -> Result<usize, DataError> {
        (**self).as_int_key()
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        (**self).as_bool()
    }
    fn as_number(&self) -> Result<Number, DataError> {
        (**self).as_number()
    }
    fn as_comparable(&self) -> Result<Comparable, DataError> {
        (**self).as_comparable()
    }

    fn iterate<'x>(&'x self)
        -> Result<Box<Iterator<Item=
            Var<'x, 'render>>+'x>,
            DataError>
        where 'render: 'x
    {
        (**self).iterate()
    }

    fn iterate_pairs<'x>(&'x self)
        -> Result<Box<Iterator<Item=(Var<'x, 'render>, Var<'x, 'render>)>+'x>,
            DataError>
        where 'render: 'x
    {
        (**self).iterate_pairs()
    }
}

impl<'render, T: Variable<'render>> Variable<'render> for Arc<T> {
    fn attr<'x>(&'x self,  attr: &str)
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        (**self).attr(attr)
    }
    fn index<'x>(&'x self,
        key: &(Variable<'render> + 'render))
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        (**self).index(key)
    }
    fn output(&self) -> Result<Output, DataError> {
        (**self).output()
    }
    fn typename(&self) -> &'static str {
        return stringify!(#name);
    }
    fn as_str_key<'x>(&'x self)
        -> Result<&'x str, DataError>
    {
        (**self).as_str_key()
    }
    fn as_int_key(&self) -> Result<usize, DataError> {
        (**self).as_int_key()
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        (**self).as_bool()
    }
    fn as_number(&self) -> Result<Number, DataError> {
        (**self).as_number()
    }
    fn as_comparable(&self) -> Result<Comparable, DataError> {
        (**self).as_comparable()
    }

    fn iterate<'x>(&'x self)
        -> Result<Box<Iterator<Item=
            Var<'x, 'render>>+'x>,
            DataError>
        where 'render: 'x
    {
        (**self).iterate()
    }

    fn iterate_pairs<'x>(&'x self)
        -> Result<Box<Iterator<Item=(Var<'x, 'render>, Var<'x, 'render>)>+'x>,
            DataError>
        where 'render: 'x
    {
        (**self).iterate_pairs()
    }
}

impl<'render, T: Variable<'render>> Variable<'render> for &'render T {
    fn attr<'x>(&'x self,  attr: &str)
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        (**self).attr(attr)
    }
    fn index<'x>(&'x self,
        key: &(Variable<'render> + 'render))
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        (**self).index(key)
    }
    fn output(&self) -> Result<Output, DataError> {
        (**self).output()
    }
    fn typename(&self) -> &'static str {
        return stringify!(#name);
    }
    fn as_str_key<'x>(&'x self)
        -> Result<&'x str, DataError>
    {
        (**self).as_str_key()
    }
    fn as_int_key(&self) -> Result<usize, DataError> {
        (**self).as_int_key()
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        (**self).as_bool()
    }
    fn as_number(&self) -> Result<Number, DataError> {
        (**self).as_number()
    }
    fn as_comparable(&self) -> Result<Comparable, DataError> {
        (**self).as_comparable()
    }

    fn iterate<'x>(&'x self)
        -> Result<Box<Iterator<Item=
            Var<'x, 'render>>+'x>,
            DataError>
        where 'render: 'x
    {
        (**self).iterate()
    }

    fn iterate_pairs<'x>(&'x self)
        -> Result<Box<Iterator<Item=(Var<'x, 'render>, Var<'x, 'render>)>+'x>,
            DataError>
        where 'render: 'x
    {
        (**self).iterate_pairs()
    }
}
