use regex::{Regex, RegexSet};

use parse_error::{ParseError, ParseErrorEnum};
use validators::Filter;
use {Options};


pub struct Preparser {
    set: RegexSet,
    list: Vec<(Regex, Token)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Syntax {
    Plain,
    Indent,
    Oneline,
}

#[derive(Clone, Copy, Debug)]
pub enum Token {
    Syntax,
    Validate,
    Filter,
    Comment,
}

impl Preparser {
    pub fn new() -> Preparser {
        use self::Token::*;

        let list = &[
            (r"^##\s*syntax:\s*(\w+)(?:\n|$)", Syntax),
            (r"^##\s*validate\s+(\w+):[ \t]*(.*)\s*(?:\n|$)",
                Validate),
            (r"^##\s*filter\s+(\w+):[ \t]*(.*)\s*(?:\n|$)",
                Filter),
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

    pub fn scan(&self, data: &str, defaults: Options)
        -> Result<Options, ParseError>
    {
        let mut options = defaults;
        let mut has_syntax = false;
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
                                options.syntax = Syntax::Indent;
                            } else if kind == "oneline" {
                                options.syntax = Syntax::Oneline;
                            } else {
                                return Err(
                                    ParseErrorEnum::InvalidSyntaxDirective
                                    .into());
                            }
                        }
                        Token::Validate => {
                            let name = m.get(1).unwrap().as_str();
                            let mut regex = m.get(2).unwrap()
                                .as_str().to_string();
                            // Strip comment
                            if let Some(end) = regex.as_bytes()
                                .iter().position(|&x| x == b'#')
                            {
                                if end > 0 && regex[..end].ends_with(" ") {
                                    let nlen = regex[..end].trim().len();
                                    regex.truncate(nlen);
                                }
                            }
                            // add anchors if not there
                            if !regex.starts_with("^") {
                                regex.insert(0, '^');
                            }
                            if !regex.ends_with("$") {
                                regex.push('$');
                            }
                            let regex = Regex::new(&regex)
                                .map_err(|e| ParseErrorEnum::BadRegexValidator(
                                    regex.to_string(), e))?;
                            if name == "default" {
                                options.default_filter =
                                    Filter::Validate(regex);
                            } else {
                                options.filters.insert(
                                    name.to_string(), Filter::Validate(regex));
                            }
                        }
                        Token::Filter => {
                            let name = m.get(1).unwrap().as_str();
                            let filter = m.get(2).unwrap().as_str().parse()?;
                            if name == "default" {
                                options.default_filter =
                                    Filter::Escape(filter);
                            } else {
                                options.filters.insert(name.to_string(),
                                    Filter::Escape(filter));
                            }
                        }
                        Token::Comment => {
                            // Skip
                        }
                    }
                }
            }
        }
        Ok(options)
    }
}


#[cfg(test)]
mod test {
    use validators::Filter;
    use super::{Preparser, Syntax};
    use {Options};

    #[test]
    fn indent() {
        let opt = Preparser::new().scan("## syntax: indent\n",
            Options::new().clone()).unwrap();
        assert_eq!(opt.syntax, Syntax::Indent);
        assert!(matches!(opt.default_filter, Filter::NoFilter));
        assert_eq!(opt.filters.len(), 0);
    }

    #[test]
    fn oneline() {
        let opt = Preparser::new().scan("## syntax: oneline\n",
            Options::new().clone()).unwrap();
        assert_eq!(opt.syntax, Syntax::Oneline);
        assert!(matches!(opt.default_filter, Filter::NoFilter));
        assert_eq!(opt.filters.len(), 0);
    }

    #[test]
    fn default_oneline_approve() {
        let opt = Preparser::new().scan("## syntax: oneline\n",
            Options::new().syntax_oneline().clone()).unwrap();
        assert_eq!(opt.syntax, Syntax::Oneline);
    }

    #[test]
    fn default_oneline_override() {
        let opt = Preparser::new().scan("## syntax: indent\n",
            Options::new().syntax_oneline().clone()).unwrap();
        assert_eq!(opt.syntax, Syntax::Indent);
    }

    #[test]
    fn default_oneline() {
        let opt = Preparser::new().scan("xxxx\n",
            Options::new().syntax_oneline().clone()).unwrap();
        assert_eq!(opt.syntax, Syntax::Oneline);
    }

    #[test]
    fn minimal() {
        let opt = Preparser::new().scan("", Options::new()).unwrap();
        assert_eq!(opt.syntax, Syntax::Plain);
    }
}
