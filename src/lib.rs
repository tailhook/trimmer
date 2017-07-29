//! Trimmer: A yet another text template engine
//!
#![warn(missing_docs)]

extern crate combine;
extern crate owning_ref;
extern crate regex;
#[macro_use] extern crate quick_error;
#[cfg(test)] #[macro_use] extern crate matches;
#[cfg(feature="serde")] extern crate serde_json;

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
mod varmap;
mod owning;
mod oneline;
#[cfg(feature="serde")] mod serde;
#[cfg(test)] mod tests;


pub use grammar::Parser;
pub use parse_error::ParseError;
pub use position::Pos;
pub use render_error::{RenderError, DataError};
pub use render::Template;
pub use vars::{Variable, Var};
pub use varmap::Context;

