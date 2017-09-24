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


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CmpOperator {
    Eq,
    Neq,
    LessEq,
    Less,
    GreaterEq,
    Greater,
}

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
    Comparison(Box<Expr>, Vec<(CmpOperator, Expr)>),
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

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Ord, Eq)]
pub enum OutputMode {
    Strip,
    Space,
    Preserve,
}

#[derive(Debug, PartialEq)]
pub enum StatementCode {
    OutputRaw(String),
    Output {
        left_ws: OutputMode,
        expr: Expr,
        validator: Option<String>,
        right_ws: OutputMode,
    },
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
    .skip(ws())
    .map(|(s, c, e)| Expr {
        position: (s, e),
        code: c,
    })
    .or(paren("(")
        .skip(ws())
        .with(parser(top_level_expression))
        .skip(paren(")")))
        .skip(ws())
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
        operator(".").skip(ws()).with(kind(Ident)).skip(ws()).map(Suffix::Attr)
        .or(
            paren("[")
            .skip(ws())
            .with(parser(top_level_expression))
            .skip(paren("]"))
            .skip(ws())
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

fn unary<'a>(input: TokenStream<'a>)
    -> ParseResult<Expr, TokenStream<'a>>
{
    use helpers::{operator, ws};
    use self::ExprCode::*;

    let expr = optional(operator("not").skip(ws())).and(parser(attr));

    (position(), expr, position())
    .map(|(s, (op, attr), e)| {
        if let Some(op) = op {
            Expr {
                position: (s, e),
                code: match op.value {
                    "not" => Not(Box::new(attr)),
                    _ => unreachable!(),
                },
            }
        } else {
            attr
        }
    })
    .parse_stream(input)
}

fn factors<'a>(input: TokenStream<'a>)
    -> ParseResult<Expr, TokenStream<'a>>
{
    use helpers::*;

    enum Op {
        Mul,
        Div,
        Mod,
    }

    parser(unary)
    .and(many(
        operator("*").map(|_| Op::Mul)
        .or(operator("/").map(|_| Op::Div))
        .or(operator("%").map(|_| Op::Mod))
        .skip(ws())
        .and(parser(unary))
        .and(position())))
    .map(|(expr, vec): (_, Vec<_>)| {
        vec.into_iter().fold(expr,
        |a: Expr, ((op, b), e): ((_, Expr), _)| {
            Expr {
                position: (a.position.0, e),
                code: match op {
                    Op::Mul => ExprCode::Mul(Box::new(a), Box::new(b)),
                    Op::Div => ExprCode::Div(Box::new(a), Box::new(b)),
                    Op::Mod => ExprCode::Mod(Box::new(a), Box::new(b)),
                },
            }
        })
    })
    .parse_stream(input)
}


fn addition<'a>(input: TokenStream<'a>)
    -> ParseResult<Expr, TokenStream<'a>>
{
    use helpers::*;

    enum Op {
        Add,
        Sub,
    }

    parser(factors)
    .and(many(
        operator("+").map(|_| Op::Add).or(operator("-").map(|_| Op::Sub))
        .skip(ws())
        .and(parser(factors))
        .and(position())))
    .map(|(expr, vec): (_, Vec<_>)| {
        vec.into_iter().fold(expr,
        |a: Expr, ((op, b), e): ((_, Expr), _)| {
            Expr {
                position: (a.position.0, e),
                code: match op {
                    Op::Add => ExprCode::Add(Box::new(a), Box::new(b)),
                    Op::Sub => ExprCode::Sub(Box::new(a), Box::new(b)),
                },
            }
        })
    })
    .parse_stream(input)
}

fn comparison<'a>(input: TokenStream<'a>)
    -> ParseResult<Expr, TokenStream<'a>>
{
    use helpers::*;
    use self::CmpOperator::*;

    parser(addition)
    .and(many(
        operator("==").map(|_| Eq)
        .or(operator("!=").map(|_| Neq))
        .or(operator(">").map(|_| Greater))
        .or(operator(">=").map(|_| GreaterEq))
        .or(operator("<").map(|_| Less))
        .or(operator("<=").map(|_| LessEq))
        .skip(ws())
        .and(parser(addition))))
    .and(position())
    .map(|((expr, vec), e): ((Expr, Vec<_>), _)|  {
        if vec.len() == 0 {
            expr
        } else {
            Expr {
                position: (expr.position.0, e),
                code: ExprCode::Comparison(Box::new(expr), vec),
            }
        }
    })
    .parse_stream(input)
}

fn bool_and<'a>(input: TokenStream<'a>)
    -> ParseResult<Expr, TokenStream<'a>>
{
    use helpers::*;

    parser(comparison)
    .and(many(
        operator("and")
        .skip(ws())
        .with(parser(comparison))
        .and(position())))
    .map(|(expr, vec): (_, Vec<_>)| {
        vec.into_iter().fold(expr,
        |a: Expr, (b, e): (Expr, _)| {
            Expr {
                position: (a.position.0, e),
                code: ExprCode::And(Box::new(a), Box::new(b)),
            }
        })
    })
    .parse_stream(input)
}

fn bool_or<'a>(input: TokenStream<'a>)
    -> ParseResult<Expr, TokenStream<'a>>
{
    use helpers::*;

    parser(bool_and)
    .and(many(
        operator("or")
        .skip(ws())
        .with(parser(bool_and))
        .and(position())))
    .map(|(expr, vec): (_, Vec<_>)| {
        vec.into_iter().fold(expr,
        |a: Expr, (b, e): (Expr, _)| {
            Expr {
                position: (a.position.0, e),
                code: ExprCode::Or(Box::new(a), Box::new(b)),
            }
        })
    })
    .parse_stream(input)
}

fn top_level_expression<'a>(input: TokenStream<'a>)
    -> ParseResult<Expr, TokenStream<'a>>
{
    parser(bool_or)
    .parse_stream(input)
}

fn expression<'a>(input: TokenStream<'a>)
    -> ParseResult<StatementCode, TokenStream<'a>>
{
    use tokenizer::Kind::*;
    use helpers::*;

    kind(ExprStart).skip(ws())
        .and(parser(top_level_expression)).skip(ws())
        .and(optional(operator("|").skip(ws()).with(kind(Ident))))
        .skip(ws()).and(kind(ExprEnd))
    .map(|(((start, expr), validator), end)| {
        let left_ws = OutputMode::start(&start);
        let right_ws = OutputMode::end(&end);
        // TODO(tailhook) parse validator
        StatementCode::Output { left_ws, expr,
            validator: validator.map(|x| x.value.to_string()), right_ws }
    })
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
        .or(parser(expression))
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

impl OutputMode {
    fn start(tok: &Token) -> OutputMode {
        if tok.value.ends_with("+") {
            OutputMode::Space
        } else if tok.value.ends_with("-") {
            OutputMode::Strip
        } else {
            OutputMode::Preserve
        }
    }
    fn end(tok: &Token) -> OutputMode {
        if tok.value.starts_with("+") {
            OutputMode::Space
        } else if tok.value.starts_with("-") {
            OutputMode::Strip
        } else {
            OutputMode::Preserve
        }
    }
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
                .skip(kind(Newline))
                .or(st_start("validate")
                .skip(skip_many(satisfy(|x: Token| x.kind != Newline)))
                .skip(kind(Newline))))
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
