use std::collections::HashMap;

use combine::{Parser as CombineParser, ParseResult};
use combine::combinator::{position, parser, many};

use oneline;
use parse_error::ParseError;
use preparser::{Preparser, Options, Syntax};
use regex::Regex;
use render::{self, template};
use tokenizer::{Tokenizer, TokenStream, Token, Kind};
use {Pos};


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
        target: AssignTarget,
        iterator: Expr,
        filter: Option<Expr>,
        body: Body,
    },
    Alias {
        target: AssignTarget,
        value: Expr,
    }
}

#[derive(Debug, PartialEq)]
pub enum AssignTarget {
    Var(String),
}

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub position: (Pos, Pos),
    pub code: StatementCode,
}

#[derive(Debug)]
pub struct Template {
    pub options: Options,
    pub body: Body,
}

/// A reusable parser
///
/// Instance of this class must (and should) be reused for compiling multiple
/// templates
pub struct Parser {
    pre: Preparser,
    tok: Tokenizer,
    oneline_post: oneline::Postprocess,
}

// TODO(tailhook) allow escaping errors
// TODO(tailhook) implement hex escapes
fn parse_str(val: &str) -> String {
    let mut escape = false;
    let mut res = String::new();

    for ch in val[1..val.len()-1].chars() {
        if escape {
            match ch {
                '"' => res.push('"'),
                '\\' => res.push('\\'),
                '/' => res.push('/'),
                'b' => res.push('\x08'),
                'f' => res.push('\x0c'),
                'n' => res.push('\n'),
                'r' => res.push('\r'),
                't' => res.push('\t'),
                ch => res.push(ch),
            }
            escape = false;
        } else if ch == '\\' {
            escape = true;
        } else {
            res.push(ch);
        }
    }
    return res;
}

fn atom<'a>(input: TokenStream<'a>)
    -> ParseResult<Expr, TokenStream<'a>>
{
    use tokenizer::Kind::*;
    use helpers::*;
    use self::ExprCode::*;

    let expr =
        kind(Ident).map(|t| Var(t.value.into()))
        .or(kind(String).map(|t| Str(parse_str(t.value))));
    (position(), expr, position())
    .map(|(s, c, e)| Expr {
        position: (s, e),
        code: c,
    })
    .parse_stream(input)
}

fn attr<'a>(input: TokenStream<'a>)
    -> ParseResult<Expr, TokenStream<'a>>
{
    use tokenizer::Kind::*;
    use helpers::*;
    parser(atom)
    .and(many(kind(Operator).with(kind(Ident)).and(position())))
    .map(|(atom, vec):(_, Vec<_>)| {
        vec.into_iter().fold(atom, |expr: Expr, (ident, e): (Token<'a>, _)| {
            Expr {
                position: (expr.position.0, e),
                code: ExprCode::Attr(Box::new(expr),
                                     ident.value.to_string())
            }
        })
    })
    .parse_stream(input)
}
fn top_level_expression<'a>(input: TokenStream<'a>)
    -> ParseResult<Expr, TokenStream<'a>>
{
    attr(input)
}

fn expression<'a>(input: TokenStream<'a>)
    -> ParseResult<Expr, TokenStream<'a>>
{
    use tokenizer::Kind::*;
    use helpers::*;

    kind(ExprStart).and(ws())
        .with(parser(top_level_expression))
        .skip(ws()).skip(kind(ExprEnd))
    .parse_stream(input)
}

fn assign_target<'a>(input: TokenStream<'a>)
    -> ParseResult<AssignTarget, TokenStream<'a>>
{
    use tokenizer::Kind::*;
    use helpers::*;

    kind(Ident)
    .map(|t| AssignTarget::Var(t.value.to_string()))
    .parse_stream(input)
}

