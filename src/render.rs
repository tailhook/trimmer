use std::io::{self, Write};

use grammar::{self, Statement};
use render_error::{RenderError, DataError};
use {Context, Pos};


/// A parsed template code that can be rendered
pub struct Template(grammar::Template);


pub struct Renderer<'a, W: Write + 'a> {
    buf: &'a mut W,
    errors: Vec<(Pos, DataError)>,
}


impl Template {
    /// Render template to string
    pub fn render(&self, context: &Context) -> Result<String, RenderError> {
        let mut buf = Vec::new();
        self.render_into(context, &mut buf)?;
        Ok(String::from_utf8(buf).unwrap())
    }

    /// Render template to custom buffer
    ///
    /// Note: when error occurs partial data are already written
    pub fn render_into<B: io::Write>(&self, context: &Context, buf: &mut B)
        -> Result<(), RenderError>
    {
        let mut rnd = Renderer {
            buf: buf,
            errors: Vec::new(),
        };
        rnd.write_block(&self.0.body.statements)?;
        if rnd.errors.len() == 0 {
            return Err(RenderError::Data(rnd.errors));
        } else {
            return Ok(())
        }
    }
}

impl<'a, W: Write> Renderer<'a, W> {
    fn write_block(&mut self, vec: &[Statement]) -> Result<(), io::Error> {
        unimplemented!();
    }
}
