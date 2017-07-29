use std::rc::Rc;
use std::sync::Arc;

use owning_ref::OwningRef;

use grammar::{self, Template as Tpl, Expr};


pub trait Own {
    type Owned;
    fn own(&self) -> Self::Owned;
}

type Owned<T> = OwningRef<Rc<Arc<Tpl>>, T>;

pub enum ExprCode {
    Str(Owned<String>),
    Int(i64),
    Float(f64),
    Var(Owned<str>),
    Attr(Owned<Expr>, Owned<String>),
    Item(Owned<Expr>, Owned<Expr>),
    Filter(Owned<Expr>, Owned<Expr>),
    And(Owned<Expr>, Owned<Expr>),
    Or(Owned<Expr>, Owned<Expr>),
    Not(Owned<Expr>),
    Eq(Owned<Expr>, Owned<Expr>),
    Neq(Owned<Expr>, Owned<Expr>),
    LessEq(Owned<Expr>, Owned<Expr>),
    Less(Owned<Expr>, Owned<Expr>),
    GreaterEq(Owned<Expr>, Owned<Expr>),
    Greater(Owned<Expr>, Owned<Expr>),
    List(Vec<Expr>),
    Dict(Vec<(Expr, Expr)>),
    Range(Option<Owned<Expr>>, Option<Owned<Expr>>),
    Add(Owned<Expr>, Owned<Expr>),
    Sub(Owned<Expr>, Owned<Expr>),
    Mul(Owned<Expr>, Owned<Expr>),
    Div(Owned<Expr>, Owned<Expr>),
    Mod(Owned<Expr>, Owned<Expr>),
}

macro_rules! omap {
    ($me:ident, $ns:ident :: $variant:ident ( ref $var:ident ) => $expr:expr)
    => {
        $me.clone().map(|expr| match *expr {
            $ns::$variant(ref $var) => $expr,
            _ => unreachable!(),
        })
    };
    ($me:ident, $ns:ident :: $variant:ident ( ref $var:ident ))
    => {
        $me.clone().map(|expr| match *expr {
            $ns::$variant(ref $var) => $var,
            _ => unreachable!(),
        })
    };
    ($me:ident, $ns:ident :: $variant:ident ( ref $var:ident, _ ))
    => {
        $me.clone().map(|expr| match *expr {
            $ns::$variant(ref $var, _) => $var,
            _ => unreachable!(),
        })
    };

    ($me:ident, $ns:ident :: $variant:ident ( _, ref $var:ident ))
    => {
        $me.clone().map(|expr| match *expr {
            $ns::$variant(_, ref $var) => $var,
            _ => unreachable!(),
        })
    };
    ($me:ident, $ns:ident :: $variant:ident ( ref $var:ident, _ ) => $e:expr)
    => {
        $me.clone().map(|expr| match *expr {
            $ns::$variant(ref $var, _) => $e,
            _ => unreachable!(),
        })
    };

    ($me:ident, $ns:ident :: $variant:ident ( _, ref $var:ident ) => $e:expr)
    => {
        $me.clone().map(|expr| match *expr {
            $ns::$variant(_, ref $var) => $e,
            _ => unreachable!(),
        })
    };
}

impl Own for OwningRef<Rc<Arc<Tpl>>, grammar::ExprCode> {
    type Owned = ExprCode;
    fn own(&self) -> ExprCode {
        use grammar::ExprCode as I;
        use owning::ExprCode as O;
        match **self {
            I::Str(_) => O::Str(omap!(self, I::Str(ref x))),
            I::Int(v) => O::Int(v),
            I::Float(v) => O::Float(v),
            I::Var(_) => O::Var(omap!(self, I::Var(ref x) => &**x)),
            I::Attr(_, _) => O::Attr(
                omap!(self, I::Attr(ref x, _) => &**x),
                omap!(self, I::Attr(_, ref a) => a)),
            I::Item(_, _) => O::Item(
                omap!(self, I::Item(ref x, _) => &**x),
                omap!(self, I::Item(_, ref i) => &**i)),
            I::Filter(_, _) => O::Filter(
                omap!(self, I::Filter(ref a, _) => &**a),
                omap!(self, I::Filter(_, ref b) => &**b)),
            I::And(_, _) => O::And(
                omap!(self, I::And(ref a, _) => &**a),
                omap!(self, I::And(_, ref b) => &**b)),
            I::Or(_, _) => O::Or(
                omap!(self, I::Or(ref a, _) => &**a),
                omap!(self, I::Or(_, ref b) => &**b)),
            I::Not(_) => O::Not(omap!(self, I::Not(ref x) => &**x)),
            I::Eq(_, _) => O::Eq(
                omap!(self, I::Eq(ref a, _) => &**a),
                omap!(self, I::Eq(_, ref b) => &**b)),
            I::Neq(_, _) => O::Neq(
                omap!(self, I::Neq(ref a, _) => &**a),
                omap!(self, I::Neq(_, ref b) => &**b)),
            I::LessEq(_, _) => O::LessEq(
                omap!(self, I::LessEq(ref a, _) => &**a),
                omap!(self, I::LessEq(_, ref b) => &**b)),
            I::Less(_, _) => O::Less(
                omap!(self, I::Less(ref a, _) => &**a),
                omap!(self, I::Less(_, ref b) => &**b)),
            I::GreaterEq(_, _) => O::LessEq(
                omap!(self, I::GreaterEq(ref a, _) => &**a),
                omap!(self, I::GreaterEq(_, ref b) => &**b)),
            I::Greater(_, _) => O::Less(
                omap!(self, I::Greater(ref a, _) => &**a),
                omap!(self, I::Greater(_, ref b) => &**b)),
            I::List(_) => unimplemented!(),
            I::Dict(_) => unimplemented!(),
            I::Range(_, _) => unimplemented!(),
            I::Add(_, _) => O::Less(
                omap!(self, I::Add(ref a, _) => &**a),
                omap!(self, I::Add(_, ref b) => &**b)),
            I::Sub(_, _) => O::Less(
                omap!(self, I::Sub(ref a, _) => &**a),
                omap!(self, I::Sub(_, ref b) => &**b)),
            I::Mul(_, _) => O::Less(
                omap!(self, I::Mul(ref a, _) => &**a),
                omap!(self, I::Mul(_, ref b) => &**b)),
            I::Div(_, _) => O::Less(
                omap!(self, I::Div(ref a, _) => &**a),
                omap!(self, I::Div(_, ref b) => &**b)),
            I::Mod(_, _) => O::Less(
                omap!(self, I::Mod(ref a, _) => &**a),
                omap!(self, I::Mod(_, ref b) => &**b)),
        }
    }
}
