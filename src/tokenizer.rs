use regex::RegexSet;
use combine::{StreamOnce};
use combine::primitives::{SourcePosition, Error};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Kind {
    Whitespace,
    CommentStart,
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
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token<'a> {
    kind: Kind,
    value: &'a str,
}

pub struct Tokenizer {
    top: RegexSet,
    expr: RegexSet,
}

#[derive(Clone)]
pub struct TokenStream<'a> {
    tok: &'a Tokenizer,
    buf: &'a str,
    position: SourcePosition,
    off: usize,
}

impl<'a> StreamOnce for TokenStream<'a> {
    type Item = Token<'a>;
    type Range = Token<'a>;
    type Position = SourcePosition;
    fn uncons(&mut self) -> Result<Self::Item, Error<Self::Item, Self::Range>>
    {
        unimplemented!();
    }
    fn position(&self) -> Self::Position {
        self.position
    }
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            top: RegexSet::new(&[
                r"^[ \t]+",  // Whitespace
                r"^\n",      // Newline
                r"^\{\{[+-]?",    // ExprStart
                r"^\{#",     // CommentStart
            ]).unwrap(),
            expr: RegexSet::new(&[
                r"^\s+",      // Whitespace
                r"^[+-]?\}\}",     // ExprEnd
                r"^\{#",      // CommentStart
                r"^and|or|not|>=|<=|==|!=|\.\.|[.|:><%*/+-]",  // Operator
                r"^[{}()\[\]]",  // Parenthesis
                r"for|in|endfor\
                 |skip\
                 |if|elif|else|endif\
                 |let\
                 |syntax\
                 ",           // Keyword
                r"^[a-zA-Z_][a-zA-Z0-9_]*",  // Ident
                r"(?:0[oxb])?[0-9][0-9_]*(\.[0-9_]+)?",  // Number
                r#""#,  // String
            ]).unwrap(),
        }
    }

    pub fn parse<'x: 'y, 'y>(&'x self, buf: &'y str) -> TokenStream<'y> {
        TokenStream {
            tok: self,
            buf: buf,
            position: SourcePosition { line: 1, column: 1 },
            off: 0,
        }
    }
}
