use std::collections::HashMap;

use grammar::ExprCode::*;
use grammar::StatementCode::*;
use grammar::{Parser, Template, Syntax, Expr, Statement};

fn parse(data: &'static str) -> Vec<Statement> {
    Parser::new().parse(data).unwrap().body.statements
}

#[test]
fn empty() {
    assert_eq!(parse("hello"), vec![]);
}

#[test]
fn hello() {
    assert_eq!(parse("hello"), vec![]);
}
