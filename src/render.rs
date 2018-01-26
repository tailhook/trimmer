use std::fmt::{self, Write};
use std::cmp::min;
use std::mem::transmute;
use std::rc::Rc;
use std::sync::Arc;
use std::collections::HashMap;

use owning_ref::{OwningRef, Erased};

use grammar::OutputMode::{self, Preserve, Strip, Space};
use grammar::{self, Statement, Expr, AssignTarget, Template as Tpl};
use number::{self, Number};
use owning::{Own, ExprCode};
use compare::{compare};
use preparser::Syntax::Oneline;
use render_error::{RenderError, DataError};
use varmap::{Context, SubContext, set, get};
use vars::{UNDEFINED, TRUE, FALSE, Val, VarRef, RefVar};
use {Pos, Variable, Var};


/// A parsed template code that can be rendered
pub struct Template(Arc<Tpl>);


pub(crate) struct Renderer {
    pub(crate) buf: String,
    pub(crate) template: Arc<Tpl>,
    pub(crate) frozen: usize,
    pub(crate) tail_mode: OutputMode,
    pub(crate) errors: Vec<(Pos, DataError)>,
    pub(crate) nothing: Rc<()>,
}

impl Template {
    /// Render template to string
    pub fn render(&self, root: &Context)
        -> Result<String, RenderError>
    {
        let mut rnd = Renderer {
            template: self.0.clone(),
            buf: String::new(),
            errors: Vec::new(),
            nothing: Rc::new(()),
            tail_mode: Preserve,
            frozen: 0,
        };
        render(&mut rnd, &mut SubContext::from(root),
            &OwningRef::new(Rc::new(self.0.clone())).map(|x| &**x))?;
        if rnd.errors.len() != 0 {
            return Err(RenderError::Data(rnd.errors));
        }
        return Ok(rnd.buf)
    }
}

fn render(r: &mut Renderer, root: &mut SubContext,
    t: &OwningRef<Rc<Arc<Tpl>>, Tpl>)
    -> Result<(), fmt::Error>
{
    write_block(r, root,
        &t.clone().map(|t| &t.body.statements[..]))
}

fn nothing<'x, 'y, 'render: 'x>(n: &'x Rc<()>, _: &SubContext<'y, 'render>)
    -> Rc<Erased+'render>
{
    n.clone()
}

fn operator<'x, 'render: 'x>(op: fn(Number, Number) -> VarRef<'render>,
    a: &OwningRef<Rc<Arc<Tpl>>, Expr>, b: &OwningRef<Rc<Arc<Tpl>>, Expr>,
    r: &mut Renderer, root: &SubContext<'x, 'render>)
    -> VarRef<'render>
{
    let left = eval_expr(r, root, a);
    let right = eval_expr(r, root, b);
    match (left.as_number(), right.as_number()) {
        (Ok(a), Ok(b)) => {
            op(a, b)
        }
        (Ok(_), Err(eb)) => {
            r.errors.push((b.position.0, eb));
            left
        }
        (Err(ea), Ok(_)) => {
            r.errors.push((a.position.0, ea));
            right
        }
        (Err(ea), Err(eb)) => {
            r.errors.push((a.position.0, ea));
            r.errors.push((b.position.0, eb));
            OwningRef::new(nothing(&r.nothing, root))
                .map(|_| UNDEFINED as &Variable)
        }
    }
}

