use grammar::{Body, Statement};
use {Options};


pub struct Postprocess {
}

impl Postprocess {
    pub fn new() -> Postprocess {
        Postprocess {}
    }

    fn visit_body(&self, body: Body) -> Body {
        use grammar::StatementCode::*;
        use grammar::OutputMode::{Preserve, Space, Strip};

        let mut st = body.statements;
        let last = st.len().saturating_sub(1);
        for i in 0..st.len() {
            let fix_start = if i == 0 {
                true
            } else {
                match st[i-1].code {
                    Output {..} => false,
                    // Empty OutputRaw's are already optimized out
                    OutputRaw(ref x)
                    => x.chars().rev().next().unwrap().is_whitespace(),
                    _ => true,
                }
            };
            let fix_end = if i == last {
                true
            } else {
                match st[i+1].code {
                    Output {..} => false,
                    // Empty OutputRaw's are already optimized out
                    OutputRaw(ref x)
                    => x.chars().next().unwrap().is_whitespace(),
                    _ => true,
                }
            };
            match st[i].code {
                Output { ref mut left_ws, ref mut right_ws, .. } => {
                    if *left_ws == Preserve {
                        *left_ws = if fix_start { Space } else { Strip };
                    }
                    if *right_ws == Preserve && fix_end {
                        *right_ws = if fix_end { Space } else { Strip };
                    }
                }
                _ => {}
            }
        }
        let st = st.into_iter().map(|s| {
            let code = match s.code {
                OutputRaw(text) => OutputRaw(
                    text.split_whitespace()
                        .collect::<Vec<_>>().join(" ")
                ),
                s@Output {..} | s@Alias { .. } => s,
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
            statements: st,
            .. body
        }
    }

    pub fn process(&self, _opt: &Options, body: Body) -> Body {
        self.visit_body(body)
    }

}
