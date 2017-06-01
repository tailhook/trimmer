use std::rc::Rc;
use std::fmt::{self, Write};

use owning_ref::{ErasedRcRef, OwningRef};

use grammar::{self, Statement, Expr, AssignTarget, Template as Tpl};
use grammar::{ExprCode};
use render_error::{RenderError, DataError};
use vars::{UNDEFINED, Var};
use varmap::Varmap;
use target;
use {Pos, Variable};


/// A parsed template code that can be rendered
pub struct Template(Rc<Tpl>);


struct Renderer {
    buf: String,
    errors: Vec<(Pos, DataError)>,
}


impl Template {
    /// Render template to string
    pub fn render(&self, root: &Rc<Variable>)
        -> Result<String, RenderError>
    {
        let mut rnd = Renderer {
            buf: String::new(),
            errors: Vec::new(),
        };
        let mut root = Varmap::new(root);
        render(&mut rnd, &mut root, &OwningRef::new(self.0.clone()))?;
        if rnd.errors.len() != 0 {
            return Err(RenderError::Data(rnd.errors));
        }
        return Ok(rnd.buf)
    }
}

fn render(r: &mut Renderer, root: &mut Varmap, t: &OwningRef<Rc<Tpl>, Tpl>)
    -> Result<(), fmt::Error>
{
    write_block(r, root, &t.map(|t| &t.body.statements[..]))
}

fn eval_expr(r: &mut Renderer, root: &Varmap,
    expr_ref: &OwningRef<Rc<Tpl>, Expr>)
    -> ErasedRcRef<Variable>
{
    expr_ref.map(|expr| {
        match expr.code {
            ExprCode::Str(ref s) => s as &Variable,
            ExprCode::Var(ref s) => {
                match root.get(s) {
                    Ok(x) => x,
                    Err(e) => {
                        r.errors.push((expr.position.0, e));
                        UNDEFINED
                    }
                }
            }
            ExprCode::Attr(ref e, ref a) => {
                unimplemented!();
                /*
                match eval_expr(r, root, &expr_ref.map(|_| &**e)).attr(a) {
                    Ok(Var::Ref(x)) => x,
                    Ok(Var::Rc(_)) => unimplemented!(),
                    Err(e) => {
                        r.errors.push((expr.position.0, e));
                        UNDEFINED as &Variable
                    }
                }
                */
            }
            _ => unimplemented!(),
        }
    }).erase_owner()
}

fn write_block(r: &mut Renderer, root: &mut Varmap,
    items: &OwningRef<Rc<Tpl>, [Statement]>)
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
                    var.output().unwrap_or(&""))?;
            }
            Cond { ref conditional, ref otherwise } => {
                for &(ref cond, ref branch_body) in conditional {
                    let condval = &eval_expr(r, root, cond);
                    match condval.as_bool() {
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
                let value = eval_expr(r, root, iterator);
                let kind = target::make_kind(target);
                let mut iterator = match value.iterate(kind) {
                    Ok(iter) => iter,
                    Err(e) => {
                        r.errors.push((iterator.position.0, e));
                        // treating as empty loop
                        continue 'outer;
                    }
                };
                loop {
                    let mut sub = root.sub();
                    {
                        let mut target = target::make_target(target, &mut sub);
                        if !iterator.next(&mut target) {
                            break;
                        }
                    }
                    write_block(r, &mut sub, &body.statements)?;
                }
            }
            Alias { ref target, ref value } => {
                let value = &eval_expr(r, root, value);
                match *target {
                    AssignTarget::Var(ref var_name) => {
                        root.set(var_name.to_string(), *value);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn template(imp: grammar::Template) -> Template {
    Template(Rc::new(imp))
}

pub fn extract(tpl: Template) -> grammar::Template {
    Rc::try_unwrap(tpl.0).expect("Can only extract uncloned template")
}