fn eval_expr<'x, 'render: 'x>(r: &mut Renderer, root: &SubContext<'x, 'render>,
    expr: &OwningRef<Rc<Arc<Tpl>>, Expr>)
    -> VarRef<'render>
{
    use render_error::DataError::*;

    match expr.clone().map(|e| &e.code).own() {
        ExprCode::Str(ref s) => {
            s.clone().map(|x| x as &Variable).erase_owner()
        },
        ExprCode::Var(ref s) => {
            match get(root, s) {
                Ok(x) => x,
                Err(e) => {
                    r.errors.push((expr.position.0, e));
                    OwningRef::new(nothing(&r.nothing, root))
                        .map(|_| UNDEFINED as &Variable)
                }
            }
        }
        ExprCode::Attr(ref e, ref a) => {
            let value = eval_expr(r, root, e);
            match value.try_map(|v| match v.attr(a) {
                Ok(Var(Val::Ref(x))) => Ok(x),
                Ok(Var(Val::Rc(v))) => Err(v),
                Err(e) => {
                    if !matches!(e, AttrNotFound) {
                        r.errors.push((expr.position.0, e));
                    }
                    Err(OwningRef::new(nothing(&r.nothing, root))
                        .map(|_| UNDEFINED as &Variable))
                }
            }) {
                Ok(x) => x,
                Err(v) => v,
            }
        }
        ExprCode::Add(ref a, ref b) => {
            operator(number::add, a, b, r, root)
        }
        ExprCode::Sub(ref a, ref b) => {
            operator(number::sub, a, b, r, root)
        }
        ExprCode::Mul(ref a, ref b) => {
            operator(number::mul, a, b, r, root)
        }
        ExprCode::Div(ref a, ref b) => {
            operator(number::div, a, b, r, root)
        }
        ExprCode::Mod(ref a, ref b) => {
            operator(number::modulo, a, b, r, root)
        }
        ExprCode::And(ref a, ref b) => {
            let left = eval_expr(r, root, a);
            match left.as_bool() {
                Ok(true) | Err(BoolUnsupported(_)) => {
                    eval_expr(r, root, b)
                }
                Ok(false) => {
                    left
                }
                Err(e) => {
                    r.errors.push((expr.position.0, e));
                    // this is kinda undefined, so false
                    OwningRef::new(nothing(&r.nothing, root))
                        .map(|_| UNDEFINED as &Variable)
                }
            }
        }
        ExprCode::Or(ref a, ref b) => {
            let left = eval_expr(r, root, a);
            match left.as_bool() {
                Ok(true) | Err(BoolUnsupported(_)) => {
                    left
                }
                Ok(false) => {
                    eval_expr(r, root, b)
                }
                Err(e) => {
                    r.errors.push((expr.position.0, e));
                    // this is kinda undefined, so false
                    eval_expr(r, root, b)
                }
            }
        }
        ExprCode::Item(ref e, ref a) => {
            let value = eval_expr(r, root, e);
            let index = eval_expr(r, root, a);
            match value.try_map(|v| match v.index(&*index) {
                Ok(Var(Val::Ref(x))) => Ok(x),
                Ok(Var(Val::Rc(v))) => Err(v),
                Err(e) => {
                    // we allow AttrNotFound too, for the cases where
                    // square brackets equal to attribute access (JSON)
                    if !matches!(e, IndexNotFound | AttrNotFound) {
                        r.errors.push((expr.position.0, e));
                    }
                    Err(OwningRef::new(nothing(&r.nothing, root))
                        .map(|_| UNDEFINED as &Variable))
                }
            }) {
                Ok(x) => x,
                Err(v) => v,
            }
        }
        ExprCode::Int(ref val) => {
            val.clone().map(|x| x as &Variable).erase_owner()
        }
        ExprCode::Float(ref val) => {
            val.clone().map(|x| x as &Variable).erase_owner()
        }
        ExprCode::Not(ref v) => {
            let value = eval_expr(r, root, v);
            match value.as_bool() {
                Ok(true) => {
                    OwningRef::new(nothing(&r.nothing, root))
                        .map(|_| FALSE as &Variable)
                }
                Ok(false) => {
                    OwningRef::new(nothing(&r.nothing, root))
                        .map(|_| TRUE as &Variable)
                }
                Err(e) => {
                    r.errors.push((expr.position.0, e));
                    OwningRef::new(nothing(&r.nothing, root))
                        .map(|_| UNDEFINED as &Variable)
                }
            }
        },
        ExprCode::Comparison(ref left, ref vec) => {
            assert!(vec.len() > 0);
            let mut cur_exp = eval_expr(r, root, left);
            for i in 0..vec.len() {
                cur_exp = {
                    let left = match cur_exp.as_comparable() {
                        Ok(c) => c,
                        Err(e) => {
                            r.errors.push((expr.position.0, e));
                            return OwningRef::new(nothing(&r.nothing, root))
                                .map(|_| UNDEFINED as &Variable)
                        }
                    };
                    let right = expr.clone().map(|e| match e.code {
                        grammar::ExprCode::Comparison(_, ref vec) => {
                            &vec[i].1
                        }
                        _ => unreachable!(),
                    });
                    let oper = vec[i].0;
                    let rexpr = eval_expr(r, root, &right);
                    let next = match rexpr.as_comparable() {
                        Ok(c) => c,
                        Err(e) => {
                            r.errors.push((expr.position.0, e));
                            return OwningRef::new(nothing(&r.nothing, root))
                                .map(|_| UNDEFINED as &Variable)
                        }
                    };
                    match compare(&left, &next, oper) {
                        Ok(false) => {
                            return OwningRef::new(nothing(&r.nothing, root))
                                .map(|_| FALSE as &Variable);
                        }
                        Ok(true) => {}
                        Err(()) => {
                            r.errors.push((expr.position.0,
                                Incomparable(cur_exp.typename(),
                                             rexpr.typename())
                            ));
                            return OwningRef::new(nothing(&r.nothing, root))
                                .map(|_| UNDEFINED as &Variable)
                        }
                    }
                    rexpr.clone()
                };
            }
            return OwningRef::new(nothing(&r.nothing, root))
                .map(|_| TRUE as &Variable);
        }
        ExprCode::Dict(ref pairs) => {
            let map = pairs.iter().flat_map(|&(ref key, ref value)| {
                    let key_exp = eval_expr(r, root, key);
                    let value_exp = eval_expr(r, root, value);
                    match key_exp.as_str_key() {
                        Ok(kstr) => {
                            // TODO(tailhook) no clone?
                            let key = kstr.to_string();
                            Some((key, RefVar(value_exp)))
                        }
                        Err(e) => {
                            r.errors.push((key.position.0, e));
                            None
                        }
                    }
            }).collect::<HashMap<_, _>>();
            return OwningRef::new(Rc::new(map))
                .map(|x: &HashMap<_, _>| x as &Variable).erase_owner();
        }
        ExprCode::List(ref pairs) => {
            let map = pairs.iter().map(|item| {
                  RefVar(eval_expr(r, root, item))
            }).collect::<Vec<_>>();
            return OwningRef::new(Rc::new(map))
                .map(|x: &Vec<_>| x as &Variable).erase_owner();
        }
        x => panic!("Unimplemented oper {:?}", x),
    }
}

