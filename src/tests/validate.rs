use grammar::{Parser};
use {Context};

fn render_x(template: &str, x: &str) -> String {
    let tpl = Parser::new().parse(template).unwrap();
    let mut vars: Context = Context::new();
    vars.set("x", &x);
    tpl.render(&vars).unwrap()
}

#[test]
fn valid_default() {
    assert_diff!(
        &render_x(r#"## syntax: oneline
                 ## validate default: [a-z]+
                 {{ x }}"#, "hello"),
        "hello", "\n", 0);
}

#[test]
#[should_panic]
fn valid_default_non_matching() {
    render_x("## syntax: oneline\n\
              ## validate default: [a-z]+\n\
              {{ x }}", "a+b");
}