fn if_stmt<'a>(input: TokenStream<'a>)
    -> ParseResult<StatementCode, TokenStream<'a>>
{
    use tokenizer::Kind::*;
    use self::StatementCode::*;
    use helpers::*;

    st_start("if")
        .skip(ws())
        .with(parser(top_level_expression))
        .skip(ws())
        .skip(kind(Newline))
    .and(parser(body))
    .skip(st_start("endif")).skip(ws()).skip(kind(Newline))
    .map(|(condition, block)| {
        Cond {
            conditional: vec![
                (condition, block),
            ],
            otherwise: Body {
                statements: Vec::new(),
            }
        }
    })
    .parse_stream(input)
}
fn for_stmt<'a>(input: TokenStream<'a>)
    -> ParseResult<StatementCode, TokenStream<'a>>
{
    use tokenizer::Kind::*;
    use self::StatementCode::*;
    use helpers::*;

    st_start("for")
        .skip(ws())
        .with(parser(assign_target))
        .skip(ws())
        .skip(keyword("in"))
        .skip(ws())
        .and(parser(top_level_expression))
        .skip(ws())
        .skip(kind(Newline))
    .and(parser(body))
    .skip(st_start("endfor")).skip(ws()).skip(kind(Newline))
    .map(|((target, list), block)| {
        Loop {
            target: target,
            iterator: list,
            filter: None,
            body: block,
        }
    })
    .parse_stream(input)
}

fn let_stmt<'a>(input: TokenStream<'a>)
    -> ParseResult<StatementCode, TokenStream<'a>>
{
    use tokenizer::Kind::*;
    use self::StatementCode::*;
    use helpers::*;

    st_start("let")
        .skip(ws())
        .with(parser(assign_target))
        .skip(ws())
        .skip(operator("="))
        .skip(ws())
        .and(parser(top_level_expression))
        .skip(ws())
        .skip(kind(Newline))
    .map(|(target, value)| Alias { target, value })
    .parse_stream(input)
}

fn block<'a>(input: TokenStream<'a>)
    -> ParseResult<StatementCode, TokenStream<'a>>
{
    parser(if_stmt)
    .or(parser(for_stmt))
    .or(parser(let_stmt))
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
        .or(parser(expression).map(Output))
        .or(parser(block))
        // Whitespace out of any blocks is output as is
        .or(kind(Whitespace).map(|tok| OutputRaw(tok.value.to_string())))
        .or(kind(Newline).map(|tok| OutputRaw(tok.value.to_string())));
    (position(), statements, position()).map(|(s, c, e)| Statement {
        position: (s, e),
        code: c,
    }).parse_stream(input)
}

fn body<'a>(input: TokenStream<'a>)
    -> ParseResult<Body, TokenStream<'a>>
{
    many(parser(statement)).map(|x| {
        Body {
            statements: optimize_statements(x),
        }
    }).parse_stream(input)
}

pub fn optimize_statements(src: Vec<Statement>) -> Vec<Statement> {
    use self::StatementCode::OutputRaw;
    let mut dst = Vec::with_capacity(src.len());
    for item in src.into_iter() {
        match (&item, dst.last_mut()) {
            (
                &Statement {
                    position: (_, new_end),
                    code: OutputRaw(ref next),
                },
                Some(&mut Statement {
                    position: (s, ref mut old_end),
                    code: OutputRaw(ref mut prev),
                })
            ) => {
                *old_end = new_end;
                prev.push_str(next);
                continue;
            }
            _ => {}
        }
        dst.push(item);
    }
    return dst;
}

impl Parser {
    /// Create a new parser
    ///
    /// Note: it's expensive to create a new parser (i.e. it compiles regular
    /// expressions) so you should reuse the parser instance if you're going
    /// to compile multiple templates.
    pub fn new() -> Parser {
        Parser {
            pre: Preparser::new(),
            tok: Tokenizer::new(),
            oneline_post: oneline::Postprocess::new(),
        }
    }
    /// Parse and compile a template
    pub fn parse(&self, data: &str) -> Result<render::Template, ParseError> {
        use combine::combinator::{skip_many, parser, satisfy};
        use tokenizer::Kind::Newline;
        use helpers::{kind, st_start};

        let options = self.pre.scan(data)?;
        let s = self.tok.scan(data);

        let mut p =
            skip_many(
                st_start("syntax")
                .skip(skip_many(satisfy(|x: Token| x.kind != Newline)))
                .skip(kind(Newline)))
            .with(parser(body)).skip(kind(Kind::Eof));

        let (body, _) = p.parse(s)?;
        let body = match options.syntax {
            Syntax::Oneline => {
                self.oneline_post.process(&options, body)
            }
            Syntax::Indent => {
                body
            }
            Syntax::Plain => body,
        };
        let tpl = Template {
            options: options,
            body: body,
        };
        // TODO(tailhook) should we assert EOF?
        // TODO(tailhook) execute checks
        return Ok(template(tpl));
    }
}
