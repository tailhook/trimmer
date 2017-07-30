use grammar::{Body, Statement};
use {Options};


pub struct Optimizer {
}

impl Optimizer {
    pub fn new() -> Optimizer {
        Optimizer {}
    }
    fn visit_body(&self, body: Body) -> Body {
        use grammar::StatementCode::*;

        let mut dst = Vec::with_capacity(body.statements.len());
        for item in body.statements.into_iter() {
            match (&item, dst.last_mut()) {
                (
                    &Statement {
                        position: (_, new_end),
                        code: OutputRaw(ref next),
                    },
                    Some(&mut Statement {
                        position: (_, ref mut old_end),
                        code: OutputRaw(ref mut prev),
                    })
                ) => {
                    *old_end = new_end;
                    prev.push_str(next);
                    continue;
                }
                _ => {}
            }
            dst.push(item);
        }
        let s = dst.into_iter().map(|s| {
            let code = match s.code {
                s@OutputRaw(..) | s@Alias { .. } | s@Output(..) => s,
                Cond { indent, conditional, otherwise } => Cond {
                    indent,
                    conditional: conditional.into_iter().map(|(e, b)| {
                        (e, self.visit_body(b))
                    }).collect(),
                    otherwise: self.visit_body(otherwise),
                },
                Loop { indent, target, iterator, filter, body } => Loop {
                    indent, target, iterator, filter,
                    body: self.visit_body(body),
                }
            };
            Statement {
                code: code,
                .. s
            }
        }).collect();
        Body {
            statements: s,
            .. body
        }
    }
    pub fn optimize(&self, _opt: &Options, body: Body) -> Body {
        self.visit_body(body)
    }
}
