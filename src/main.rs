use std::io::Result;

use clap::Parser;

mod hash;
mod repl;

use repl::repl;

#[derive(Parser, Debug)]
#[clap(name = "hydrogen", about = "A simple programming langauge!")]
struct Opt {
    #[clap(short = 'm', long = "mode", default_value = "normal")]
    mode: String,
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    repl(opt.mode)?;

    Ok(())
}
