//! Trimmer: A yet another text template engine
//!
#![warn(missing_docs)]

extern crate regex;
extern crate combine;
#[macro_use] extern crate quick_error;
#[cfg(feature="serde")] extern crate serde_json;

mod position;
mod tokenizer;
mod helpers;
mod grammar;
mod parse_error;
mod vars;
mod render;
mod render_error;
mod std_vars;
#[cfg(feature="serde")] mod serde;
#[cfg(test)] mod tests;


pub use position::Pos;
pub use grammar::Parser;
pub use parse_error::ParseError;
pub use vars::{Variable, Var, IntoVariable};
pub use render::Template;
pub use render_error::{RenderError, DataError};

