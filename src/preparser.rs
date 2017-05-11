use std::collections::HashMap;

use regex::{Regex, RegexSet};

use parse_error::{ParseError, ParseErrorEnum};
use validators::Validator;


pub struct Preparser {
    set: RegexSet,
    list: Vec<(Regex, Token)>,
}

#[derive(Debug)]
pub struct Options {
    pub indent: bool,
    pub default_validator: Validator,
    pub validators: HashMap<String, Validator>,
}

#[derive(Clone, Copy, Debug)]
pub enum Token {
    Syntax,
    Validate,
    Comment,
}

impl Preparser {
    pub fn new() -> Preparser {
        use self::Token::*;

        let list = &[
            (r"^##\s*syntax:\s*(\w+)(?:\n|$)", Syntax),
            (r"^##\s*validate\s+\w+:.*(?:\n|$)", Validate),
            (r"^#.*(?:\n|$)", Comment),
            (r"^###.*(?:\n|$)", Comment),
            (r"^\s*\n", Comment),
        ];
        Preparser {
            set: RegexSet::new(list.iter().map(|&(r, _)| r)).unwrap(),
            list: list.iter()
                .map(|&(r, k)| (Regex::new(r).unwrap(), k))
                .collect(),
        }
    }

    pub fn scan(&self, data: &str) -> Result<Options, ParseError> {
        let mut has_syntax = false;
        let mut options = Options::default();
        let mut cur = data;
        loop {
            match self.set.matches(cur).into_iter().next() {
                None => break,
                Some(idx) => {
                    let m = self.list[idx].0.captures(cur).unwrap();
                    cur = &cur[m.get(0).unwrap().end()..];
                    match self.list[idx].1 {
                        Token::Syntax => {
                            if has_syntax {
                                return Err(
                                    ParseErrorEnum::DuplicateSyntaxDirective
                                    .into());
                            }
                            has_syntax = true;
                            let kind = m.get(1).unwrap().as_str();
                            if kind == "indent" {
                                options.indent = true;
                            } else {
                                return Err(
                                    ParseErrorEnum::InvalidSyntaxDirective
                                    .into());
                            }
                        }
                        Token::Validate => {
                            unimplemented!();
                        }
                        Token::Comment => {
                            // Skip
                        }
                    }
                }
            }
        }
        if !options.indent {
            return Err(ParseErrorEnum::UnsupportedSyntax.into());
        }
        Ok(options)
    }
}

impl Default for Options {
    fn default() -> Options {
        Options {
            indent: false,
            default_validator: Validator::Anything,
            validators: HashMap::new(),
        }
    }
}


#[cfg(test)]
mod test {
    use validators::Validator;
    use super::Preparser;

    #[test]
    fn minimal() {
        let opt = Preparser::new().scan("## syntax: indent\n").unwrap();
        assert!(opt.indent, true);
        assert!(matches!(opt.default_validator, Validator::Anything));
        assert_eq!(opt.validators.len(), 0);
    }

    #[test]
    #[should_panic(expected="UnsupportedSyntax")]
    fn no_syntax() {
        Preparser::new().scan("").unwrap();
    }
}
