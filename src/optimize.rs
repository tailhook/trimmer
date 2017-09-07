use grammar::{Body, Statement, StatementCode};
use {Options};


pub struct Optimizer {
}

fn is_whitespace(text: &str) -> bool {
    text.chars().all(|x| x.is_whitespace())
}

fn is_line_or_none(stmt: Option<&Statement>) -> bool {
    if let Some(st) = stmt {
        is_line_statement(&st.code)
    } else {
        true
    }
}

fn is_line_statement(code: &StatementCode) -> bool {
    use grammar::StatementCode::*;
    match *code {
        OutputRaw(..) => false,
        Output {..} => false,
        Cond {..} => true,
        Loop {..} => true,
        Alias {..} => true,
    }
}

impl Optimizer {
    pub fn new() -> Optimizer {
        Optimizer {}
    }
    fn visit_body(&self, body: Body) -> Body {
        use grammar::StatementCode::*;

        let mut interm = Vec::with_capacity(body.statements.len());
        for item in body.statements.into_iter() {
            match (interm.last_mut(), &item) {
                (
                    Some(&mut Statement {
                        position: (_, ref mut old_end),
                        code: OutputRaw(ref mut prev),
                    }),
                    &Statement {
                        position: (_, new_end),
                        code: OutputRaw(ref next),
                    },
                ) => {
                    *old_end = new_end;
                    prev.push_str(next);
                    continue;
                }
                _ => {}
            }
            interm.push(item);
        }
        let mut dst = Vec::with_capacity(interm.len());
        let mut iter = interm.into_iter().peekable();
        while let Some(item) = iter.next() {
            match (dst.last(), &item, iter.peek()) {
                (prev, &Statement { code: OutputRaw(ref this), .. }, next)
                if is_whitespace(this) &&
                    is_line_or_none(prev) && is_line_or_none(next)
                // skip current element
                => continue,
                _ => {}
            }
            dst.push(item);
        }
        let s = dst.into_iter().map(|s| {
            let code = match s.code {
                s@OutputRaw(..) | s@Alias { .. } | s@Output {..} => s,
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
