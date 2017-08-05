use {Parser, Context};


fn render(template: &str) -> String {
    let tpl = Parser::new().parse(template).unwrap();
    let yes = true;
    let no = false;
    let a = "a";
    let b = "b";
    let mut vars: Context = Context::new();
    vars.set("yes", &yes);
    vars.set("no", &no);
    vars.set("a", &a);
    vars.set("b", &b);
    tpl.render(&vars).unwrap()
}

#[test]
fn test_and_true() {
    assert_eq!(render("{{ yes and a }}"), "a");
}

#[test]
fn test_and_false() {
    assert_eq!(render("{{ no and a }}"), "false");
}

#[test]
fn test_or_true() {
    assert_eq!(render("{{ yes or a }}"), "true");
}

#[test]
fn test_or_false() {
    assert_eq!(render("{{ no or a }}"), "a");
}

#[test]
fn test_and_or_a() {
    assert_eq!(render("{{ yes and a or b }}"), "a");
}

#[test]
fn test_and_or_b() {
    assert_eq!(render("{{ no and a or b }}"), "b");
}

#[test]
fn test_and_or_false() {
    assert_eq!(render("{{ yes and no or b }}"), "b");
}
