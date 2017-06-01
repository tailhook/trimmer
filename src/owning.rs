use std::rc::Rc;

use owning_ref::OwningRef;

use grammar::{self, Template as Tpl, Expr};


pub trait Own {
    type Owned;
    fn own(&self) -> Self::Owned;
}

type Owned<T> = OwningRef<Rc<Tpl>, T>;

pub enum ExprCode {
    Str(Owned<String>),
    Int(i64),
    Float(f64),
    Var(Owned<String>),
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
}

impl Own for OwningRef<Rc<Tpl>, grammar::ExprCode> {
    type Owned = ExprCode;
    fn own(&self) -> ExprCode {
        use grammar::ExprCode as I;
        use owning::ExprCode as O;
        match **self {
            I::Str(_) => O::Str(omap!(self, I::Str(ref x) => x)),
            _ => unimplemented!(),
            /*
            I::Int(v) => O::Int(v),
            I::Float(v) => O::Float(v),
            I::Var(ref s) => O::Var(s),
            I::Attr(ref e, ref a) => O::Attr(e, &a[..]),
            I::Item(ref e, ref i) => O::Attr,
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
            */
        }
    }
}
