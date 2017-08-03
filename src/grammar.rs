use combine::{Parser as CombineParser, ParseResult};
use combine::combinator::{position, parser, many, optional, skip_many};

use indent;
use oneline;
use optimize;
use parse_error::ParseError;
use preparser::{Preparser, Syntax};
use render::{self, template};
use tokenizer::{Tokenizer, TokenStream, Token, Kind};
use {Options, Pos};


#[derive(Debug, PartialEq)]
#[allow(dead_code)] // TODO(tailhook)
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
        indent: usize,
        conditional: Vec<(Expr, Body)>,
        otherwise: Body,
    },
    Loop {
        indent: usize,
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
    Pair(String, String),
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
    optimizer: optimize::Optimizer,
    indent_post: indent::Postprocess,
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
        .or(kind(String).map(|t| Str(parse_str(t.value))))
        .or(kind(Number).map(|t| {
            t.value.parse().map(Int)
                    .unwrap_or_else(|_| Float(t.value.parse().unwrap()))
        }));
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

    enum Suffix<'a> {
        Attr(Token<'a>),
        Item(Expr),
    }

    parser(atom)
    .and(many(
        operator(".").with(kind(Ident)).map(Suffix::Attr).or(
            paren("[")
            .with(parser(top_level_expression))
            .skip(paren("]"))
            .map(Suffix::Item)
        ).and(position())))
    .map(|(atom, vec): (_, Vec<_>)| {
        vec.into_iter().fold(atom,
        |expr: Expr, (suffix, e): (Suffix<'a>, _)| {
            Expr {
                position: (expr.position.0, e),
                code: match suffix {
                    Suffix::Attr(ident) => {
                        ExprCode::Attr(Box::new(expr), ident.value.to_string())
                    }
                    Suffix::Item(item) => {
                        ExprCode::Item(Box::new(expr), Box::new(item))
                    }
                },
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

fn for_target<'a>(input: TokenStream<'a>)
    -> ParseResult<AssignTarget, TokenStream<'a>>
{
    use tokenizer::Kind::*;
    use helpers::*;

    kind(Ident)
    .skip(ws())
    .and(optional(operator(",").skip(ws()).with(kind(Ident)).skip(ws())))
    .map(|(a, b)| {
        b.map(|b| AssignTarget::Pair(a.value.to_string(), b.value.to_string()))
        .unwrap_or_else(|| AssignTarget::Var(a.value.to_string()))
    })
    .parse_stream(input)
}

fn let_target<'a>(input: TokenStream<'a>)
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
        .and(parser(top_level_expression))
        .skip(ws())
        .skip(kind(Newline))
    .and(parser(body))
    .and(many::<Vec<_>, _>(
        st_start("elif")
        .skip(ws())
        .with(parser(top_level_expression))
        .skip(kind(Newline))
        .and(parser(body))))
    .and(optional(
        st_start("else").skip(ws()).skip(kind(Newline))
        .with(parser(body))))
    .skip(st_start("endif")).skip(ws()).skip(kind(Newline))
    .map(|((((if_token, condition), block), mut elifs), else_block)| {
        Cond {
            indent: if_token.value.len() - if_token.value.trim_left().len(),
            conditional: {
                elifs.insert(0, (condition, block));
                elifs
            },
            otherwise: else_block.unwrap_or_else(|| Body {
                statements: Vec::new(),
            }),
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
        .and(parser(for_target))
        .skip(keyword("in"))
        .skip(ws())
        .and(parser(top_level_expression))
        .skip(ws())
        .skip(kind(Newline))
    .and(parser(body))
    .skip(st_start("endfor")).skip(ws()).skip(kind(Newline))
    .map(|(((for_token, target), list), block)| {
        Loop {
            indent: for_token.value.len() - for_token.value.trim_left().len(),
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
        .with(parser(let_target))
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
    use tokenizer::Kind::Comment;
    use helpers::kind;

    optional(skip_many(kind(Comment)))
    .with(many(parser(statement).skip(skip_many(kind(Comment))))).map(|x| {
        Body {
            statements: x,
        }
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
            pre: Preparser::new(),
            tok: Tokenizer::new(),
            optimizer: optimize::Optimizer::new(),
            oneline_post: oneline::Postprocess::new(),
            indent_post: indent::Postprocess::new(),
        }
    }
    /// Parse and compile a template
    pub fn parse(&self, data: &str) -> Result<render::Template, ParseError> {
        self.parse_with_options(&Options::new(), data)
    }
    /// Parse and compile a template with some predefined options set
    pub fn parse_with_options(&self, options: &Options, data: &str)
        -> Result<render::Template, ParseError>
    {
        use combine::combinator::{skip_many, parser, satisfy};
        use tokenizer::Kind::Newline;
        use helpers::{kind, st_start};

        let options = self.pre.scan(data, options.clone())?;
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
                // easier after optimizer
                let body = self.optimizer.optimize(&options, body);
                self.oneline_post.process(&options, body)
            }
            Syntax::Indent => {
                // easier before optimizer
                let body = self.indent_post.process(&options, body)?;
                self.optimizer.optimize(&options, body)
            }
            Syntax::Plain => {
                self.optimizer.optimize(&options, body)
            }
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
