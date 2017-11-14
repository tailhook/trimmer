use std::cmp::min;

use grammar::{Body, Statement};
use {Options, ParseError as Error};
use parse_error::ParseErrorEnum as ErrorEnum;


pub struct Postprocess {
}

impl Postprocess {
    pub fn new() -> Postprocess {
        Postprocess {}
    }
    fn visit_body(&self, body: Body, base_indent: usize, mut strip: usize)
        -> Result<Body, Error>
    {
        use grammar::StatementCode::*;
        debug_assert!(strip <= base_indent);

        let mut statements = body.statements;
        let mut indent = None;
        {
            let mut line_start = true;
            let mut update_indent = |cindent| {
                indent = Some(indent
                    .map(|x| min(x, cindent))
                    .unwrap_or(cindent));
            };
            for s in &mut statements {
                line_start = match s.code {
                    Joiner => line_start,
                    OutputRaw(ref txt) if txt == "\n" => true,
                    OutputRaw(ref txt) if line_start => {
                        let cindent = txt.len() - txt.trim_left().len();
                        if cindent < base_indent {
                            return Err(ErrorEnum::InvalidSyntax(s.position.0,
                                format!("line is under-indented \
                                     in 'indent' syntax mode, \
                                     expected {} but is {}",
                                     base_indent, cindent))
                                .into());
                        }
                        update_indent(cindent);
                        false
                    }
                    OutputRaw(_) => false,
                    Alias { .. } => true,
                    Output { .. } => false,
                    Cond { indent, .. } => {
                        update_indent(indent);
                        true
                    }
                    Loop { indent, .. } => {
                        update_indent(indent);
                        true
                    }
                };
            }
        }
        strip += indent.unwrap_or(base_indent) - base_indent;
        if strip > 0 {
            let mut line_start = true;
            for s in &mut statements {
                line_start = match s.code {
                    Joiner => line_start,
                    OutputRaw(ref txt) if txt == "\n" => true,
                    OutputRaw(ref mut txt) if line_start => {
                        *txt = txt[strip..].to_string();
                        false
                    }
                    OutputRaw(_) => false,
                    Alias { .. } => true,
                    Output { .. } => false,
                    Cond { .. } => true,
                    Loop { .. } => true,
                };
            }

        }

        let s = statements.into_iter().map(|s| {
            let code = match s.code {
                Joiner => Joiner,
                s@OutputRaw(..) | s@Alias { .. } | s@Output {..} => s,
                Cond { indent, conditional, otherwise } => Cond {
                    indent,
                    conditional: conditional.into_iter().map(|(e, b)| {
                        Ok((e, self.visit_body(b, indent, strip)?))
                    }).collect::<Result<_, Error>>()?,
                    otherwise: self.visit_body(otherwise, indent, strip)?,
                },
                Loop { indent, target, iterator, filter, body } => Loop {
                    indent, target, iterator, filter,
                    body: self.visit_body(body, indent, strip)?,
                }
            };
            Ok(Statement {
                code: code,
                .. s
            })
        }).collect::<Result<_, Error>>()?;
        Ok(Body {
            statements: s,
            .. body
        })
    }
    pub fn process(&self, _opt: &Options, body: Body) -> Result<Body, Error> {
        self.visit_body(body, 0, 0)
    }

}
