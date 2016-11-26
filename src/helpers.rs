use std::marker::PhantomData;

use combine::{Stream, Parser, ConsumedResult, satisfy};
use combine::primitives::{Error, ParseError, Info};

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

impl<'a> Parser for TokenMatch<'a> {
    type Input = TokenStream<'a>;
    type Output = Token<'a>;

    #[inline]
    fn parse_lazy(&mut self, input: Self::Input)
        -> ConsumedResult<Self::Output, Self::Input>
    {
        satisfy(|c: Token<'a>| c.kind == self.kind).parse_lazy(input)
    }
}

#[derive(Clone)]
pub struct Value<'a> {
    kind: Kind,
    value: &'static str,
    phantom: PhantomData<&'a u8>,
}

pub fn keyword<'x>(value: &'static str) -> Value<'x> {
    Value {
        kind: Kind::Keyword,
        value: value,
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
}
