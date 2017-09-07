use regex::Regex;


#[derive(Debug, Clone)]
pub enum Validator {
    Anything,
    Regex(Regex),
}

impl PartialEq for Validator {
    fn eq(&self, other: &Validator) -> bool {
        use self::Validator::*;
        match (self, other) {
            (&Anything, &Anything) => true,
            (&Anything, &Regex(..)) => false,
            (&Regex(..), &Anything) => false,
            (&Regex(ref a), &Regex(ref b)) => a.as_str() == b.as_str(),
        }
    }
}
