pub struct Syntax {
    indent: bool,
    curly: bool,
    square: bool,
    round: bool,
}

pub enum Expr {
    Str(String),
    Int(i64),
    Float(f64),
    Var(String),
    Attr(Box<Expr>, String),
    Item(Box<Expr>, Box<Expr>),

    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Neq(Box<Expr>, Box<Expr>),
    LessEq(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    GreaterEq(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    Filter(Box<Expr>, Box<Expr>),  // pipe operator
    List(Vec<Expr>),
    Dict(Vec<(Expr, Expr)>),
    Range(Option<Box<Expr>>, Option<Box<Expr>>),

    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
}

pub struct Body(Vec<Statement>);

pub enum Statement {
    Raw(String),
    Expr(Expr),
    Cond {
        conditional: Vec<(Expr, Body)>,
        otherwise: Box<Statement>,
    },
    Loop {
        filter: Expr,
        body: Body,
    },
    Alias {
        name: String,
        expr: Expr,
    }
}

pub struct Template {
    check: Syntax,
    body: Body,
}
