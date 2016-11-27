use {Template, Parser, Context};


fn parse(data: &'static str) -> Template {
    Parser::new().parse(data).unwrap()
}

#[test]
fn hello() {
    assert_eq!(&parse("hello").render(&Context::new()).unwrap(),
               "hello");
}
