use std::fmt;

use grammar::AssignTarget;
use varmap::Varmap;
use vars::Variable;

#[derive(Debug, Clone, Copy)]
pub struct TargetKind(InternalKind);

#[derive(Debug, Clone, Copy)]
pub enum InternalKind {
    Var,
}

/// This is an opaque type used to assign variable values into context by
/// loop iterator
pub struct Target<'a: 'b+'c, 'b: 'c, 'c> {
    kind: TargetKind,
    vars: &'c mut Varmap<'a, 'b>,
    target: &'b AssignTarget,
}

impl<'a: 'b+'c, 'b: 'c, 'c> Target<'a, 'b, 'c> {
    pub fn set(&mut self, value: &'a Variable) {
        match *self.target {
            AssignTarget::Var(ref name) => {
                self.vars.set(name.clone(), value);
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

pub fn make_target<'x: 'y, 'y, 'z>(target: &'y AssignTarget,
    vars: &'z mut Varmap<'x, 'y>)
    -> Target<'x, 'y, 'z>
{
    Target {
        kind: make_kind(target),
        vars: vars,
        target: target,
    }
}
