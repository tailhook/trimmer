use tests::assert_eq;

use render::extract;
use grammar::ExprCode::*;
use grammar::StatementCode::*;
use grammar::{Parser, Statement, Expr, Body};
use {Pos};

fn parse(data: &'static str) -> Vec<Statement> {
    extract(Parser::new().parse(data).unwrap()).body.statements
}

fn line(line_no: usize, start: usize, end: usize) ->  (Pos, Pos) {
    (Pos { line: line_no, column: start },
     Pos { line: line_no, column: end })
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
    assert_eq(parse("hello"), vec![Statement {
        position: line(1, 1, 6),
        code: OutputRaw("hello".into()),
    }]);
}

#[test]
#[should_panic(expected="Statement must start at the beginning of the line")]
fn invalid() {
    parse("he ## if x\n## endif\n");
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

#[test]
fn condition() {
    use grammar::StatementCode::*;
    use grammar::ExprCode::*;

    assert_eq(parse("a\n## if x\n  b\n## endif\n"), vec![
        Statement {
            position: lines(1, 1, 2, 1),
            code: OutputRaw("a\n".into()),
        },
        Statement {
            position: lines(2, 1, 5, 1),
            code: Cond {
                indent: 0,
                conditional: vec![
                    (Expr {
                        position: line(2, 7, 8),
                        code: Var("x".into()),
                    }, Body {
                        statements: vec![Statement {
                            position: lines(3, 1, 4, 1),
                            // TODO(tailhook) no indent
                            code: OutputRaw("  b\n".into()),
                        }],
                    }),
                ],
                otherwise: Body {
                    statements: vec![],
                },
            }
        },
    ]);
}

#[test]
fn indented_condition() {
    use grammar::StatementCode::*;
    use grammar::ExprCode::*;

    assert_eq(parse("a\n    ## if x\n  b\n    ## endif\n"), vec![
        Statement {
            position: lines(1, 1, 2, 1),
            code: OutputRaw("a\n".into()),
        },
        Statement {
            position: lines(2, 1, 5, 1),
            code: Cond {
                indent: 4,
                conditional: vec![
                    (Expr {
                        position: line(2, 11, 12),
                        code: Var("x".into()),
                    }, Body {
                        statements: vec![Statement {
                            position: lines(3, 1, 4, 1),
                            // TODO(tailhook) no indent
                            code: OutputRaw("  b\n".into()),
                        }],
                    }),
                ],
                otherwise: Body {
                    statements: vec![],
                },
            }
        },
    ]);
}

#[test]
fn iteration() {
    use grammar::AssignTarget;
    use grammar::StatementCode::*;
    use grammar::ExprCode::*;

    assert_eq(parse("a\n## for x in y\n  - {{ x }}\n## endfor\n"), vec![
        Statement {
            position: lines(1, 1, 2, 1),
            code: OutputRaw("a\n".into()),
        },
        Statement {
            position: lines(2, 1, 5, 1),
            code: Loop {
                indent: 0,
                target: AssignTarget::Var("x".into()),
                iterator: Expr {
                    position: line(2, 13, 14),
                    code: Var("y".into()),
                },
                filter: None,
                body: Body {
                    statements: vec![
                        Statement {
                            position: line(3, 1, 5),
                            // TODO(tailhook) no indent
                            code: OutputRaw("  - ".into()),
                        },
                        Statement {
                            position: line(3, 5, 12),
                            // TODO(tailhook) no indent
                            code: Output(Expr {
                                position: line(3, 8, 9),
                                code: Var("x".into()),
                            }),
                        },
                        Statement {
                            position: lines(3, 12, 4, 1),
                            code: OutputRaw("\n".into()),
                        },
                    ],
                },
            }
        },
    ]);
}

#[test]
fn assign() {
    use grammar::AssignTarget;
    use grammar::StatementCode::*;
    use grammar::ExprCode::*;

    assert_eq(parse("## let x = y\n"), vec![
        Statement {
            position: lines(1, 1, 2, 1),
            code: Alias {
                target: AssignTarget::Var("x".into()),
                value: Expr {
                    position: line(1, 12, 13),
                    code: Var("y".into()),
                },
            },
        },
    ]);
}

#[test]
fn at_eof() {
    use grammar::AssignTarget;
    use grammar::StatementCode::*;
    use grammar::ExprCode::*;

    assert_eq(parse("## let x = y"), vec![
        Statement {
            position: lines(1, 1, 1, 13),
            code: Alias {
                target: AssignTarget::Var("x".into()),
                value: Expr {
                    position: line(1, 12, 13),
                    code: Var("y".into()),
                },
            },
        },
    ]);
}
