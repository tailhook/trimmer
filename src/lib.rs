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
mod indent;
mod oneline;
mod optimize;
mod options;
mod owning;
mod parse_error;
mod position;
mod preparser;
mod render;
mod render_error;
mod std_vars;
mod tokenizer;
mod validators;
mod varmap;
mod vars;
#[cfg(feature="serde")] mod serde;
#[cfg(test)] mod tests;

pub use grammar::Parser;
pub use parse_error::ParseError;
pub use position::Pos;
pub use render_error::{RenderError, DataError};
pub use render::Template;
pub use vars::{Variable};
pub use varmap::Context;

use std::collections::HashMap;

use vars::Val;

#[derive(Debug, Clone)]
/// Options of the template
///
/// Usually all options are set in the template itself using
/// `## syntax ...` and `## validator...` directives, but this object
/// can be prefilled with better default that suit your application. For
/// example, if you use template for a log message it's good idea to use
/// `## syntax: oneline` but it's tedious to write it every time.
pub struct Options {
    syntax: preparser::Syntax,
    new_line_at_eof: Option<bool>,
    // parenthesis
    curly: bool,
    square: bool,
    round: bool,

    default_validator: validators::Validator,
    validators: HashMap<String, validators::Validator>,
}

/// Variable reference returned from methods of Variable trait
///
/// It can contain borrowed reference from current variable or
/// owned (reference counted) box to another object
pub struct Var<'a, 'render: 'a>(Val<'a, 'render>);
