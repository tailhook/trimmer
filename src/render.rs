use std::fmt::{self, Write};

use grammar::{self, Statement, Expr};
use render_error::{RenderError, DataError};
use vars::undefined;
use {Context, Pos, Variable, Var, IntoVariable};


/// A parsed template code that can be rendered
pub struct Template(grammar::Template);


struct Renderer {
    buf: String,
    errors: Vec<(Pos, DataError)>,
}


impl Template {
    /// Render template to string
    pub fn render(&self, context: &Context)
        -> Result<String, RenderError>
    {
        let mut rnd = Renderer {
            buf: String::new(),
            errors: Vec::new(),
        };
        render(&mut rnd, context, &self.0)?;
        if rnd.errors.len() != 0 {
            return Err(RenderError::Data(rnd.errors));
        }
        return Ok(rnd.buf)
    }
}

fn render(r: &mut Renderer, c: & Context, t: &grammar::Template)
    -> Result<(), fmt::Error>
{
        write_block(r, c, &t.body.statements)
}

fn eval_expr<'x>(r: &mut Renderer, c: &'x Context<'x>, expr: &'x Expr)
    -> Var<'x>
{
    use grammar::ExprCode::*;

    match expr.code() {
        &Str(ref s) => s.into_variable(),
        &Var(ref s) => {
            match c.get(s) {
                Some(x) => x,
                None => {
                    r.errors.push((
                        expr.position.0,
                        DataError::VariableNotFound(s.to_string())));
                    undefined()
                }
            }
        }
        _ => unimplemented!(),
    }
}

fn write_block(r: &mut Renderer, c: &Context<>, items: &[Statement])
    -> Result<(), fmt::Error>
{
    use grammar::StatementCode::*;

    for item in items {
        match item.code {
            OutputRaw(ref x) => {
                r.buf.push_str(x);
            }
            Output(ref e) => {
                let var = &eval_expr(r, c, e);
                write!(&mut r.buf, "{}",
                    var.output().unwrap_or(&""))?;
            }
            _ => unimplemented!(),
        }
    }
    Ok(())
}

pub fn template(imp: grammar::Template) -> Template {
    Template(imp)
}

pub fn extract(tpl: Template) -> grammar::Template {
    tpl.0
}
