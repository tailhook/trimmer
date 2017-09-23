#![feature(test)]

extern crate trimmer;
extern crate test;
extern crate serde_json;

use std::str::FromStr;
use trimmer::{Template, Context};
use serde_json::Value;
use test::Bencher;


fn template() -> Template {
    trimmer::Parser::new().parse(r###"## syntax: indent
<!DOCTYPE html>
<html>
  <head>
    <title>Listing of the directory {{ path }}</title>
  </head>
  <body>
    <h1>Listing of the directory {{ path }}</h1>
    <ul>
      ## for entry in entries
      <li>
        ## if entry.is_dir
          <a href="{{ path }}/{{ entry.name }}/">{{ entry.name }}/</a>
        ## else
          <a href="{{ path }}/{{ entry.name }}">{{ entry.name }}</a>
        ## endif
        {# TODO(tailhook) add some file attributes #}
      </li>
      ## endfor
    </ul>
    <hr>
    <p>Yours faithfully,<br>
        swindon web server
    </p>
  </body>
</html>
    "###).unwrap()
}

fn data() -> Value {
    Value::from_str(r#"[
        {"name": "file1", "is_dir": false},
        {"name": "file2", "is_dir": false},
        {"name": "file3", "is_dir": false},
        {"name": "file4", "is_dir": false},
        {"name": "file5", "is_dir": false},
        {"name": "file6", "is_dir": false},
        {"name": "dir1", "is_dir": true},
        {"name": "dir2", "is_dir": true},
        {"name": "dir3", "is_dir": true},
        {"name": "dir4", "is_dir": true},
        {"name": "dir5", "is_dir": true},
        {"name": "file7", "is_dir": false},
        {"name": "file8", "is_dir": false},
        {"name": "file9", "is_dir": false},
        {"name": "file10", "is_dir": false}
    ]"#).unwrap()
}

#[test]
fn works() {
    let tpl = template();
    let path = "/my/path";
    let data = data();
    let mut ctx = Context::new();
    ctx.set("path", &path);
    ctx.set("entries", &data);
    tpl.render(&ctx).unwrap();
}

#[bench]
fn render(b: &mut Bencher) {
    let tpl = template();
    let path = "/my/path";
    let data = data();
    let mut ctx = Context::new();
    ctx.set("path", &path);
    ctx.set("entries", &data);
    b.iter(|| {
        tpl.render(&ctx).unwrap();
    });
}
