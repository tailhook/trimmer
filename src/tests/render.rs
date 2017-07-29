
use {Template, Parser, Context};


fn parse(template: &str) -> Template {
    Parser::new().parse(template).unwrap()
}

#[cfg(feature="serde")]
fn render_json(template: &str, value: &str) -> String {
    use serde_json;
    let tpl = Parser::new().parse(template).unwrap();
    let mut vars: Context = Context::new();
    for (k, v) in serde_json::from_str::<serde_json::Value>(value).unwrap().as_object().unwrap() {
        vars.set(k.clone(), v.clone());
    }
    tpl.render(&vars).unwrap()
}

#[test]
fn hello() {
    let t = parse("hello");
    assert_eq!(&t.render(&Context::new()).unwrap(),
               "hello");
}

#[test]
fn var_owned() {
    let t = parse("a{{ y }}b");
    let mut c = Context::new();
    c.set("y".into(), String::from("-"));
    assert_eq!(&t.render(&c).unwrap(), "a-b");
}

#[test]
fn var_str() {
    let t = parse("a{{ z }}b");
    let mut c = Context::new();
    c.set("z".into(), "*");
    assert_eq!(&t.render(&c).unwrap(), "a*b");
}

#[test]
fn aliasing_vars() {
    let t = parse("## let x = y\na{{ x }}b");
    let mut c = Context::new();
    c.set("y".into(), "+");
    assert_eq!(&t.render(&c).unwrap(), "a+b");
}

#[test]
fn const_str() {
    let t = parse(r#"a{{ " and " }}b"#);
    let c = Context::new();
    assert_eq!(&t.render(&c).unwrap(), "a and b");
}

#[test]
fn cond() {
    let t = parse("## if x\n  y\n## endif\n");
    let mut c = Context::new();
    c.set("x".into(), "");
    assert_eq!(&t.render(&c).unwrap(), "");
    c.set("x".into(), "x");
    assert_eq!(&t.render(&c).unwrap(), "  y\n");
}

#[test]
fn iteration() {
    let t = parse("## for x in items\n  - {{ x }}\n## endfor\n");
    let mut c = Context::new();
    c.set("items".into(), vec!["a", "b"]);
    // TODO(tailhook) fix indentation
    assert_eq!(&t.render(&c).unwrap(), "  - a\n  - b\n");
}

#[test]
#[cfg(feature="serde")]
fn attr() {
    assert_eq!(
        render_json("{{ x.a }} + {{ x.b }}",
            r#"{"x": {"a": 2, "b": 73}}"#),
        "2 + 73");
}
