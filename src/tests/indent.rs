use tests::assert_eq;

use render::extract;
use grammar::StatementCode::*;
use grammar::{Parser, Statement};
use Pos;

fn parse(data: &'static str) -> Vec<Statement> {
    extract(Parser::new().parse(data).unwrap()).body.statements
}

#[cfg(feature="json")]
fn render_json(template: &str, value: &str) -> String {
    use serde_json;
    use {Context};

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
#[cfg(feature="json")]
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

#[test]
#[cfg(feature="json")]
fn nested_blocks() {
    assert_eq(
        render_json(r#"
hello:
    a: 1
    ## if x
        {{ x }}: 2
        ## if y
            y: {{ x }}+x
        ## endif
    ## endif
"#, r#"{"x": "x", "y": true}"#).lines().collect(),
        "
hello:
    a: 1
    x: 2
    y: x+x
".lines().collect());
}

#[test]
#[cfg(feature="json")]
fn directly_nested_blocks() {
    assert_eq(
        render_json(r#"
hello:
    a: 1
    ## if x
        ## if y
            x: {{ x }}
            y: {{ x }}+x
        ## endif
    ## endif
"#, r#"{"x": 2, "y": true}"#).lines().collect(),
        "
hello:
    a: 1
    x: 2
    y: 2+x
".lines().collect());
}
