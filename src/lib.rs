#![warn(missing_docs)]

extern crate regex;
extern crate combine;
#[macro_use] extern crate quick_error;

mod tokenizer;
mod helpers;
mod grammar;
mod parse_error;
#[cfg(test)] mod tests;


pub use combine::primitives::SourcePosition as Pos;

pub use grammar::Parser;
pub use parse_error::ParseError;

pub struct Template(grammar::Template);
