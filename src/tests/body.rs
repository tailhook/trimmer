use tests::assert_eq;

use render::extract;
use grammar::ExprCode::*;
use grammar::StatementCode::*;
use grammar::{Parser, Statement, Expr};
use {Pos};

fn parse(data: &'static str) -> Vec<Statement> {
    extract(Parser::new().parse(data).unwrap()).body.statements
}

fn line(line_no: usize, start: usize, end: usize) ->  (Pos, Pos) {
    (Pos { line: line_no, column: start },
     Pos { line: line_no, column: end })
}

#[test]
fn empty() {
    assert_eq(parse(""), vec![]);
}

#[test]
fn hello() {
    assert_eq(parse("hello"), vec![Statement {
        position: line(1, 1, 6),
        code: OutputRaw("hello".into()),
    }]);
}

#[test]
fn var() {
    assert_eq(parse("a{{ x }}b"), vec![
        Statement {
            position: line(1, 1, 2),
            code: OutputRaw("a".into()),
        },
        Statement {
            position: line(1, 2, 9),
            code: Output(Expr {
                position: line(1, 5, 6),
                code: Var(String::from("x")),
            }),
        },
        Statement {
            position: line(1, 9, 10),
            code: OutputRaw("b".into()),
        },
    ]);
}
