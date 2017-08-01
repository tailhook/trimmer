use std::marker::PhantomData;

use combine::{Parser, ConsumedResult, satisfy, skip_many};
use combine::combinator::{SkipMany, Or};
use combine::primitives::{ParseError, Error, Info};

use tokenizer::{TokenStream, Kind, Token};


#[derive(Clone)]
pub struct TokenMatch<'a> {
    kind: Kind,
    phantom: PhantomData<&'a u8>,
}

pub fn kind<'x>(kind: Kind) -> TokenMatch<'x> {
    TokenMatch {
        kind: kind,
        phantom: PhantomData,
    }
}

pub fn ws<'x>() -> SkipMany<Or<TokenMatch<'x>, TokenMatch<'x>>> {
    skip_many(TokenMatch {
        kind: Kind::Whitespace,
        phantom: PhantomData,
    }.or(TokenMatch {
        kind: Kind::Comment,
        phantom: PhantomData,
    }))
}

impl<'a> Parser for TokenMatch<'a> {
    type Input = TokenStream<'a>;
    type Output = Token<'a>;

    #[inline]
    fn parse_lazy(&mut self, input: Self::Input)
        -> ConsumedResult<Self::Output, Self::Input>
    {
        satisfy(|c: Token<'a>| c.kind == self.kind).parse_lazy(input)
    }
    fn add_error(&mut self, error: &mut ParseError<Self::Input>) {
        error.add_error(Error::Expected(Info::Owned(
            format!("{:?}", self.kind))));
    }
}

#[derive(Clone)]
pub struct Value<'a> {
    kind: Kind,
    value: &'static str,
    phantom: PhantomData<&'a u8>,
}

#[derive(Clone)]
pub struct StStart<'a> {
    keyword: &'static str,
    phantom: PhantomData<&'a u8>,
}

pub fn keyword<'x>(value: &'static str) -> Value<'x> {
    Value {
        kind: Kind::Keyword,
        value: value,
        phantom: PhantomData,
    }
}

pub fn st_start<'x>(keyword: &'static str) -> StStart<'x> {
    StStart {
        keyword: keyword,
        phantom: PhantomData,
    }
}

pub fn operator<'x>(value: &'static str) -> Value<'x> {
    Value {
        kind: Kind::Operator,
        value: value,
        phantom: PhantomData,
    }
}

pub fn paren<'x>(value: &'static str) -> Value<'x> {
    Value {
        kind: Kind::Paren,
        value: value,
        phantom: PhantomData,
    }
}

impl<'a> Parser for Value<'a> {
    type Input = TokenStream<'a>;
    type Output = Token<'a>;

    #[inline]
    fn parse_lazy(&mut self, input: Self::Input)
        -> ConsumedResult<Self::Output, Self::Input>
    {
        satisfy(|c: Token<'a>| {
            c.kind == self.kind && c.value == self.value
        }).parse_lazy(input)
    }
    fn add_error(&mut self, error: &mut ParseError<Self::Input>) {
        error.add_error(Error::Expected(Info::Borrowed(self.value)));
    }
}

impl<'a> Parser for StStart<'a> {
    type Input = TokenStream<'a>;
    type Output = Token<'a>;

    #[inline]
    fn parse_lazy(&mut self, input: Self::Input)
        -> ConsumedResult<Self::Output, Self::Input>
    {
        satisfy(|c: Token<'a>| {
            c.kind == Kind::StStart
            && c.value.split_whitespace().nth(1) == Some(self.keyword)
        }).parse_lazy(input)
    }
    fn add_error(&mut self, error: &mut ParseError<Self::Input>) {
        error.add_error(Error::Expected(Info::Borrowed(self.keyword)));
    }
}
