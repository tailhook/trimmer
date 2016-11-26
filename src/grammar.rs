use std::collections::HashMap;

use combine::{Parser as CombineParser, ParseResult};

use regex::Regex;
use tokenizer::{Tokenizer, TokenStream, Kind};
use parse_error::ParseError;
use {Pos};


pub struct Syntax {
    indent: bool,
    curly: bool,
    square: bool,
    round: bool,
}

pub enum ExprCode {
    // Constants
    Str(String),
    Int(i64),
    Float(f64),
    // Vars
    Var(String),
    Attr(Box<Expr>, String),
    Item(Box<Expr>, Box<Expr>),
    // Special
    Filter(Box<Expr>, Box<Expr>),  // pipe operator
    // Booleans
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Neq(Box<Expr>, Box<Expr>),
    LessEq(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    GreaterEq(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    // Constructors
    List(Vec<Expr>),
    Dict(Vec<(Expr, Expr)>),
    Range(Option<Box<Expr>>, Option<Box<Expr>>),
    // Math
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
}

pub struct Expr {
    position: (Pos, Pos),
    code: ExprCode,
}

pub struct Body(Vec<Statement>);

pub enum StatementCode {
    Raw(String),
    Output(Expr),
    Cond {
        conditional: Vec<(Expr, Body)>,
        otherwise: Body,
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

pub struct Statement {
    position: (Pos, Pos),
    code: StatementCode,
}

pub struct Template {
    check: Syntax,
    validators: HashMap<String, Regex>,
    body: Body,
}

pub struct Parser {
    tok: Tokenizer,
}

fn statement<'a>(input: TokenStream<'a>)
    -> ParseResult<Statement, TokenStream<'a>>
{
    use tokenizer::Kind::*;
    use helpers::*;
    use combine::combinator::position;
    let statements =
        kind(String).map(|tok| StatementCode::Raw(tok.value.to_string()));
    (position(), statements, position()).map(|(s, c, e)| Statement {
        position: (s, e),
        code: c,
    }).parse_stream(input)
}

impl Parser {
    /// Create a new parser
    ///
    /// Note: it's expensive to create a new parser (i.e. it compiles regular
    /// expressions) so you should reuse the parser instance if you're going
    /// to compile multiple templates.
    pub fn new() -> Parser {
        Parser {
            tok: Tokenizer::new(),
        }
    }
    pub fn parse(&self, data: &str) -> Result<Template, ParseError> {
        use combine::combinator::*;
        let s = self.tok.scan(data);

        let mut p = many(parser(statement)).map(|stmts| Template {
            check: Syntax::new(),  // TODO(tailhook)
            validators: HashMap::new(),  // TODO(tailhook)
            body: Body(stmts),
        });

        let (template, _) = p.parse(s)?;
        // TODO(tailhook) should we assert EOF?
        // TODO(tailhook) execute checks
        return Ok(template);
    }
}

impl Syntax {
    fn new() -> Syntax {
        Syntax {
            indent: false,
            curly: false,
            square: false,
            round: false,
        }
    }
}
