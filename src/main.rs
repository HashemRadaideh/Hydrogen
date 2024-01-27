use std::{fs, io::Result, path::Path};

use clap::Parser;

mod hash;
mod repl;

use hash::{exec, validate};
use repl::repl;

#[derive(Parser, Debug)]
#[clap(name = "hydrogen", about = "A simple programming langauge!")]
struct Opt {
    #[clap(short = 'm', long = "mode", default_value = "normal")]
    mode: String,
    #[clap(short = 'r', long = "run", default_value = "")]
    run: String,
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    if opt.run == "repl" {
        repl(opt.mode)?;
    } else {
        let ast = validate(&fs::read_to_string(Path::new("test/hello.txt")).unwrap());
        match ast {
            Ok(tree) => {
                exec(tree);
            }
            Err(_) => {}
        }
    }

    Ok(())
}
