use std::fmt::{self, Write};

use grammar::{self, Statement, Expr, AssignTarget};
use render_error::{RenderError, DataError};
use vars::{UNDEFINED};
use varmap::Varmap;
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
        let mut root = Varmap::new(root);
        render(&mut rnd, &mut root, &self.0)?;
        if rnd.errors.len() != 0 {
            return Err(RenderError::Data(rnd.errors));
        }
        return Ok(rnd.buf)
    }
}

fn render<'x: 'y, 'y>(r: &mut Renderer<'x>, root: &mut Varmap<'x, 'y>,
    t: &'x grammar::Template)
    -> Result<(), fmt::Error>
{
    write_block(r, root, &t.body.statements)
}

fn eval_expr<'x: 'y, 'y>(r: &mut Renderer, root: &Varmap<'x, 'y>, expr: &'x Expr)
    -> &'x Variable
{
    use grammar::ExprCode::*;

    match expr.code {
        Str(ref s) => s,
        Var(ref s) => {
            match root.get(&mut r.context, s) {
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

fn write_block<'x: 'y, 'y>(r: &mut Renderer<'x>,
    root: &mut Varmap<'x, 'y>, items: &'x [Statement])
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
                            let mut sub = root.sub();
                            write_block(r, &mut sub, &branch_body.statements)?;
                            continue 'outer;
                        }
                        Ok(_) => {}
                        Err(e) => {
                            r.errors.push((cond.position.0, e));
                            // treating as false
                        }
                    }
                }
                let mut sub = root.sub();
                write_block(r, &mut sub, &otherwise.statements)?;
            }
            Loop { ref target, ref iterator, ref filter, ref body } => {
                let iterator = eval_expr(r, root, iterator);
                //let mut sub = root.sub();
                //sub.insert(String::from("__current_iterator"), iterator);
                unimplemented!();
            }
            Alias { ref target, ref value } => {
                let value = &eval_expr(r, root, value);
                match *target {
                    AssignTarget::Var(ref var_name) => {
                        root.insert(var_name.to_string(), *value);
                    }
                }
            }
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
