use std::fmt;

use regex::{Regex, RegexSet};
use combine::{StreamOnce};
use combine::primitives::{Error, Info};
use {Pos};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Kind {
    Whitespace,
    CommentStart,
    Eof,
    Raw,
    // Top level tokens
    Newline,
    ExprStart,
    ExprEnd,
    StStart,  // Statement start '## something'
    // Expression tokens
    Operator,
    Paren,
    Keyword,
    Ident,
    Number,
    String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token<'a> {
    pub kind: Kind,
    pub value: &'a str,
}

pub struct Tokenizer {
    top_scan: Regex,
    top_set: RegexSet,
    top_list: Vec<(Regex, Kind)>,
    expr_set: RegexSet,
    expr_list: Vec<(Regex, Kind)>,
    line_set: RegexSet,
    line_list: Vec<(Regex, Kind)>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Top,
    Expr,
    Statement,
}

#[derive(Clone)]
pub struct TokenStream<'a> {
    tok: &'a Tokenizer,
    buf: &'a str,
    indent: Option<usize>,
    position: Pos,
    off: usize,
    state: State,
}

impl<'a> StreamOnce for TokenStream<'a> {
    type Item = Token<'a>;
    type Range = Token<'a>;
    type Position = Pos;

    fn uncons(&mut self) -> Result<Self::Item, Error<Self::Item, Self::Range>>
    {
        use self::Kind::*;
        if self.off == self.buf.len() {
            return Ok(Token { kind: Kind::Eof, value: "" });
        }
        match self.state {
            State::Top => {
                let tok = self.match_top();

                let indent = self.update_pos(tok.value);

                match tok.kind {
                    CommentStart => unimplemented!(),
                    ExprStart => {
                        self.state = State::Expr;
                    }
                    StStart => {
                        if indent.is_none() {
                            return Err(Error::Message(
                                Info::Borrowed("Statement must start at the \
                                    beginning of the line")));
                        }
                        self.state = State::Statement;
                    }
                    _ => {}
                }
                return Ok(tok)
            }
            State::Expr => {
                let tok = self.match_expr()?;

                self.update_pos(tok.value);

                match tok.kind {
                    CommentStart => unimplemented!(),
                    ExprEnd => {
                        self.state = State::Top;
                    }
                    _ => {}
                }
                return Ok(tok)
            }
            State::Statement => {
                let tok = self.match_line_expr()?;

                self.update_pos(tok.value);

                match tok.kind {
                    CommentStart => unimplemented!(),
                    Newline => {
                        self.state = State::Top;
                    }
                    _ => {}
                }
                return Ok(tok)
            }
        }
    }

    fn position(&self) -> Self::Position {
        self.position
    }
}

