//! Trimmer: A yet another text template engine
//!
#![warn(missing_docs)]

extern crate combine;
extern crate regex;
extern crate typed_arena;
#[macro_use] extern crate quick_error;
#[cfg(feature="serde")] extern crate serde_json;

mod context;
mod grammar;
mod helpers;
mod parse_error;
mod position;
mod render;
mod render_error;
mod std_vars;
mod tokenizer;
mod vars;
#[cfg(feature="serde")] mod serde;
#[cfg(test)] mod tests;


pub use position::Pos;
pub use grammar::Parser;
pub use parse_error::ParseError;
pub use vars::Variable;
pub use context::Context;
pub use render::Template;
pub use render_error::{RenderError, DataError};

