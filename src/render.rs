use std::fmt::{self, Write};
use std::mem::transmute;
use std::rc::Rc;
use std::sync::Arc;

use owning_ref::{ErasedRcRef, OwningRef, OwningRefMut, OwningHandle};

use grammar::{self, Statement, Expr, AssignTarget, Template as Tpl};
use owning::{Own, ExprCode};
use render_error::{RenderError, DataError};
use vars::{UNDEFINED, Var};
use varmap::{Context, set, get};
use {Pos, Variable};


/// A parsed template code that can be rendered
pub struct Template(Arc<Tpl>);


struct Renderer {
    buf: String,
    errors: Vec<(Pos, DataError)>,
    nothing: Rc<()>,
}

impl Template {
    /// Render template to string
    pub fn render(&self, root: &Context)
        -> Result<String, RenderError>
    {
        let mut rnd = Renderer {
            buf: String::new(),
            errors: Vec::new(),
            nothing: Rc::new(()),
        };
        render(&mut rnd, &mut root.sub(),
            &OwningRef::new(Rc::new(self.0.clone())).map(|x| &**x))?;
        if rnd.errors.len() != 0 {
            return Err(RenderError::Data(rnd.errors));
        }
        return Ok(rnd.buf)
    }
}

fn render(r: &mut Renderer, root: &mut Context,
    t: &OwningRef<Rc<Arc<Tpl>>, Tpl>)
    -> Result<(), fmt::Error>
{
    write_block(r, root,
        &t.clone().map(|t| &t.body.statements[..]))
}

fn eval_expr(r: &mut Renderer, root: &Context,
    expr: &OwningRef<Rc<Arc<Tpl>>, Expr>)
    -> ErasedRcRef<Variable>
{
    match expr.clone().map(|e| &e.code).own() {
        ExprCode::Str(ref s) => {
            s.clone().map(|x| x as &Variable).erase_owner()
        },
        ExprCode::Var(ref s) => {
            match get(root, s) {
                Ok(x) => x,
                Err(e) => {
                    r.errors.push((expr.position.0, e));
                    OwningRef::new(r.nothing.clone())
                        .map(|_| UNDEFINED as &Variable)
                        .erase_owner()
                }
            }
        }
        ExprCode::Attr(ref e, ref a) => {
            let value = eval_expr(r, root, e);
            match value.try_map(|v| match v.attr(a) {
                Ok(Var::Ref(x)) => Ok(x),
                Ok(Var::Rc(v)) => Err(v),
                Err(e) => {
                    r.errors.push((expr.position.0, e));
                    Ok(UNDEFINED as &Variable)
                }
            }) {
                Ok(x) => x,
                Err(v) => v,
            }
        }
        _ => unimplemented!(),
    }
}

fn write_block(r: &mut Renderer, root: &mut Context,
    items: &OwningRef<Rc<Arc<Tpl>>, [Statement]>)
    -> Result<(), fmt::Error>
{
    use grammar::StatementCode::*;

    'outer: for (idx, item) in items.iter().enumerate() {
        match item.code {
            OutputRaw(ref x) => {
                r.buf.push_str(x);
            }
            Output(_) => {
                let e = items.clone().map(|x| match x[idx].code {
                    Output(ref e) => e,
                    _ => unreachable!(),
                });
                let var = &eval_expr(r, root, &e);
                write!(&mut r.buf, "{}",
                    var.output().unwrap_or(&""))?;
            }
            Alias { ref target, .. } => {
                let expr = items.clone().map(|x| match x[idx].code {
                    Alias { ref value, .. } => value,
                    _ => unreachable!(),
                });
                let value = eval_expr(r, root, &expr);
                match *target {
                    AssignTarget::Var(_) => {
                        let name = items.clone().map(|x| match x[idx].code {
                            Alias { target: AssignTarget::Var(ref name), .. }
                            => &name[..],
                            _ => unreachable!(),
                        }).erase_owner();
                        set(root, name, value);
                    }
                }
            }
            Cond { conditional: ref clist, ref otherwise } => {
                for (cidx, &(ref cond, ref bbody)) in clist.iter().enumerate()
                {
                    let cond = items.clone().map(|x| match x[idx].code {
                        Cond { ref conditional, .. } => &conditional[cidx].0,
                        _ => unreachable!(),
                    });
                    let condval = eval_expr(r, root, &cond);

                    match condval.as_bool() {
                        Ok(x) if x => {
                            let bstatements = items.clone()
                                .map(|x| match x[idx].code {
                                    Cond { ref conditional, .. }
                                    => &conditional[cidx].1.statements[..],
                                    _ => unreachable!(),
                                });
                            let mut sub = root.sub();
                            write_block(r, &mut sub, &bstatements)?;
                            continue 'outer;
                        }
                        Ok(_) => {}
                        Err(e) => {
                            r.errors.push((cond.position.0, e));
                            // treating as false
                        }
                    }
                }
                let ostatements = items.clone()
                    .map(|x| match x[idx].code {
                        Cond { ref otherwise, .. }
                        => &otherwise.statements[..],
                        _ => unreachable!(),
                    });
                let mut sub = root.sub();
                write_block(r, &mut sub, &ostatements)?;
            }
            Loop { ref target, ref iterator, ref filter, ref body } => {
                let iterator = items.clone().map(|x| match x[idx].code {
                    Loop { ref iterator, .. } => iterator,
                    _ => unreachable!(),
                });
                let value = eval_expr(r, root, &iterator);
                let mut iter = match value.iterate() {
                    Ok(iter) => iter,
                    Err(e) => {
                        r.errors.push((iterator.position.0, e));
                        // treating as empty loop
                        continue 'outer;
                    }
                };

                let statements = items.clone().map(|x| match x[idx].code {
                    Loop { ref body, .. } => &body.statements[..],
                    _ => unreachable!(),
                });

                let target = items.clone().map(|x| match x[idx].code {
                    Loop { target: AssignTarget::Var(ref var), .. } => &var[..],
                    _ => unreachable!(),
                }).erase_owner();

                loop {
                    let mut sub = root.sub();
                    {
                        let res: Result<ErasedRcRef<Variable>, _> =
                            value.clone()
                            .try_map(|x| match iter.next() {
                                Some(Var::Ref(r)) => {
                                    // This transmute should be safe,
                                    // because we only transmute lifetime
                                    // and x and r have basically same lifetime
                                    // because are both tied to the lifetime
                                    // of `value` even if rust doesn't think so
                                    Ok(unsafe { transmute(r) })
                                }
                                Some(Var::Rc(r)) => Err(Err(r)),
                                None => Err(Ok(())),
                            });
                        let val = match res {
                            Ok(x) => x,
                            Err(Err(v)) => v,
                            Err(Ok(())) => break,
                        };
                        set(&mut sub, target.clone(), val);
                    }
                    write_block(r, &mut sub, &statements)?;
                }
            }
        }
    }
    Ok(())
}

pub fn template(imp: grammar::Template) -> Template {
    Template(Arc::new(imp))
}

pub fn extract(tpl: Template) -> grammar::Template {
    Arc::try_unwrap(tpl.0)
        .unwrap_or_else(|_| panic!("Can only extract uncloned template"))
}

impl fmt::Debug for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO(tailhook) show some info about template
        f.debug_struct("Template")
         .finish()
    }
}
