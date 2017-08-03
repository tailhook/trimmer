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
