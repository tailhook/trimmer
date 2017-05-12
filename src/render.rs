use std::fmt::{self, Write};

use grammar::{self, Statement, Expr};
use render_error::{RenderError, DataError};
use vars::{UNDEFINED};
use {Pos, Variable, Context};


/// A parsed template code that can be rendered
pub struct Template(grammar::Template);


struct Renderer<'a> {
    buf: String,
    context: Context<'a>,
    errors: Vec<(Pos, DataError)>,
}


impl Template {
    /// Render template to string
    pub fn render(&self, root: &Variable)
        -> Result<String, RenderError>
    {
        let mut rnd = Renderer {
            buf: String::new(),
            context: Context::new(),
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
            match root.attr(&mut r.context, s) {
                Ok(x) => x,
                Err(e) => {
                    r.errors.push((expr.position.0, e));
                    UNDEFINED
                }
            }
        }
        Attr(ref e, ref a) => {
            match eval_expr(r, root, e).attr(&mut r.context, a) {
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

    'outer: for item in items {
        match item.code {
            OutputRaw(ref x) => {
                r.buf.push_str(x);
            }
            Output(ref e) => {
                let var = &eval_expr(r, root, e);
                write!(&mut r.buf, "{}",
                    var.output(&mut r.context).unwrap_or(&""))?;
            }
            Cond { ref conditional, ref otherwise } => {
                for &(ref cond, ref branch_body) in conditional {
                    let condval = &eval_expr(r, root, cond);
                    match condval.as_bool(&mut r.context) {
                        Ok(x) if x => {
                            write_block(r, root, &branch_body.statements)?;
                            continue 'outer;
                        }
                        Ok(_) => {}
                        Err(e) => {
                            r.errors.push((cond.position.0, e));
                            // treating as false
                        }
                    }
                }
                write_block(r, root, &otherwise.statements)?;
            }
            Loop { ref target, ref iterator, ref filter, ref body } => {
                let iterator = &eval_expr(r, root, iterator);
                unimplemented!();
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
