use std::collections::HashMap;

use combine::{Parser as CombineParser, ParseResult};

use regex::Regex;
use tokenizer::{Tokenizer, TokenStream, Kind};
use parse_error::ParseError;
use {Pos};


#[derive(Debug, PartialEq)]
pub struct Syntax {
    indent: bool,
    new_line_at_eof: bool,
    // parenthesis
    curly: bool,
    square: bool,
    round: bool,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub position: (Pos, Pos),
    pub code: ExprCode,
}

#[derive(Debug, PartialEq)]
pub struct Body {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum StatementCode {
    OutputRaw(String),
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

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub position: (Pos, Pos),
    pub code: StatementCode,
}

pub struct Template {
    pub check: Syntax,
    pub validators: HashMap<String, Regex>,
    pub body: Body,
}

pub struct Parser {
    tok: Tokenizer,
}

fn expression<'a>(input: TokenStream<'a>)
    -> ParseResult<Expr, TokenStream<'a>>
{
    use tokenizer::Kind::*;
    use helpers::*;
    use combine::combinator::{position};

    let expr = kind(Ident).map(|t| ExprCode::Var(t.value.into()));
    (kind(ExprStart).skip(ws()).with(position()),
        expr,
        position().skip(ws()).skip(kind(ExprEnd)))
    .map(|(s, c, e)| Expr {
        position: (s, e),
        code: c,
    })
    .parse_stream(input)
}

fn statement<'a>(input: TokenStream<'a>)
    -> ParseResult<Statement, TokenStream<'a>>
{
    use tokenizer::Kind::*;
    use helpers::*;
    use combine::combinator::{position, parser};
    use self::StatementCode::*;

    let statements =
        kind(Raw).map(|tok| OutputRaw(tok.value.to_string()))
        // Whitespace out of any blocks is output as is
        .or(parser(expression).map(Output))
        .or(kind(Whitespace).map(|tok| OutputRaw(tok.value.to_string())))
        .or(kind(Newline).map(|tok| OutputRaw(tok.value.to_string())));
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
        use helpers::kind;

        let s = self.tok.scan(data);

        let mut p = many(parser(statement)).map(|stmts| Template {
            check: Syntax::new(),  // TODO(tailhook)
            validators: HashMap::new(),  // TODO(tailhook)
            body: Body { statements: stmts },
        }).skip(kind(Kind::Eof));

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
            new_line_at_eof: true,
            curly: false,
            square: false,
            round: false,
        }
    }
}
