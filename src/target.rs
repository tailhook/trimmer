use std::fmt;

use grammar::AssignTarget;
use varmap::Context;
use vars::Variable;

#[derive(Debug, Clone, Copy)]
pub struct TargetKind(InternalKind);

#[derive(Debug, Clone, Copy)]
pub enum InternalKind {
    Var,
}

/// This is an opaque type used to assign variable values into context by
/// loop iterator
pub struct Target<'a: 'b, 'b> {
    kind: TargetKind,
    vars: &'b mut Context<'a>,
    target: &'b AssignTarget,
}

impl<'a, 'b> Target<'a, 'b> {
    pub fn set(&mut self, value: &Variable) {
        match *self.target {
            AssignTarget::Var(ref name) => {
                unimplemented!();
                //self.vars.set(name.clone(), value);
            }
        }
    }
}

impl fmt::Display for TargetKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            InternalKind::Var => {
                write!(f, "single variable")
            }
        }
    }
}

pub fn make_kind(target: &AssignTarget) -> TargetKind {
    match *target {
        AssignTarget::Var(_) => TargetKind(InternalKind::Var),
    }
}

pub fn make_target<'x, 'y>(target: &'y AssignTarget,
    vars: &'y mut Context<'x>)
    -> Target<'x, 'y>
{
    Target {
        kind: make_kind(target),
        vars: vars,
        target: target,
    }
}
