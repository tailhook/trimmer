extern crate trimmer;
extern crate argparse;
#[cfg(feature="serde")] extern crate serde_json;

use std::io::Read;
use std::fs::File;
use std::path::{Path};
use std::process::exit;

use trimmer::Parser;


fn main() {
    let mut vars = Vec::<String>::new();
    let mut templates = Vec::<String>::new();
    #[cfg(feature="serde")]
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
        #[cfg(feature="serde")]
        {
            ap.refer(&mut json_vars)
                .add_option(&["-J", "--json"], Collect,
                    "Define set of variables using json dict (object).
                     This option is repeatable, where latter objects
                     override former. `-D` args override json variables");
        }
        ap.parse_args_or_exit();
    }
    if let Some(out_file) = output {
        unimplemented!();
    } else {
        #[cfg(not(feature="serde"))]
        let has_vars = vars.len() > 0;
        #[cfg(feature="serde")]
        let has_vars = vars.len() > 0 || json_vars.len() > 0;
        if has_vars {
            println!("No vars allowed in syntax check mode. \
                (Use `-o`/`--render-to-file` to render template)");
            exit(1);
        }

        let parser = Parser::new();
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
                    code = 1;
                }
            }
        }
        exit(code);
    }
}
