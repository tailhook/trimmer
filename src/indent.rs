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
    fn visit_body(&self, body: Body, base_indent: usize)
        -> Result<Body, Error>
    {
        use grammar::StatementCode::*;

        let mut statements = body.statements;
        let mut line_start = true;
        let mut indent = None;
        for s in &mut statements {
            line_start = match s.code {
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
                    indent = Some(indent
                        .map(|x| min(x, cindent))
                        .unwrap_or(cindent));
                    false
                }
                OutputRaw(_) => false,
                Alias { .. } => true,
                Output(..) => false,
                Cond { .. } => true,
                Loop { .. } => true,
            };
        }
        if let Some(indent) = indent {
            let strip_indent = indent - base_indent;
            if strip_indent > 0 {
                let mut line_start = true;
                for s in &mut statements {
                    line_start = match s.code {
                        OutputRaw(ref txt) if txt == "\n" => true,
                        OutputRaw(ref mut txt) if line_start => {
                            *txt = txt[strip_indent..].to_string();
                            false
                        }
                        OutputRaw(_) => false,
                        Alias { .. } => true,
                        Output(..) => false,
                        Cond { .. } => true,
                        Loop { .. } => true,
                    };
                }

            }
        }
        let s = statements.into_iter().map(|s| {
            let code = match s.code {
                s@OutputRaw(..) | s@Alias { .. } | s@Output(..) => s,
                Cond { indent, conditional, otherwise } => Cond {
                    indent,
                    conditional: conditional.into_iter().map(|(e, b)| {
                        Ok((e, self.visit_body(b, indent)?))
                    }).collect::<Result<_, Error>>()?,
                    otherwise: self.visit_body(otherwise, indent)?,
                },
                Loop { indent, target, iterator, filter, body } => Loop {
                    indent, target, iterator, filter,
                    body: self.visit_body(body, indent)?,
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
        self.visit_body(body, 0)
    }

}
