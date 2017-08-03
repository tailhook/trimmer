
#[cfg(feature="serde")]
fn render_json(template: &str, value: &str) -> String {
    use {Parser, Context};
    use render::extract;

    use serde_json;
    let tpl = Parser::new().parse(template).unwrap();
    println!("Template {:#?}", extract(tpl));
    let tpl = Parser::new().parse(template).unwrap();
    let json = serde_json::from_str::<serde_json::Value>(value).unwrap();
    let mut vars: Context = Context::new();
    for (k, v) in json.as_object().unwrap() {
        vars.set(k, v);
    }
    tpl.render(&vars).unwrap()
}

#[cfg(feature="serde")]
fn stdvars(template: &str) -> String {
    render_json(&format!("## syntax: oneline\n{}", template),
        r#"{"x": "1"}"#)
}

#[test]
#[cfg(feature="serde")]
fn text_only() {
    assert_eq!(stdvars(r#"
        just some
            text
    "#), "just some text");
}

#[test]
#[cfg(feature="serde")]
fn var_and_ws() {
    assert_eq!(stdvars("   {{ x }} "), "1");
}
#[test]
#[cfg(feature="serde")]
fn var_at_start() {
    assert_eq!(stdvars("{{ x }}x"), "1x");
}

#[test]
#[cfg(feature="serde")]
fn var_at_end() {
    assert_eq!(stdvars("x{{ x }}"), "x1");
}

#[test]
#[cfg(feature="serde")]
fn start_spaces() {
    assert_eq!(stdvars("  x{{ x }}"), "x1");
}

#[test]
#[cfg(feature="serde")]
fn var_start_spaces() {
    assert_eq!(stdvars("  x  {{ x }}"), "x 1");
}

#[test]
#[cfg(feature="serde")]
fn start_var_spaces() {
    assert_eq!(stdvars("x  {{ x }}"), "x 1");
}

#[test]
#[cfg(feature="serde")]
fn end_spaces() {
    assert_eq!(stdvars("{{ x }}x   "), "1x");
}

#[test]
#[cfg(feature="serde")]
fn var_end_spaces() {
    assert_eq!(stdvars("{{ x }}  x   "), "1 x");
}

#[test]
#[cfg(feature="serde")]
fn end_var_spaces() {
    assert_eq!(stdvars("{{ x }} x"), "1 x");
}

#[test]
#[cfg(feature="serde")]
fn few_vars() {
    assert_eq!(render_json(r#"## syntax: oneline
        {{ hello }} /
        8 -

        {{ world }}+{{ x }}
    "#, r#"{"hello": 1, "world": 2, "x": "3" }"#),
    "1 / 8 - 2+3");
}

#[test]
#[cfg(feature="serde")]
fn if_spaces() {
    assert_eq!(stdvars(r#"
## if x
            {{ x }}
## endif
    "#), "1");
}
