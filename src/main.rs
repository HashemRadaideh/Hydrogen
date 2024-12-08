//! Main entry point for the Hydrogen programming language.
//!
//! Hydrogen is a simple programming language with a REPL (Read-Eval-Print Loop) for interactive use.
//!
//! The program can be run in either REPL mode or by providing a script file for execution.
//! The REPL mode allows users to enter Hydrogen code interactively, while script mode reads and
//! executes code from a specified file.
//!
//! # Usage
//!
//! - `hydrogen -m [mode] -r [run]`
//!   - `-m`, `--mode`: Specify the cursor mode for the REPL (default is "normal").
//!   - `-r`, `--run`: Specify the mode to run the program in ("repl" for REPL, script file path for script mode).
//!
//! # Examples
//!
//! - Run in REPL mode with normal cursor mode:
//!   ```
//!   hydrogen -m normal -r repl
//!   ```
//! - Run a Hydrogen script file:
//!   ```
//!   hydrogen -r path/to/script.hydro
//!   ```

#![warn(missing_docs)]

use std::{fs, io::Result, path::Path};

use clap::Parser;

mod hash;
mod repl;

use hash::evaluator::Evaluator;
use repl::repl;

/// Command-line options for the Hydrogen program.
#[derive(Parser, Debug)]
#[clap(name = "hydrogen", about = "A simple programming language!")]
struct Opt {
    /// Specify the cursor mode for the REPL (default is "normal").
    #[clap(short = 'm', long = "mode", default_value = "normal")]
    mode: String,
    /// Specify the mode to run the program in ("repl" for REPL, script file path for script mode).
    #[clap(short = 'r', long = "run", default_value = "")]
    run: String,
}

/// Main function for the Hydrogen program.
fn main() -> Result<()> {
    // Parse command-line options using Clap.
    let opt = Opt::parse();

    // Check if the program is running in REPL mode or script mode.
    if opt.run == "repl" {
        // Run the REPL with the specified cursor mode.
        repl(opt.mode)?;
    } else {
        // Read and validate code from the specified script file.
        let path = fs::read_to_string(Path::new("test/hello.hy")).unwrap();
        let mut evaluator = Evaluator::new(&path);
        evaluator.eval();
    }

    Ok(())
}
