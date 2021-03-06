use std::collections::HashMap;

use {Template, Parser, Context};


fn parse(template: &str) -> Template {
    Parser::new().parse(template).unwrap()
}

#[cfg(feature="json")]
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

fn render_x(template: &str) -> String {
    let tpl = Parser::new().parse(template).unwrap();
    let x = "x";
    let mut vars: Context = Context::new();
    vars.set("x", &x);
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

#[test]
fn line_comment() {
    assert_eq!(render_x("x\n### hello\ny"), "x\ny")
}

#[test]
fn indented_comment() {
    assert_eq!(render_x("x\n    ### hello\ny"), "x\ny")
}

#[test]
fn not_comment() {
    assert_eq!(render_x("x ### hello\ny"), "x\ny")
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
fn parenthesis() {
    let t = parse("## let x = (\ny)\na{{ x }}b");
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
fn cond_comment() {
    let t = parse("## if x #comment\n  y\n## endif\n");
    let x = "x";
    let mut c = Context::new();
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
fn skip_if() {
    let t = parse("## for x in items\n\
                     ## skip if x == 'b'\n\
                     - {{ x }}\n\
                   ## endfor\n");
    let v = vec!["a", "b", "c"];
    let mut c = Context::new();
    c.set("items", &v);
    assert_eq!(&t.render(&c).unwrap(), "- a\n- c\n");
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
#[cfg(feature="json")]
fn attr() {
    assert_eq!(
        render_json("{{ x.a }} + {{ x.b }}",
            r#"{"x": {"a": 2, "b": 73}}"#),
        "2 + 73");
}

#[test]
#[cfg(feature="json")]
fn sum_json() {
    assert_eq!(
        render_json("{{ x.a + x.b }}",
            r#"{"x": {"a": 2, "b": 73}}"#),
        "75");
}

#[test]
#[cfg(feature="json")]
fn prod_json() {
    assert_eq!(
        render_json("{{ x.a * x.b }}",
            r#"{"x": {"a": 2, "b": 73}}"#),
        "146");
}

#[test]
#[cfg(feature="json")]
fn item() {
    assert_eq!(
        render_json(r#"{{ x["a"] }} + {{ x[key] }}"#,
            r#"{"x": {"a": 2, "b": 73}, "key": "b"}"#),
        "2 + 73");
}

#[test]
#[cfg(feature="json")]
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

#[test]
fn strip_right() {
    assert_eq!(render_x(r#"{{ x }}   {{- x }}"#), "xx");
}

#[test]
fn strip_left() {
    assert_eq!(render_x(r#"{{ x }}     {{- x }}"#), "xx");
}

#[test]
fn strip_both() {
    assert_eq!(render_x(r#"{{ x -}}   {{- x }}"#), "xx");
}

#[test]
fn space_left() {
    assert_eq!(render_x(r#"{{ x +}}   {{ x }}"#), "x x");
}

#[test]
fn space_right() {
    assert_eq!(render_x(r#"{{ x }}  {{+ x }}"#), "x x");
}

#[test]
fn space_right_eol() {
    assert_eq!(render_x("{{ x }}\n## if x\n  {{+ x }}\n## endif"), "x x\n");
}

#[test]
fn space_left_eol() {
    assert_eq!(render_x("{{ x +}}\n## if x\n  {{ x }}\n## endif"), "x x\n");
}

#[test]
fn space_both() {
    assert_eq!(render_x(r#"{{ x }}  {{+ x }}"#), "x x");
}

#[test]
fn space_and_strip1() {
    assert_eq!(render_x(r#"{{ x -}}  {{+ x }}"#), "xx");
}

#[test]
fn space_and_strip2() {
    assert_eq!(render_x(r#"{{ x +}}  {{- x }}"#), "xx");
}

#[test]
fn preserve_manually() {
    assert_eq!(render_x(r#"{{ x -}}  {{ "" }}  {{ " " }} {{- x }}"#), "x   x");
}

#[test]
fn line_joiner() {
    assert_eq!(render_x("a  ##\n  b"), "a b");
}

#[test]
fn use_dict() {
    assert_eq!(render_x("## let a = {'x': 1, 'y': 2}\n\
                         {{ a['x'] }} / {{ a['y'] }}"),
        "1 / 2");
}

#[test]
fn use_list() {
    assert_eq!(render_x("## let a = [4, 3, 5]\n\
                         ## for i in a\n\
                         {{ i }}
                         ## endfor"),
        "4\n3\n5\n");
}
