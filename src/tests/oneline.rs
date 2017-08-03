use {Parser, Context};
use render::extract;


#[cfg(feature="json")]
fn render_json(template: &str, value: &str) -> String {
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

fn stdvars(template: &str) -> String {
    let template = format!("## syntax: oneline\n{}", template);
    let tpl = Parser::new().parse(&template).unwrap();
    println!("Template {:#?}", extract(tpl));
    let x = 1;
    let tpl = Parser::new().parse(&template).unwrap();
    let mut vars: Context = Context::new();
    vars.set("x", &x);
    tpl.render(&vars).unwrap()
}

#[test]
fn text_only() {
    assert_eq!(stdvars(r#"
        just some
            text
    "#), "just some text");
}

#[test]
fn var_and_ws() {
    assert_eq!(stdvars("   {{ x }} "), "1");
}
#[test]
fn var_at_start() {
    assert_eq!(stdvars("{{ x }}x"), "1x");
}

#[test]
fn var_at_end() {
    assert_eq!(stdvars("x{{ x }}"), "x1");
}

#[test]
fn start_spaces() {
    assert_eq!(stdvars("  x{{ x }}"), "x1");
}

#[test]
fn var_start_spaces() {
    assert_eq!(stdvars("  x  {{ x }}"), "x 1");
}

#[test]
fn start_var_spaces() {
    assert_eq!(stdvars("x  {{ x }}"), "x 1");
}

#[test]
fn end_spaces() {
    assert_eq!(stdvars("{{ x }}x   "), "1x");
}

#[test]
fn var_end_spaces() {
    assert_eq!(stdvars("{{ x }}  x   "), "1 x");
}

#[test]
fn end_var_spaces() {
    assert_eq!(stdvars("{{ x }} x"), "1 x");
}

#[test]
#[cfg(feature="json")]
fn few_vars() {
    assert_eq!(render_json(r#"## syntax: oneline
        {{ hello }} /
        8 -

        {{ world }}+{{ x }}
    "#, r#"{"hello": 1, "world": 2, "x": "3" }"#),
    "1 / 8 - 2+3");
}

#[test]
fn if_spaces() {
    assert_eq!(stdvars(r#"
## if x
            {{ x }}
## endif
    "#), "1");
}

#[test]
fn two_lines() {
    assert_eq!(stdvars(r#"
            {{ x }}
            {{ x }}
    "#), "1 1");
}

#[test]
fn two_ifs() {
    assert_eq!(stdvars(r#"
## if 1
a
## endif
## if 2
b
## endif
    "#), "a b");
}

#[test]
fn two_ifs_vars() {
    assert_eq!(stdvars(r#"
## if 1
  {{ x }}
## endif
## if 2
  {{ x }}
## endif
    "#), "1 1");
}

#[test]
fn two_ifs_mixed_left() {
    assert_eq!(stdvars(r#"
## if 1
  x
## endif
## if 2
  {{ x }}
## endif
    "#), "x 1");
}

#[test]
fn two_ifs_mixed_right() {
    assert_eq!(stdvars(r#"
## if 1
  {{ x }}
## endif
## if 2
  x
## endif
    "#), "1 x");
}
