use std::collections::HashMap;


use {Template, Parser};


fn parse(data: &'static str) -> Template {
    Parser::new().parse(data).unwrap()
}

#[test]
fn hello() {
    let t = parse("hello");
    assert_eq!(&t.render(&HashMap::<_, String>::new()).unwrap(),
               "hello");
}

#[test]
fn var_borrow() {
    let t = parse("a{{ x }}b");
    let x = String::from("+");
    let mut c = HashMap::new();
    c.insert("x", &x);
    assert_eq!(&t.render(&c).unwrap(), "a+b");
}

#[test]
fn var_owned() {
    let t = parse("a{{ y }}b");
    let mut c = HashMap::new();
    c.insert("y", String::from("-"));
    assert_eq!(&t.render(&c).unwrap(), "a-b");
}

#[test]
fn var_str() {
    let t = parse("a{{ z }}b");
    let mut c = HashMap::new();
    c.insert("z", "*");
    assert_eq!(&t.render(&c).unwrap(), "a*b");
}

#[test]
fn const_str() {
    let t = parse(r#"a{{ " and " }}b"#);
    let c = HashMap::<_, String>::new();
    assert_eq!(&t.render(&c).unwrap(), "a and b");
}
