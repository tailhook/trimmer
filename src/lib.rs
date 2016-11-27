//! Trimmer: A yet another text template engine
//!
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
mod vars;
#[cfg(test)] mod tests;


pub use position::Pos;
pub use grammar::Parser;
pub use parse_error::ParseError;
pub use vars::Context;

/// A parsed template code that can be rendered
pub struct Template(grammar::Template);
