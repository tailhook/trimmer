use {Variable, Parser, Context};

fn render_x_y<A, B>(template: &str, x: A, y: B) -> String
    where A: for<'x> Variable<'x>, B: for<'x> Variable<'x>
{
    let tpl = Parser::new().parse(template).unwrap();
    let mut vars: Context = Context::new();
    vars.set("x", &x);
    vars.set("y", &y);
    tpl.render(&vars).unwrap()
}

#[test]
fn render_plus_same_types() {
    assert_eq!(render_x_y("{{ x + y }}", 1u32, 1u32), "2");
    assert_eq!(render_x_y("{{ x + y }}", 10u64, 7u64), "17");
    assert_eq!(render_x_y("{{ x + y }}", 10i64, -1i64), "9");
    assert_eq!(render_x_y("{{ x + y }}", 1.5f64, -1f64), "0.5");
}

#[test]
fn render_plus_different_types() {
    assert_eq!(render_x_y("{{ x + y }}", 1u32, 1f64), "2");
    assert_eq!(render_x_y("{{ x + y }}", 10u64, 7u64), "17");
    assert_eq!(render_x_y("{{ x + y }}", 10u64, -1i64), "9");
    assert_eq!(render_x_y("{{ x + y }}", 1.5f64, -1i64), "0.5");
    assert_eq!(render_x_y("{{ x + y }}", 10u64, -100i64), "-90");
    assert_eq!(render_x_y("{{ x + y }}", -100i64, 10i64), "-90");
}

#[test]
fn render_minus() {
    assert_eq!(render_x_y("{{ x - y }}", 1u32, 1u32), "0");
    assert_eq!(render_x_y("{{ x - y }}", 10u64, 7u64), "3");
    assert_eq!(render_x_y("{{ x - y }}", 10i64, -1i64), "11");
    assert_eq!(render_x_y("{{ x - y }}", 1.5f64, -1f64), "2.5");
}
