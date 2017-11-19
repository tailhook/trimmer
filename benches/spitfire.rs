//! Benchmark from youtube/spitfire

#![feature(test)]

extern crate trimmer;
extern crate test;
#[macro_use] extern crate serde_json;

use trimmer::{Template, Context};
use serde_json::Value;
use test::Bencher;


fn template() -> Template {
    trimmer::Parser::new().parse(r###"## syntax: indent
<table>
    ## for row in table
        <tr>
            ## for _, column in row
                <td>{{ column }}</td>
            ## endfor
        </tr>
    ## endfor
</table>
    "###).unwrap()
}

#[bench]
fn render(b: &mut Bencher) {
    let tpl = template();
    let row = json!({
        "a": 1,
        "b": 2,
        "c": 3,
        "d": 4,
        "e": 5,
        "f": 6,
        "g": 7,
        "h": 8,
        "i": 9,
        "j": 10,
    });
    let table = Value::Array((0..1000).map(|_| row.clone()).collect());
    let mut ctx = Context::new();
    ctx.set("table", &table);
    b.iter(|| {
        tpl.render(&ctx).unwrap();
    });
}
