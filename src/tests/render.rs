use {Template, Parser, Context};


fn parse(data: &'static str) -> Template {
    Parser::new().parse(data).unwrap()
}

#[test]
fn hello() {
    let c = Context::new();
    let t = parse("hello");
    assert_eq!(&t.render(&c).unwrap(),
               "hello");
}

#[test]
fn var_borrow() {
    let x = String::from("+");
    let mut c = Context::new();
    c.add("x", &x);
    let t = parse("a{{ x }}b");
    assert_eq!(&t.render(&c).unwrap(), "a+b");
}

#[test]
fn var_owned() {
    let t = parse("a{{ y }}b");
    let mut c = Context::new();
    c.add("y", String::from("-"));
    assert_eq!(&t.render(&c).unwrap(), "a-b");
}

#[test]
fn var_str() {
    let t = parse("a{{ z }}b");
    let mut c = Context::new();
    c.add("z", "*");
    assert_eq!(&t.render(&c).unwrap(), "a*b");
}

#[test]
fn const_str() {
    let t = parse(r#"a{{ " and " }}b"#);
    let c = Context::new();
    assert_eq!(&t.render(&c).unwrap(), "a and b");
}
