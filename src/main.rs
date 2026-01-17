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
        let Some([_, query, file_path]) = args.first_chunk::<3>() else {
            return Err(get_help_message(args));
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        println!("Searching '{query}' in '{file_path} (ignore case ? {ignore_case:?})'.");
        Ok(Self {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case,
        })
    }
}

fn get_help_message(args: &[String]) -> String {
    format!(
        "Expected at least two arguments (a filename and a string pattern), got : {:?}.",
        &args[1..]
    )
}
