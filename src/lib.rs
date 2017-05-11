//! Trimmer: A yet another text template engine
//!
#![warn(missing_docs)]

extern crate combine;
extern crate regex;
extern crate typed_arena;
#[macro_use] extern crate quick_error;
#[cfg(test)] #[macro_use] extern crate matches;
#[cfg(feature="serde")] extern crate serde_json;

mod context;
mod grammar;
mod helpers;
mod parse_error;
mod position;
mod preparser;
mod render;
mod render_error;
mod std_vars;
mod tokenizer;
mod validators;
mod vars;
#[cfg(feature="serde")] mod serde;
#[cfg(test)] mod tests;


pub use context::Context;
pub use grammar::Parser;
pub use parse_error::ParseError;
pub use position::Pos;
pub use preparser::Options;
pub use render_error::{RenderError, DataError};
pub use render::Template;
pub use validators::Validator;
pub use vars::Variable;

