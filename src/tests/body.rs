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

    assert_eq(parse("a\n## if x\n  b## endif\n"), vec![
        Statement {
            position: lines(1, 1, 2, 0),
            code: OutputRaw("a\n".into()),
        },
        Statement {
            position: lines(2, 0, 4, 0),
            code: Cond {
                conditional: vec![
                    (Expr {
                        position: line(2, 6, 7),
                        code: Var("x".into()),
                    }, Body {
                        statements: vec![Statement {
                            position: line(3, 0, 3),
                            // TODO(tailhook) no indent
                            code: OutputRaw("  b".into()),
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
