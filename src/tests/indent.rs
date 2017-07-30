use tests::assert_eq;

use render::extract;
use grammar::StatementCode::*;
use grammar::{Parser, Statement};
use {Pos, Context};

fn parse(data: &'static str) -> Vec<Statement> {
    extract(Parser::new().parse(data).unwrap()).body.statements
}

#[cfg(feature="serde")]
fn render_json(template: &str, value: &str) -> String {
    use serde_json;
    let tpl = Parser::new()
        .parse(&format!("## syntax: indent\n{}", template)).unwrap();
    let json = serde_json::from_str::<serde_json::Value>(value).unwrap();
    let mut vars: Context = Context::new();
    for (k, v) in json.as_object().unwrap() {
        vars.set(k, v);
    }
    tpl.render(&vars).unwrap()
}

fn lines(line_st: usize, start: usize, line_end: usize, end: usize)
    -> (Pos, Pos)
{
    (Pos { line: line_st, column: start },
     Pos { line: line_end, column: end })
}

#[test]
fn empty() {
    assert_eq(parse(""), vec![]);
}

#[test]
fn hello() {
    assert_eq(parse("hello\n  world"), vec![Statement {
        position: lines(1, 1, 2, 8),
        code: OutputRaw("hello\n  world".into()),
    }]);
}

#[test]
#[cfg(feature="serde")]
fn indented_if() {
    assert_eq(
        render_json(r#"
hello:
    a: 1
    ## if x
        x: {{ x }}
        y: {{ x }}+x
    ## endif
"#, r#"{"x": 2}"#).lines().collect(),
        "
hello:
    a: 1
    x: 2
    y: 2+x
".lines().collect());
}
