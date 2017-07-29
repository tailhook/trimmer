use tests::assert_eq;

use render::extract;
use grammar::StatementCode::*;
use grammar::{Parser, Statement};
use {Pos};

fn parse(data: &'static str) -> Vec<Statement> {
    extract(Parser::new().parse(data).unwrap()).body.statements
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
        position: lines(1, 1, 2, 7),
        code: OutputRaw("hello\n  world".into()),
    }]);
}
