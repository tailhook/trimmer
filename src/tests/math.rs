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

#[test]
fn render_mul() {
    assert_eq!(render_x_y("{{ x * y }}", 2u32, 3u32), "6");
}

#[test]
fn render_div() {
    assert_eq!(render_x_y("{{ x / y }}", 6u32, 3u32), "2");
}

#[test]
fn render_mod() {
    assert_eq!(render_x_y("{{ x % y }}", 5u32, 3u32), "2");
}

#[test]
fn render_sum_mul() {
    assert_eq!(render_x_y("{{ 2 + x * y }}", 2u32, 3u32), "8");
}

#[test]
fn render_parenthesis() {
    assert_eq!(render_x_y("{{ (2+x) * y }}", 2u32, 3u32), "12");
}

#[test]
fn render_greater() {
    let x = String::from("x");
    let y = String::from("y");
    assert_eq!(render_x_y("{{ x > y }}", x, y), "false");
    assert_eq!(render_x_y("{{ x > y }}", 2, 1), "true");
    assert_eq!(render_x_y("{{ x > y > 3 }}", 5, 4), "true");
}
