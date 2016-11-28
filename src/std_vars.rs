use std::fmt::Display;

use render_error::DataError;
use {Variable};

/*
impl<'a> Variable for &'a str {
    fn typename(&self) -> &'static str {
        "&str"
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(self)
    }
}

impl Variable for str {
    fn typename(&self) -> &'static str {
        "&str"
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(&self)
    }
}
*/

impl Variable for String {
    fn typename(&self) -> &'static str {
        "String"
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(self)
    }
}
