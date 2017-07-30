use std::collections::HashMap;

use {Template, Parser, Context};


fn parse(template: &str) -> Template {
    Parser::new().parse(template).unwrap()
}

#[cfg(feature="serde")]
fn render_json(template: &str, value: &str) -> String {
    use serde_json;
    let tpl = Parser::new().parse(template).unwrap();
    let json = serde_json::from_str::<serde_json::Value>(value).unwrap();
    let mut vars: Context = Context::new();
    for (k, v) in json.as_object().unwrap() {
        vars.set(k, v);
    }
    tpl.render(&vars).unwrap()
}

#[test]
fn hello() {
    let t = parse("hello");
    assert_eq!(&t.render(&Context::new()).unwrap(),
               "hello");
}

/*
#[test]
fn var_owned() {
    let t = parse("a{{ y }}b");
    let mut c = Context::new();
    c.set("y".into(), String::from("-"));
    assert_eq!(&t.render(&c).unwrap(), "a-b");
}
*/

#[test]
fn var_borrow_static() {
    let t = parse("a{{ y }}b");
    let ptr = " / ";
    let mut c = Context::new();
    c.set("y", &ptr);
    assert_eq!(&t.render(&c).unwrap(), "a / b");
}

#[test]
fn var_borrow_hashmap() {
    let t = parse("k1: {{ map.k1 }}, k2: {{ map.k2 }}");
    let mut map = HashMap::new();
    let mut c = Context::new();
    map.insert("k1", "x");
    map.insert("k2", "y");
    c.set("map", &map);
    assert_eq!(&t.render(&c).unwrap(), "k1: x, k2: y");
}

#[test]
fn var_str() {
    let t = parse("a{{ z }}b");
    let star = "*";
    let mut c = Context::new();
    c.set("z", &star);
    assert_eq!(&t.render(&c).unwrap(), "a*b");
}

#[test]
fn aliasing_vars() {
    let t = parse("## let x = y\na{{ x }}b");
    let plus = "+";
    let mut c = Context::new();
    c.set("y", &plus);
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
    let empty = "";
    let x = "x";
    let mut c = Context::new();
    c.set("x", &empty);
    assert_eq!(&t.render(&c).unwrap(), "");
    c.set("x", &x);
    assert_eq!(&t.render(&c).unwrap(), "  y\n");
}

#[test]
fn iteration() {
    let t = parse("## for x in items\n  - {{ x }}\n## endfor\n");
    let v = vec!["a", "b"];
    let mut c = Context::new();
    c.set("items", &v);
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

#[test]
#[cfg(feature="serde")]
fn item() {
    assert_eq!(
        render_json(r#"{{ x["a"] }} + {{ x[key] }}"#,
            r#"{"x": {"a": 2, "b": 73}, "key": "b"}"#),
        "2 + 73");
}
