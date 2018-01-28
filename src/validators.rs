use regex::Regex;
use std::str::FromStr;

use escape;
use parse_error::{ParseError, ParseErrorEnum};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltinEscape {
    HtmlEntities,
    QuotedShellArgument,
}

#[derive(Debug, Clone)]
pub enum Filter {
    NoFilter,
    Validate(Regex),
    Escape(BuiltinEscape),
}

impl PartialEq for Filter {
    fn eq(&self, other: &Filter) -> bool {
        use self::Filter::*;
        match (self, other) {
            (&NoFilter, &NoFilter) => true,
            (&Validate(ref a), &Validate(ref b)) => a.as_str() == b.as_str(),
            (&Escape(ref a), &Escape(ref b)) => a == b,
            (&NoFilter, _) => false,
            (&Validate(..), _) => false,
            (&Escape(..), _) => false,
        }
    }
}

impl FromStr for BuiltinEscape {
    type Err = ParseError;
    fn from_str(val: &str) -> Result<Self, ParseError> {
        use self::BuiltinEscape::*;
        match val {
            "builtin.html_entities" => Ok(HtmlEntities),
            "builtin.quoted_shell_argument" => Ok(QuotedShellArgument),
            _ => Err(ParseErrorEnum::BadFilter(val.to_string()).into()),
        }
    }
}

impl BuiltinEscape {
    pub fn escape(&self, dest: &mut String, src: &str) {
        use self::BuiltinEscape::*;
        match *self {
            HtmlEntities => escape::html_entities(dest, src),
            QuotedShellArgument => escape::quoted_shell_argument(dest, src),
        }
    }
}