fn write_block<'x, 'render>(r: &mut Renderer,
    root: &mut SubContext<'x, 'render>,
    items: &OwningRef<Rc<Arc<Tpl>>, [Statement]>)
    -> Result<(), fmt::Error>
{
    use grammar::StatementCode::*;
    use render_error::DataError::*;

    'outer: for (idx, item) in items.iter().enumerate() {
        match item.code {
            Joiner => {
                r.tail_mode = Space;
            }
            OutputRaw(ref text) => {
                let base_mode = if r.template.options.syntax == Oneline {
                    Space
                } else {
                    Preserve
                };
                r.tail_mode = match r.tail_mode {
                    Preserve => {
                        let trim_len = text.trim_right().len();
                        r.buf.push_str(text);
                        if trim_len > 0 && trim_len <= text.len() {
                            let spaces = text.len() - trim_len;
                            r.frozen = r.buf.len() - spaces;
                        }
                        base_mode
                    }
                    Strip => {
                        let off = r.frozen;
                        r.buf.truncate(off);
                        let s = text.trim_left();
                        if s.len() > 0 {
                            let trim_len = s.trim_right().len();
                            let spaces = s.len() - trim_len;
                            r.buf.push_str(s);
                            r.frozen = r.buf.len() - spaces;
                            base_mode
                        } else {
                            Strip
                        }
                    }
                    Space => {
                        let s = text.trim_left();
                        if s.len() > 0 {
                            let off = r.frozen;
                            r.buf.truncate(off);
                            let trim_len = s.trim_right().len();
                            let spaces = s.len() - trim_len;
                            if r.buf.len() > 0 {
                                r.buf.push(' ');
                            }
                            r.buf.push_str(s);
                            r.frozen = r.buf.len() - spaces;
                            base_mode
                        } else {
                            Space
                        }
                    }
                }
            }
            Output { left_ws, right_ws, ref validator, expr: _ } => {
                match min(left_ws, r.tail_mode) {
                    Preserve => {},
                    Strip => {
                        let off = r.frozen;
                        r.buf.truncate(off);
                    }
                    Space => {
                        let off = r.frozen;
                        r.buf.truncate(off);
                        if r.buf.len() > 0 {
                            r.buf.push(' ');
                        }
                    }
                }
                let e = items.clone().map(|x| match x[idx].code {
                    Output { ref expr, .. } => expr,
                    _ => unreachable!(),
                });
                let var = &eval_expr(r, root, &e);
                match var.output() {
                    Ok(value) => {
                        let start = r.buf.len();
                        write!(&mut r.buf, "{}", value.0)?;
                        let val = match *validator {
                            Some(ref name) => {
                                match r.template.options.validators.get(name) {
                                    Some(val) => val,
                                    None => {
                                        r.errors.push((item.position.0,
                                            UnknownValidator(
                                                name.to_string())));
                                        &r.template.options.default_validator
                                    }
                                }
                            }
                            None => {
                                &r.template.options.default_validator
                            }
                        };
                        match val.validate(&r.buf[start..]) {
                            Ok(()) => {}
                            Err(e) => {
                                r.errors.push((item.position.0, e));
                            }
                        }
                    }
                    Err(e) => {
                        r.errors.push((item.position.0, e));
                    }
                }
                r.frozen = r.buf.len();
                r.tail_mode = right_ws;
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
                    // unsupported by grammar yet
                    AssignTarget::Pair(..) => unreachable!(),
                }
            }
            Cond { conditional: ref clist, .. } => {
                for (cidx, _) in clist.iter().enumerate()
                {
                    let cond = items.clone().map(|x| match x[idx].code {
                        Cond { ref conditional, .. } => &conditional[cidx].0,
                        _ => unreachable!(),
                    });
                    let condval = eval_expr(r, root, &cond);

                    match condval.as_bool() {
                        Ok(true) | Err(BoolUnsupported(..)) => {
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
                        Ok(false) => {}
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
            Loop { target: AssignTarget::Var(_), ref filter, .. } => {
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

                let filter = if filter.is_some() {
                    Some(items.clone().map(|x| match x[idx].code {
                        Loop { ref filter, .. } => filter.as_ref().unwrap(),
                        _ => unreachable!(),
                    }))
                } else {
                    None
                };

                loop {
                    let mut sub = root.sub();
                    {
                        let res: Result<VarRef<'render>, _> =
                            value.clone()
                            .try_map(|_value| match iter.next() {
                                Some(Var(Val::Ref(r))) => {
                                    // This transmute should be safe,
                                    // because we only transmute lifetime
                                    // and x and r have basically same lifetime
                                    // because are both tied to the lifetime
                                    // of `value` even if rust doesn't think so
                                    Ok(unsafe { transmute(r) })
                                }
                                Some(Var(Val::Rc(r))) => Err(Err(r)),
                                None => Err(Ok(())),
                            });
                        let val = match res {
                            Ok(x) => x,
                            Err(Err(v)) => v,
                            Err(Ok(())) => break,
                        };
                        set(&mut sub, target.clone(), val);
                    }
                    if let Some(ref filter) = filter {
                        let condval = eval_expr(r, &sub, filter);
                        match condval.as_bool() {
                            Ok(true) | Err(BoolUnsupported(..)) => {
                                // Skip It!
                                continue;
                            }
                            Ok(false) => {},
                            Err(e) => {
                                r.errors.push((filter.position.0, e));
                                // treating as false
                            }
                        };
                    }
                    write_block(r, &mut sub, &statements)?;
                }
            }
            Loop { target: AssignTarget::Pair(..), ref filter, .. } => {
                let iterator = items.clone().map(|x| match x[idx].code {
                    Loop { ref iterator, .. } => iterator,
                    _ => unreachable!(),
                });
                let value = eval_expr(r, root, &iterator);
                let mut iter = match value.iterate_pairs() {
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

                let targ_a = items.clone().map(|x| match x[idx].code {
                    Loop { target: AssignTarget::Pair(ref a, _), .. }
                    => &a[..],
                    _ => unreachable!(),
                }).erase_owner();
                let targ_b = items.clone().map(|x| match x[idx].code {
                    Loop { target: AssignTarget::Pair(_, ref b), .. }
                    => &b[..],
                    _ => unreachable!(),
                }).erase_owner();

                let filter = if filter.is_some() {
                    Some(items.clone().map(|x| match x[idx].code {
                        Loop { ref filter, .. } => filter.as_ref().unwrap(),
                        _ => unreachable!(),
                    }))
                } else {
                    None
                };

                loop {
                    let mut sub = root.sub();
                    {
                        let (val_a, val_b) = match iter.next() {
                            Some((var_a, var_b)) => {
                                let val_a = match var_a {
                                    Var(Val::Ref(r)) => {
                                        value.clone()
                                        // This transmute should be safe,
                                        // because we only transmute lifetime
                                        // and x and r have basically same
                                        // lifetime because are both tied to
                                        // the lifetime of `value` even if
                                        // rust doesn't think so
                                        .map(|_| unsafe { transmute(r) })
                                    }
                                    Var(Val::Rc(r)) => r,
                                };
                                let val_b = match var_b {
                                    Var(Val::Ref(r)) => {
                                        value.clone()
                                        // This transmute should be safe,
                                        // because we only transmute lifetime
                                        // and x and r have basically same
                                        // lifetime because are both tied to
                                        // the lifetime of `value` even if
                                        // rust doesn't think so
                                        .map(|_| unsafe { transmute(r) })
                                    }
                                    Var(Val::Rc(r)) => r,
                                };
                                (val_a, val_b)
                            }
                            None => break,
                        };
                        set(&mut sub, targ_a.clone(), val_a);
                        set(&mut sub, targ_b.clone(), val_b);
                    }
                    if let Some(ref filter) = filter {
                        let condval = eval_expr(r, &sub, filter);
                        match condval.as_bool() {
                            Ok(true) | Err(BoolUnsupported(..)) => {
                                // Skip It!
                                continue;
                            }
                            Ok(false) => {},
                            Err(e) => {
                                r.errors.push((filter.position.0, e));
                                // treating as false
                            }
                        };
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

#[cfg(test)]
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
