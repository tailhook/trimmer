use std::fmt;
use std::cmp::min;

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
    StStart,  // Statement start '##'
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
    top_set: RegexSet,
    top_list: Vec<(Regex, Kind)>,
    expr_set: RegexSet,
    expr_list: Vec<(Regex, Kind)>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Top,
}

#[derive(Clone)]
pub struct TokenStream<'a> {
    tok: &'a Tokenizer,
    buf: &'a str,
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

                self.update_pos(tok.value);

                match tok.kind {
                    CommentStart => unimplemented!(),
                    ExprStart => unimplemented!(),
                    StStart => unimplemented!(),
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
        match self.tok.top_set.matches(cur).into_iter().next() {
            None => {
                Token { kind: Kind::Raw, value: cur }
            }
            Some(idx) => {
                let (s, e) = self.tok.top_list[idx].0.find(cur).unwrap();
                if s == 0 {
                    Token { kind: self.tok.top_list[idx].1, value: &cur[..e] }
                } else {
                    Token { kind: Kind::Raw, value: &cur[..s] }
                }
            }
        }
    }
    fn update_pos(&mut self, val: &str) {
        self.off += val.len();
        let lines = val.as_bytes().iter().filter(|&&x| x == b'\n').count();
        self.position.line += lines as i32;
        if lines > 0 {
            let num = val[val.rfind('\n').unwrap()+1..].chars().count();
            self.position.column = num as i32;
        } else {
            self.position.column += val.chars().count() as i32;
        }
    }
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        use self::Kind::*;
        let top = &[
            (r"[ \t]+", Whitespace),
            (r"\n", Newline),
            (r"\{\{[+-]?", ExprStart),
            (r"\{#", CommentStart),
        ];
        let expr = &[
            (r"^\s+", Whitespace),
            (r"^[+-]?\}\}", ExprEnd),
            (r"^\{#", CommentStart),
            (r"^(?:and|or|not|>=|<=|==|!=|\.\.|[.|:><%*/+-])", Operator),
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
        Tokenizer {
            top_set: RegexSet::new(top.iter().map(|&(r, _)| r)).unwrap(),
            top_list: top.iter()
                .map(|&(r, k)| (Regex::new(r).unwrap(), k))
                .collect(),
            expr_set: RegexSet::new(expr.iter().map(|&(r, _)| r)).unwrap(),
            expr_list: expr.iter()
                .map(|&(r, k)| (Regex::new(r).unwrap(), k))
                .collect(),
        }
    }

    pub fn scan<'x: 'y, 'y>(&'x self, buf: &'y str) -> TokenStream<'y> {
        TokenStream {
            tok: self,
            buf: buf,
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
