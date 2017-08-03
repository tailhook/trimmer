extern crate trimmer;
extern crate argparse;
#[cfg(feature="json")] extern crate serde_json;

use std::io::{Read, Write, stdout, stderr};
use std::fs::File;
use std::path::{Path};
use std::process::exit;

use trimmer::Parser;
#[cfg(feature="json")] use serde_json::Value;


fn main() {
    let mut vars = Vec::<String>::new();
    let mut templates = Vec::<String>::new();
    #[cfg(feature="json")]
    let mut json_vars = Vec::<String>::new();
    let mut output = None::<String>;
    {
        use argparse::*;
        let mut ap = ArgumentParser::new();
        ap.refer(&mut templates)
            .add_argument("template", Collect,
                "Templates to check or render. \
                 If `-o` specified then rendering mode is activated and \
                 only one template is allowed");
        ap.refer(&mut output)
            .add_option(&["-o", "--render-to-file"], ParseOption,
                "Output file to render to. \
                 If specified template will be rendered rather than just \
                 syntax checked. To render to stdout use dash (`-o-`)");
        ap.refer(&mut vars)
            .add_option(&["-D", "--var"], Collect,
                "Define a string variable. Only useful if `-o-` is also
                 specified");
        #[cfg(feature="json")]
        {
            ap.refer(&mut json_vars)
                .add_option(&["-J", "--json"], Collect,
                    "Define set of variables using json dict (object).
                     This option is repeatable, where latter objects
                     override former. `-D` args override json variables");
        }
        ap.parse_args_or_exit();
    }
    let parser = Parser::new();
    // TODO(tailhook) use eprintln when we can upgrade to rust 1.19
    if let Some(out_file) = output {
        if templates.len() != 1 {
            writeln!(&mut stderr(),
                "Exactly one template might be specified when in \
                 render mode (with `-o`/`--render-to_file`)").ok();
            exit(1);
        }
        let path = Path::new(&templates[0]);

        #[cfg(feature="json")]
        let parsed_jsons = {
            let mut v = Vec::new();
            for val in &json_vars {
                match serde_json::from_str::<Value>(val) {
                    Ok(Value::Object(map)) => v.push(map),
                    Ok(val) => {
                        writeln!(&mut stderr(),
                            "Json must contain object, not {:?}", val).ok();
                        exit(1);
                    }
                    Err(e) => {
                        writeln!(&mut stderr(),
                            "Can't parse json: {}", e).ok();
                        exit(1);
                    }
                }
            }
            v
        };


        let mut parsed_vars = Vec::new();
        for pair in vars {
            let mut piter = pair.splitn(2, '=');
            match (piter.next(), piter.next()) {
                (Some(""), _) | (None, _) => {
                    writeln!(&mut stderr(),
                        "Var name must not be empty in {:?}",
                        pair).ok();
                    exit(1);
                }
                (_, None) => {
                    writeln!(&mut stderr(),
                        "Var {:?} must contain equals sign",
                        pair).ok();
                    exit(1);
                }
                (Some(x), Some(y)) => {
                    parsed_vars.push((x.to_string(), y.to_string()));
                }
            }
        }

        let mut buf = String::with_capacity(1024);
        let read = File::open(path)
            .and_then(|mut f| f.read_to_string(&mut buf));
        match read {
            Ok(_) => {},
            Err(e) => {
                writeln!(&mut stderr(), "Error reading {:?}: {}",
                    path, e).ok();
                exit(1);
            }
        }
        let template = match parser.parse(&buf) {
            Ok(tpl) => tpl,
            Err(e) => {
                writeln!(&mut stderr(),
                    "Error parsing {:?}: {}", path, e).ok();
                exit(2);
            }
        };

        let mut context = trimmer::Context::new();
        #[cfg(feature="json")]
        for map in &parsed_jsons {
            for (k, v) in map {
                context.set(k, v);
            }
        }
        for &(ref key, ref value) in &parsed_vars {
            context.set(key, value);
        }

        let buf = match template.render(&context) {
            Ok(value) => value,
            Err(e) => {
                writeln!(&mut stderr(), "Error rendering {:?}: {}",
                    path, e).ok();
                exit(3);
            }
        };

        let res = if out_file == "-" {
            stdout().write_all(buf.as_bytes())
        } else {
            File::create(&out_file)
                .and_then(|mut f| f.write_all(&buf.as_bytes()))
        };

        match res {
            Ok(()) => {}
            Err(e) => {
                writeln!(&mut stderr(), "Error writing output {:?}: {}",
                    out_file, e).ok();
                exit(3);
            }
        }

    } else {

        #[cfg(not(feature="json"))]
        let has_vars = vars.len() > 0;
        #[cfg(feature="json")]
        let has_vars = vars.len() > 0 || json_vars.len() > 0;
        if has_vars {
            println!("No vars allowed in syntax check mode. \
                (Use `-o`/`--render-to-file` to render template)");
            exit(1);
        }

        let mut buf = String::with_capacity(4096);
        let mut code = 0;

        for template in templates {
            let path = Path::new(&template);
            if path.is_dir() {
                println!("{:?} is a directory. \
                    Scanning directories is not iplemented.", path);
                code = 1;
            }
            let read = File::open(path)
                .and_then(|mut f| f.read_to_string(&mut buf));
            match read {
                Ok(_) => {},
                Err(e) => {
                    println!("Error reading {:?}: {}", path, e);
                    code = 1;
                }
            }
            match parser.parse(&buf) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error parsing {:?}: {}", path, e);
                    code = 2;
                }
            }
        }
        exit(code);
    }
}
