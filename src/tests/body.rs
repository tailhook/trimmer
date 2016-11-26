use std::collections::HashMap;

use grammar::ExprCode::*;
use grammar::StatementCode::*;
use grammar::{Parser, Template, Syntax, Statement};
use {Pos};

fn parse(data: &'static str) -> Vec<Statement> {
    Parser::new().parse(data).unwrap().body.statements
}

fn line(line_no: i32, start: i32, end: i32) ->  (Pos, Pos) {
    (Pos { line: line_no, column: start },
     Pos { line: line_no, column: end })
}

#[test]
fn empty() {
    assert_eq!(parse(""), vec![]);
}

#[test]
fn hello() {
    assert_eq!(parse("hello"), vec![Statement {
        position: line(1, 1, 6),
        code: OutputRaw("hello".into()),
    }]);
}
