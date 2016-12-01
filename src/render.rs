use std::fmt::{self, Write};

use grammar::{self, Statement, Expr};
use render_error::{RenderError, DataError, ErrorTracker, tracker};
use {Context, Pos, Variable, Var};


pub enum Value<'ast, 'var> {
    Raw(&'ast str),
    Var(Var<'var>),
    Undefined,
}


/// A parsed template code that can be rendered
pub struct Template(grammar::Template);


struct Renderer {
    buf: String,
    errors: Vec<ErrorTracker>,
}

impl Renderer {
    fn err(&mut self, span: (Pos, Pos), err: DataError) {
        self.errors.push(tracker(span, self.buf.len(), err))
    }
}


impl Template {
    /// Render template to string
    pub fn render(&self, context: &Context)
        -> Result<String, RenderError>
    {
        let mut rnd = Renderer {
            buf: String::new(),
            errors: Vec::new(),
        };
        render(&mut rnd, context, &self.0)?;
        if rnd.errors.len() != 0 {
            return Err(RenderError::Data(rnd.errors));
        }
        return Ok(rnd.buf)
    }
}

fn render(r: &mut Renderer, c: &Context, t: &grammar::Template)
    -> Result<(), fmt::Error>
{
        write_block(r, c, &t.body.statements)
}

fn eval_expr<'ast, 'var>(r: &mut Renderer,
    c: &Context<'var>, expr: &'ast Expr)
    -> Value<'ast, 'var>
{
    use grammar::ExprCode::*;

    match expr.code {
        Str(ref s) => Value::Raw(s),
        Var(ref s) => match c.get(s) {
            Some(x) => Value::Var(x),
            None => {
                r.err(expr.position,
                    DataError::VariableNotFound(s.to_string()));
                Value::Undefined
            }
        },
        _ => unimplemented!(),
    }
}

fn write_block(r: &mut Renderer, c: &Context, items: &[Statement])
    -> Result<(), fmt::Error>
{
    use grammar::StatementCode::*;
    use self::Value::*;

    for item in items {
        match item.code {
            OutputRaw(ref x) => {
                r.buf.push_str(x);
            }
            Output(ref e) => match eval_expr(r, c, e) {
                Raw(text) => r.buf.push_str(text),
                Var(var) => match var.output() {
                    Ok(data) => write!(r.buf, "{}", data)?,
                    Err(e) => {
                        r.err(item.position, e);
                        // value written should be empty
                    }
                },
                Undefined => {
                    // Noop: we already shown this error when fetching the
                    // variable itself
                }
            },
            _ => unimplemented!(),
        }
    }
    Ok(())
}

pub fn template(imp: grammar::Template) -> Template {
    Template(imp)
}

pub fn extract(tpl: Template) -> grammar::Template {
    tpl.0
}
