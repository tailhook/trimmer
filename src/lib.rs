#![warn(missing_docs)]

extern crate regex;
extern crate combine;
#[macro_use] extern crate quick_error;
#[macro_use] extern crate difference;

mod position;
mod tokenizer;
mod helpers;
mod grammar;
mod parse_error;
#[cfg(test)] mod tests;


pub use position::Pos;
pub use grammar::Parser;
pub use parse_error::ParseError;

/// A parsed template code that can be rendered
pub struct Template(grammar::Template);
