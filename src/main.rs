use std::env;
use std::fs;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let (pattern, filename) = parse_arguments(&args)?;

    let text = match fs::read_to_string(filename) {
        Ok(result) => result,
        Err(error) => return Err(error.to_string()),
    };
    println!(
        "{filename} first line: {:?}",
        text.lines().next().unwrap_or_default()
    );

    Ok(())
}

fn parse_arguments(args: &[String]) -> Result<(&str, &str), String> {
    let Some([_, pattern, filename]) = args.first_chunk::<3>() else {
        return Err(get_help_message(args));
    };
    println!("Searching '{pattern}' in '{filename}'.");
    Ok((pattern, filename))
}

fn get_help_message(args: &[String]) -> String {
    format!(
        "Expected at least two arguments (a filename and a string pattern), got : {:?}.",
        &args[1..]
    )
}
