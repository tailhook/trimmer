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

#[test]
fn escape() {
    let t = parse("{{ '{{' }}");
    assert_eq!(&t.render(&Context::new()).unwrap(), "{{");
}

#[test]
fn number() {
    let t = parse("{{ 42 }}");
    assert_eq!(&t.render(&Context::new()).unwrap(), "42");
}

#[test]
fn comment() {
    let t = parse("{{ x }} {# +  {{ y }} #} + {{ z }}");
    let x = "2";
    let y = "3";
    let z = "4";
    let mut c = Context::new();
    c.set("x", &x);
    c.set("y", &y);
    c.set("z", &z);
    assert_eq!(&t.render(&c).unwrap(), "2  + 4");
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
fn cond_else() {
    let t = parse("## if x\n  y\n## else\n  z\n## endif\n");
    let empty = "";
    let x = "x";
    let mut c = Context::new();
    c.set("x", &empty);
    assert_eq!(&t.render(&c).unwrap(), "  z\n");
    c.set("x", &x);
    assert_eq!(&t.render(&c).unwrap(), "  y\n");
}

#[test]
fn cond_not() {
    let t = parse("## if not x\n  y\n## else\n  z\n## endif\n");
    let empty = "";
    let x = "x";
    let mut c = Context::new();
    c.set("x", &empty);
    assert_eq!(&t.render(&c).unwrap(), "  y\n");
    c.set("x", &x);
    assert_eq!(&t.render(&c).unwrap(), "  z\n");
}

#[test]
fn print_not() {
    let t = parse("{{ not x }}");
    let empty = "";
    let x = "x";
    let mut c = Context::new();
    c.set("x", &empty);
    assert_eq!(&t.render(&c).unwrap(), "true");
    c.set("x", &x);
    assert_eq!(&t.render(&c).unwrap(), "false");
}

#[test]
fn cond_elif() {
    let t = parse("## if x\n  y\n## elif 1\n  z\n## endif\n");
    let empty = "";
    let x = "x";
    let mut c = Context::new();
    c.set("x", &empty);
    assert_eq!(&t.render(&c).unwrap(), "  z\n");
    c.set("x", &x);
    assert_eq!(&t.render(&c).unwrap(), "  y\n");
}

#[test]
fn iteration() {
    let t = parse("## for x in items\n  - {{ x }}\n## endfor\n");
    let v = vec!["a", "b"];
    let mut c = Context::new();
    c.set("items", &v);
    assert_eq!(&t.render(&c).unwrap(), "  - a\n  - b\n");
}

#[test]
fn pair_iteration() {
    let t = parse("## syntax: indent\n\
                   ## for k, v in items\n  {{ k }}: {{ v }}\n## endfor\n");
    let mut v = HashMap::new();
    v.insert("x", 1);
    v.insert("y", 2);
    let mut c = Context::new();
    c.set("items", &v);
    let result = t.render(&c).unwrap();
    println!("Result:\n{}", result);
    assert!(result == "x: 1\ny: 2\n" || result == "y: 2\nx: 1\n");
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

#[test]
#[cfg(feature="serde")]
fn iterations() {
    assert_eq!(
        render_json(r#"## syntax: indent
## for x in list
    - {{ x }}
## endfor
## for k, v in map
    - {{ k }}: {{ v }}
## endfor
"#,
            r#"{"map": {"a": 2, "b": "x+73"}, "list": ["b", 7]}"#),
        "\
- b
- 7
- a: 2
- b: x+73
");
}
