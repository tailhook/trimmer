use {Template, Parser};

fn parse(template: &str) -> Template {
    Parser::new().parse(template).unwrap()
}

#[test]
fn syntax_indent() {
    parse("## syntax: indent\n");
}

#[test]
fn syntax_oneline() {
    parse("## syntax: oneline\n");
}

#[test]
fn syntax_plain() {
    parse("plain\n");
}

#[test]
#[should_panic(expected="InvalidSyntaxDirective")]
fn syntax_unknown() {
    parse("## syntax: unknown\n");
}
