use regex::Regex;


#[derive(Debug, Clone)]
pub enum Validator {
    Anything,
    Regex(Regex),
}
