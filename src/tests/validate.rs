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
        &render_x("## syntax: oneline\n\
                   ## validate default: [a-z]+\n\
                   {{ x }}", "hello"),
        "hello", "\n", 0);
}

#[test]
fn valid_empty() {
    assert_diff!(
        &render_x("## syntax: oneline\n\
                   ## validate empty:\n\
                   {{ '' }}", "hello"),
        "", "\n", 0);
}

#[test]
fn comment_in_validator() {
    assert_diff!(
        &render_x("## syntax: oneline\n\
                   ## validate default: [a-z]+  # safe default\n\
                   {{ x }}", "hello"),
        "hello", "\n", 0);
}

#[test]
fn validator_tokenizing_number() {
    assert_diff!(
        &render_x("## syntax: oneline\n\
                   ## validate default: [0-9a-z]+\n\
                   {{ x }}", "hello"),
        "hello", "\n", 0);
}

#[test]
fn validators_and_comments() {
    assert_diff!(
        &render_x("## syntax: oneline\n\
                   ### Default validator \n\
                   ## validate default: [0-9a-z]+\n\
                   {{ x }}", "hello"),
        "hello", "\n", 0);
}

#[test]
fn validator_tokenizing_caret() {
    assert_diff!(
        &render_x("## syntax: oneline\n\
                   ## validate default: [^0-9]+\n\
                   {{ x }}", "hello"),
        "hello", "\n", 0);
}

#[test]
#[should_panic]
fn valid_default_non_matching() {
    render_x("## syntax: oneline\n\
              ## validate default: [a-z]+\n\
              {{ x }}", "a+b");
}

#[test]
fn valid_non_default_override() {
    render_x("## syntax: oneline\n\
              ## validate default: [a-z]+\n\
              ## validate sum: [a-z+]+\n\
              {{ x | sum }}", "a+b");
}
