use std::collections::HashMap;
use {Parser, Context, Variable};

fn render_var<'x, V: Variable<'x> + 'x>(template: &str, value: &'x V)
    -> String
{
    let tpl = Parser::new().parse(template).unwrap();
    let mut vars: Context = Context::new();
    vars.set("x", value);
    tpl.render(&vars).unwrap()
}

#[test]
fn render_ip() {
    use std::net::{IpAddr, Ipv4Addr};
    assert_eq!(
        render_var("{{x}}", &IpAddr::from(Ipv4Addr::new(127, 0, 0, 1))),
        "127.0.0.1");
}

#[test]
#[should_panic(expected="VariableNotFound")]
fn render_unknown_var() {
    render_var("{{ x }} {{ y }}", &String::from("x"));
}

#[test]
fn render_sockaddr() {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    assert_eq!(render_var("{{x}}",
        &SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 80)),
        "127.0.0.1:80");
}

#[test]
fn render_str() {
    assert_eq!(render_var("{{x}}", &"xxyy"), "xxyy");
}

#[test]
fn render_opt() {
    assert_eq!(render_var("{{x}}", &Some("hello")), "hello");
}

#[test]
fn undefined_attrs() {
    let p = Parser::new();
    let tpl = p.parse(r#"## syntax: oneline
        k: {{ x.k1 }},
        k2: {{ x.k2 }},
        k3.b: {{ x.k3.b }}
    "#).unwrap();
    let x: HashMap<String, String> = vec![
        ("k1".into(), "v".into()),
    ].into_iter().collect();
    let mut vars: Context = Context::new();
    vars.set("x", &x);
    assert_eq!(tpl.render(&vars).unwrap(), "k: v, k2: , k3.b: ");
}

#[test]
#[cfg(feature="json")]
fn undefined_attrs_serde() {
    use serde_json::from_str;
    use render_json;

    let p = Parser::new();
    let tpl = p.parse(r#"## syntax: oneline
        k: {{ x.k1 }},
        k2: {{ x.k2 }},
        k3.b: {{ x.k3.b }}
    "#).unwrap();
    assert_eq!(
        render_json(&tpl, &from_str(r#"{"x":{"k1":123}}"#).unwrap()).unwrap(),
        "k: 123, k2: , k3.b: ");
}

#[test]
fn undefined_str_index() {
    let p = Parser::new();
    let tpl = p.parse(r#"## syntax: oneline
        k: {{ x['k1'] }},
        k2: {{ x['k2'] }},
        k3.b: {{ x['k3'].b }}
    "#).unwrap();
    let x: HashMap<String, String> = vec![
        ("k1".into(), "v".into()),
    ].into_iter().collect();
    let mut vars: Context = Context::new();
    vars.set("x", &x);
    assert_eq!(tpl.render(&vars).unwrap(), "k: v, k2: , k3.b: ");
}

#[test]
#[cfg(feature="json")]
fn undefined_str_index_serde() {
    use serde_json::from_str;
    use render_json;

    let p = Parser::new();
    let tpl = p.parse(r#"## syntax: oneline
        k: {{ x['k1'] }},
        k2: {{ x['k2'] }},
        k3.b: {{ x['k3'].b }}
    "#).unwrap();
    assert_eq!(
        render_json(&tpl, &from_str(r#"{"x":{"k1":123}}"#).unwrap()).unwrap(),
        "k: 123, k2: , k3.b: ");
}

#[test]
fn undefined_int_index() {
    let p = Parser::new();
    let tpl = p.parse(r#"## syntax: oneline
        2: {{ x[1] }},
        3: {{ x[2] }},
        3.b: {{ x[2].b }}
    "#).unwrap();
    let x: Vec<String> = vec![
        "v1".into(),
        "v2".into(),
    ];
    let mut vars: Context = Context::new();
    vars.set("x", &x);
    assert_eq!(tpl.render(&vars).unwrap(), "2: v2, 3: , 3.b: ");
}

#[test]
#[cfg(feature="json")]
fn undefined_int_index_serde() {
    use serde_json::from_str;
    use render_json;

    let p = Parser::new();
    let tpl = p.parse(r#"## syntax: oneline
        2: {{ x[1] }},
        3: {{ x[2] }},
        3.b: {{ x[2].b }}
    "#).unwrap();
    assert_eq!(
        render_json(&tpl, &from_str(r#"{"x":[2, 3]}"#).unwrap()).unwrap(),
        "2: 3, 3: , 3.b: ");
}

#[test]
#[should_panic(expected="IntKeyUnsupported")]
fn str_key_in_list() {
    let p = Parser::new();
    let tpl = p.parse(r#"## syntax: oneline
        k: {{ x['k1'] }},
    "#).unwrap();
    let x: Vec<String> = vec![];
    let mut vars: Context = Context::new();
    vars.set("x", &x);
    tpl.render(&vars).unwrap();
}
