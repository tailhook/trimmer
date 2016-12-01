use std::fmt::{self, Write};

use grammar::{self, Statement, Expr};
use render_error::{RenderError, DataError};
use vars::{UNDEFINED};
use {Pos, Variable};


/// A parsed template code that can be rendered
pub struct Template(grammar::Template);


struct Renderer {
    buf: String,
    errors: Vec<(Pos, DataError)>,
}


impl Template {
    /// Render template to string
    pub fn render(&self, root: &Variable)
        -> Result<String, RenderError>
    {
        let mut rnd = Renderer {
            buf: String::new(),
            errors: Vec::new(),
        };
        render(&mut rnd, root, &self.0)?;
        if rnd.errors.len() != 0 {
            return Err(RenderError::Data(rnd.errors));
        }
        return Ok(rnd.buf)
    }
}

fn render(r: &mut Renderer, root: &Variable, t: &grammar::Template)
    -> Result<(), fmt::Error>
{
        write_block(r, root, &t.body.statements)
}

fn eval_expr<'x>(r: &mut Renderer, root: &'x Variable, expr: &'x Expr)
    -> &'x Variable
{
    use grammar::ExprCode::*;

    match expr.code {
        Str(ref s) => s,
        Var(ref s) => {
            match root.attr(s) {
                Ok(x) => x,
                Err(e) => {
                    r.errors.push((expr.position.0, e));
                    UNDEFINED
                }
            }
        }
        Attr(ref e, ref a) => {
            match eval_expr(r, root, e).attr(a) {
                Ok(x) => x,
                Err(e) => {
                    r.errors.push((expr.position.0, e));
                    UNDEFINED
                }
            }
        }
        _ => unimplemented!(),
    }
}

fn write_block(r: &mut Renderer, root: &Variable, items: &[Statement])
    -> Result<(), fmt::Error>
{
    use grammar::StatementCode::*;

    for item in items {
        match item.code {
            OutputRaw(ref x) => {
                r.buf.push_str(x);
            }
            Output(ref e) => {
                let var = &eval_expr(r, root, e);
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
