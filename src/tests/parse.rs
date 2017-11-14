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
fn syntax_if_at_start() {
    parse("## if x\n## endif\n");
}

#[test]
fn syntax_if_not_at_start() {
    parse("\n    ## if x\n    ## endif\n");
}

#[test]
fn keyword_is_subscring_of_var() {
    parse("\n    ## if index\n    ## endif\n");
}

#[test]
#[should_panic(expected="InvalidSyntaxDirective")]
fn syntax_unknown() {
    parse("## syntax: unknown\n");
}

#[test]
fn send_and_sync() {
    fn send<T: Send>(_: &T) {}
    fn sync<T: Sync>(_: &T) {}
    let t = parse("");
    send(&t);
    sync(&t);
}
