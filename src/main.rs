use std::env;
use std::fs;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let grep_args = parse_arguments(&args)?;

    let text = match fs::read_to_string(&grep_args.file_path) {
        Ok(result) => result,
        Err(error) => return Err(error.to_string()),
    };
    println!(
        "{} first line: {:?}",
        grep_args.file_path,
        text.lines().next().unwrap_or_default()
    );

    Ok(())
}

struct GrepArgs {
    query: String,
    file_path: String,
}

fn parse_arguments(args: &[String]) -> Result<GrepArgs, String> {
    let Some([_, query, file_path]) = args.first_chunk::<3>() else {
        return Err(get_help_message(args));
    };
    println!("Searching '{query}' in '{file_path}'.");
    Ok(GrepArgs {
        query: query.to_string(),
        file_path: file_path.to_string(),
    })
}

fn get_help_message(args: &[String]) -> String {
    format!(
        "Expected at least two arguments (a filename and a string pattern), got : {:?}.",
        &args[1..]
    )
}
