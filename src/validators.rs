use regex::Regex;

use render_error::DataError;


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

impl Validator {
    pub(crate) fn validate(&self, data: &str)
        -> Result<(), DataError>
    {
        use self::Validator::*;
        match *self {
            Anything => Ok(()),
            Regex(ref re) => {
                if !re.is_match(data) {
                    return Err(DataError::RegexValidationError(
                        data.to_string(), re.as_str().to_string()));
                }
                Ok(())
            }
        }
    }
}
