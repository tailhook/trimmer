use grammar::{Parser};
use {Context};

fn render_x(template: &str, x: &str) -> String {
    let tpl = Parser::new().parse(template).unwrap();
    let mut vars: Context = Context::new();
    vars.set("x", &x);
    tpl.render(&vars).unwrap()
}

#[test]
fn filter_default() {
    assert_diff!(
        &render_x("## syntax: oneline\n\
                   ## filter default: builtin.html_entities\n\
                   {{ x }}", "<a>"),
        "&lt;a&gt;", "\n", 0);
}

#[test]
fn filter_html() {
    assert_diff!(
        &render_x("## syntax: oneline\n\
                   ## filter h: builtin.html_entities\n\
                   {{ x | h }}", "<a>"),
        "&lt;a&gt;", "\n", 0);
}

#[test]
fn filter_shell_argument() {
    assert_diff!(
        &render_x("## syntax: oneline\n\
                   ## filter arg: builtin.quoted_shell_argument\n\
                   echo {{ x | arg }}", "don't crash"),
        r#"echo 'don'"'"'t crash'"#, "\n", 0);
}
