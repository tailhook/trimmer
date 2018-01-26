use std::rc::Rc;
use std::sync::Arc;

use owning_ref::OwningRef;

use grammar::{self, Template as Tpl, Expr, CmpOperator};


pub trait Own {
    type Owned;
    fn own(&self) -> Self::Owned;
}

type Owned<T> = OwningRef<Rc<Arc<Tpl>>, T>;

#[allow(dead_code)] // TODO(tailhook)
#[derive(Debug)]
pub enum ExprCode {
    Str(Owned<String>),
    Int(Owned<i64>),
    Float(Owned<f64>),
    Var(Owned<str>),
    Attr(Owned<Expr>, Owned<String>),
    Item(Owned<Expr>, Owned<Expr>),
    Filter(Owned<Expr>, Owned<Expr>),
    And(Owned<Expr>, Owned<Expr>),
    Or(Owned<Expr>, Owned<Expr>),
    Not(Owned<Expr>),
    Comparison(Owned<Expr>, Owned<[(CmpOperator, Expr)]>),
    List(Vec<Expr>),
    Dict(Vec<(Owned<Expr>, Owned<Expr>)>),
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
            I::Int(_) => O::Int(omap!(self, I::Int(ref x))),
            I::Float(_) => O::Float(omap!(self, I::Float(ref x))),
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
            I::Comparison(_, _) => O::Comparison(
                omap!(self, I::Comparison(ref a, _) => &**a),
                omap!(self, I::Comparison(_, ref vec) => &vec[..])),
            I::List(_) => unimplemented!(),
            I::Dict(ref vec) => {
                O::Dict((0..vec.len()).map(|index| {
                    let a = self.clone().map(|expr| match *expr {
                        I::Dict(ref vec) => &vec[index].0,
                        _ => unreachable!(),
                    });
                    let b = self.clone().map(|expr| match *expr {
                        I::Dict(ref vec) => &vec[index].1,
                        _ => unreachable!(),
                    });
                    (a, b)
                }).collect())
            }
            I::Range(_, _) => unimplemented!(),
            I::Add(_, _) => O::Add(
                omap!(self, I::Add(ref a, _) => &**a),
                omap!(self, I::Add(_, ref b) => &**b)),
            I::Sub(_, _) => O::Sub(
                omap!(self, I::Sub(ref a, _) => &**a),
                omap!(self, I::Sub(_, ref b) => &**b)),
            I::Mul(_, _) => O::Mul(
                omap!(self, I::Mul(ref a, _) => &**a),
                omap!(self, I::Mul(_, ref b) => &**b)),
            I::Div(_, _) => O::Div(
                omap!(self, I::Div(ref a, _) => &**a),
                omap!(self, I::Div(_, ref b) => &**b)),
            I::Mod(_, _) => O::Mod(
                omap!(self, I::Mod(ref a, _) => &**a),
                omap!(self, I::Mod(_, ref b) => &**b)),
        }
    }
}
