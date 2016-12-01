use std::fmt::Display;

use render_error::DataError;
use {Variable};


impl<'a> Variable<'a> for String {
    fn typename(&self) -> &'static str {
        "String"
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(self)
    }
}
