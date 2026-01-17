use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let Some([_, pattern, filename]) = args.first_chunk::<3>() else {
        return Err(get_help_message(&args));
    };

    println!("Searching '{pattern}' in '{filename}'.");
    Ok(())
}

fn get_help_message(args: &[String]) -> String {
    format!(
        "Expected at least two arguments (a filename and a string pattern), got : {:?}.",
        &args[1..]
    )
}
