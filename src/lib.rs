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
mod render;
mod render_error;
mod std_vars;
#[cfg(test)] mod tests;


pub use position::Pos;
pub use grammar::Parser;
pub use parse_error::ParseError;
pub use vars::{Context, Variable, Var, IntoVariable};
pub use render::Template;
pub use render_error::{RenderError, DataError, ErrorTracker};
