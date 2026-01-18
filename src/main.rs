use minigrep::GrepArgs;
use minigrep::run;
use std::env;

fn main() -> Result<(), String> {
    let grep_args = GrepArgs::build(env::args())?;

    run(&grep_args)
}
