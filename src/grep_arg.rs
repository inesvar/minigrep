use super::search;
use std::env;
use std::fs;

#[derive(Default)]
pub struct GrepArgs {
    query: String,
    file_path: String,
    ignore_case: bool,
}

pub fn run(args: &GrepArgs) -> Result<(), String> {
    let contents = fs::read_to_string(&args.file_path).map_err(|err| err.to_string())?;
    // println!(
    //     "{} first line: {:?}",
    //     args.file_path,
    //     contents.lines().next().unwrap_or_default()
    // );

    let results = if args.ignore_case {
        search::search_case_insensitive(&args.query, &contents)
    } else {
        search::search(&args.query, &contents)
    };
    for line in results {
        println!("{line}");
    }

    Ok(())
}

impl GrepArgs {
    pub fn build(args: &[String]) -> Result<Self, String> {
        let query = get_arg(args, 1)?;
        let file_path = get_arg(args, 2)?;
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // println!("Searching '{query}' in '{file_path} (ignore case ? {ignore_case:?})'.");
        Ok(Self::new(query, file_path, ignore_case))
    }

    pub fn new(query: &str, file_path: &str, ignore_case: bool) -> Self {
        Self {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case,
        }
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}

fn get_arg(args: &[String], index: usize) -> Result<&String, String> {
    args.get(index).ok_or(get_help_message(args))
}

fn get_help_message(args: &[String]) -> String {
    format!(
        "Expected at least two arguments (a filename and a string pattern), got : {:?}.",
        &args[1..]
    )
}
