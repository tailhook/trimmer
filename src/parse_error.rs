use std::fmt::Write;

use combine::primitives::{ParseError as CombineError, SourcePosition, Error};

use tokenizer::TokenStream;


quick_error! {
    #[derive(Debug)]
    pub enum ParseError {
        Parse(position: SourcePosition, error: String) {
            description("error parsing template")
            display("{}", error)
        }
    }
}


impl<'a> From<CombineError<TokenStream<'a>>> for ParseError {
    fn from(e: CombineError<TokenStream<'a>>) -> ParseError {
        let mut buf = String::with_capacity(100);

        // First print the token that we did not expect There should really
        // just be one unexpected message at this point though we print them
        // all to be safe
        let unexpected = e.errors.iter()
            .filter(|e| {
                match **e {
                    Error::Unexpected(_) => true,
                    _ => false,
                }
            });
        for error in unexpected {
            writeln!(&mut buf, "{}", error).unwrap();
        }

        // Then we print out all the things that were expected in a comma
        // separated list 'Expected 'a', 'expression' or 'let'
        let iter = || {
            e.errors.iter()
                .filter_map(|e| {
                    match *e {
                        Error::Expected(ref err) => Some(err),
                        _ => None,
                    }
                })
        };
        let expected_count = iter().count();
        for (i, message) in iter().enumerate() {
            let s = match i {
                0 => "Expected",
                _ if i < expected_count - 1 => ",",
                // Last expected message to be written
                _ => " or",
            };
            write!(&mut buf, "    {} `{}`", s, message).unwrap();
        }
        if expected_count != 0 {
            writeln!(&mut buf, "").unwrap();
        }
        // If there are any generic messages we print them out last
        let messages = e.errors.iter()
            .filter(|e| {
                match **e {
                    Error::Message(_) |
                    Error::Other(_) => true,
                    _ => false,
                }
            });
        for error in messages {
            writeln!(&mut buf, "    {}", error).unwrap();
        }

        ParseError::Parse(e.position, buf)
    }
}
