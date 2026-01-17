use minigrep::{GrepArgs, run};
use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let grep_args = GrepArgs::build(&args)?;

    run(grep_args)
}
