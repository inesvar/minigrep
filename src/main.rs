use minigrep::search;
use minigrep::search_case_insensitive;
use std::env;
use std::fs;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let grep_args = GrepArgs::build(&args)?;

    run(grep_args)
}

fn run(args: GrepArgs) -> Result<(), String> {
    let text = match fs::read_to_string(&args.file_path) {
        Ok(result) => result,
        Err(error) => return Err(error.to_string()),
    };
    println!(
        "{} first line: {:?}",
        args.file_path,
        text.lines().next().unwrap_or_default()
    );

    let results = if args.ignore_case {
        search_case_insensitive(&args.query, &text)
    } else {
        search(&args.query, &text)
    };
    for line in results {
        println!("\t{line}");
    }

    Ok(())
}

struct GrepArgs {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl GrepArgs {
    fn build(args: &[String]) -> Result<Self, String> {
        let query = get_arg(args, 1)?;
        let file_path = get_arg(args, 2)?;
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        println!("Searching '{query}' in '{file_path} (ignore case ? {ignore_case:?})'.");
        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn get_arg(args: &[String], index: usize) -> Result<String, String> {
    Ok(args.get(index).ok_or(get_help_message(args))?.clone())
}

fn get_help_message(args: &[String]) -> String {
    format!(
        "Expected at least two arguments (a filename and a string pattern), got : {:?}.",
        &args[1..]
    )
}