impl<'a> TokenStream<'a> {
    fn match_top(&self) -> Token<'a> {
        let cur = &self.buf[self.off..];
        let matching = self.tok.top_scan.find(cur);
        match matching {
            None => {
                Token { kind: Kind::Raw, value: cur }
            }
            Some(m) => {
                if m.start() == 0 {
                    let idx = self.tok.top_set.matches(cur)
                        .into_iter().next().unwrap();
                    Token {
                        kind: self.tok.top_list[idx].1,
                        value: &cur[..m.end()]
                    }
                } else {
                    Token { kind: Kind::Raw, value: &cur[..m.start()] }
                }
            }
        }
    }
    fn match_expr(&self) -> Result<Token<'a>, Error<Token<'a>, Token<'a>>> {
        let cur = &self.buf[self.off..];
        match self.tok.expr_set.matches(cur).into_iter().next() {
            None => {
                Err(Error::Unexpected(
                    Info::Borrowed("end of file, expected `}}`")))
            }
            Some(idx) => {
                let m = self.tok.expr_list[idx].0.find(cur).unwrap();
                assert_eq!(m.start(), 0);
                Ok(Token {
                    kind: self.tok.expr_list[idx].1,
                    value: &cur[..m.end()]
                })
            }
        }
    }
    fn match_line_expr(&self)
        -> Result<Token<'a>, Error<Token<'a>, Token<'a>>>
    {
        let cur = &self.buf[self.off..];
        match self.tok.line_set.matches(cur).into_iter().next() {
            None => {
                Ok(Token {
                    kind: Kind::Newline,
                    value: "",
                })
            }
            Some(idx) => {
                let m = self.tok.line_list[idx].0.find(cur).unwrap();
                assert_eq!(m.start(), 0);
                Ok(Token {
                    kind: self.tok.line_list[idx].1,
                    value: &cur[..m.end()]
                })
            }
        }
    }
    fn update_pos(&mut self, val: &str) -> Option<usize> {
        let result = self.indent.take();
        self.off += val.len();
        let lines = val.as_bytes().iter().filter(|&&x| x == b'\n').count();
        self.position.line += lines;
        if lines > 0 {
            let line_offset = val.rfind('\n').unwrap()+1;
            let num = val[line_offset..].chars().count();
            let indent = val[line_offset..].as_bytes()
                .iter().all(|&x| x == b' ');
            if indent {
                self.indent = Some(num);
            }
            self.position.column = num;
        } else {
            self.position.column += val.chars().count();
        }
        return result;
    }
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        use self::Kind::*;
        let top = &[
            (r"\{\{[+-]?", ExprStart),
            (r"\{#", CommentStart),
            (r"##\s*(\w*)", StStart),
            (r"\n", Newline),
            (r"[ \t]+", Whitespace),
        ];
        let expr_common = &[
            (r"^[+-]?\}\}", ExprEnd),
            (r"^\{#", CommentStart),
            (r"^(?:and|or|not|>=|<=|==|!=|=|\.\.|[.|:><%*/+-])", Operator),
            (r"^[{}()\[\]]", Paren),
            ("^(?:for|in|endfor\
             |skip\
             |if|elif|else|endif\
             |let\
             |syntax|validate\
             )", Keyword),
            (r"^[a-zA-Z_][a-zA-Z0-9_]*", Ident),
            (r"^(?:0[oxb])?[0-9][0-9_]*(\.[0-9_]+)?", Number),
            (r#"^"(:?[^"]|\\")*"|^'(:?[^']|\\')*'"#, String),
        ];
        let expr = &[(r"^\s+", Whitespace)];
        let line_expr = &[
            (r"^\n", Newline),
            (r"^[ \t]+", Whitespace),
        ];
        Tokenizer {
            top_scan: Regex::new(
                &format!("(?:{})",
                    top.iter().map(|&(r, _)| format!("({})", r))
                    .collect::<Vec<_>>()
                    .join("|"))
            ).unwrap(),
            top_set: RegexSet::new(top.iter()
                .map(|&(r, _)| format!("^{}", r))
                .collect::<Vec<_>>()
                .iter()
                ).unwrap(),
            top_list: top.iter()
                .map(|&(r, k)| (Regex::new(r).unwrap(), k))
                .collect(),
            expr_set: RegexSet::new(
                expr.iter()
                .chain(expr_common.iter())
                .map(|&(r, _)| r)).unwrap(),
            expr_list: expr.iter()
                .chain(expr_common.iter())
                .map(|&(r, k)| (Regex::new(r).unwrap(), k))
                .collect(),
            line_set: RegexSet::new(
                line_expr.iter()
                .chain(expr_common.iter())
                .map(|&(r, _)| r)
                ).unwrap(),
            line_list: line_expr.iter()
                .chain(expr_common.iter())
                .map(|&(r, k)| (Regex::new(r).unwrap(), k))
                .collect(),
        }
    }

    pub fn scan<'x: 'y, 'y>(&'x self, buf: &'y str) -> TokenStream<'y> {
        TokenStream {
            tok: self,
            buf: buf,
            indent: Some(0),
            position: Pos { line: 1, column: 1 },
            off: 0,
            state: State::Top,
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}
