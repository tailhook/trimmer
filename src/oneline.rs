use grammar::{Body, Statement};
use {Options};


pub struct Postprocess {
}

impl Postprocess {
    pub fn new() -> Postprocess {
        Postprocess {}
    }

    fn visit_body(&self, body: Body, top: bool) -> Body {
        use grammar::StatementCode::*;

        let statements = body.statements;
        let last = statements.len().saturating_sub(1);
        let s = statements.into_iter().enumerate().filter_map(|(i, s)| {
            let code = match s.code {
                OutputRaw(ref val) if val == "" => return None,
                OutputRaw(val) => {
                    let mut res = String::with_capacity(val.len());
                    let first_c = val.chars().next().unwrap();
                    let last_c = val.chars().rev().next().unwrap();
                    let start_ws = first_c.is_whitespace() && (!top || i != 0);
                    let end_ws = last_c.is_whitespace() && (!top || i != last);
                    for item in val.split_whitespace() {
                        if start_ws || res.len() > 0 {
                            res.push(' ');
                        }
                        res.push_str(item);
                    }
                    if res.len() > 0 {
                        if end_ws {
                            res.push(' ');
                        }
                        OutputRaw(res)
                    } else {
                        return None;
                    }
                }
                s@Alias { .. } | s@Output(..) => s,
                Cond { conditional, otherwise } => Cond {
                    conditional: conditional.into_iter().map(|(e, b)| {
                        (e, self.visit_body(b, false))
                    }).collect(),
                    otherwise: self.visit_body(otherwise, false),
                },
                Loop { target, iterator, filter, body } => Loop {
                    target, iterator, filter,
                    body: self.visit_body(body, false),
                }
            };
            Some(Statement {
                code: code,
                .. s
            })
        }).collect();
        Body {
            statements: s,
            .. body
        }
    }

    pub fn process(&self, _opt: &Options, body: Body) -> Body {
        self.visit_body(body, true)
    }

}
