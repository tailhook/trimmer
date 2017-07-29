use grammar::{Body, Statement};
use preparser::Options;
use regex::Regex;


pub struct Postprocess {
}

fn empty(x: &Statement) -> bool {
    use grammar::StatementCode::OutputRaw;
    match x.code {
        OutputRaw(ref data) if data == "" => false,
        _ => true
    }
}

impl Postprocess {
    pub fn new() -> Postprocess {
        Postprocess {}
    }

    fn visit_body(&self, body: Body, top: bool) -> Body {
        use grammar::StatementCode::OutputRaw;

        let mut statements = body.statements;
        let last = statements.len().saturating_sub(1);
        for i in 0..statements.len() {
            match &mut statements[i].code {
                &mut OutputRaw(ref mut val) if val == "" => {}
                &mut OutputRaw(ref mut val) => {
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
                    if end_ws && res.len() > 0 {
                        res.push(' ');
                    }
                    *val = res;
                }
                other => {},
            }
        }
        Body {
            statements: statements.into_iter().filter(empty).collect(),
            .. body
        }
    }

    pub fn process(&self, opt: &Options, body: Body) -> Body {
        self.visit_body(body, true)
    }

}
