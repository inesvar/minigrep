use super::search;
use std::env;
use std::fs;

#[derive(Default, Debug, PartialEq)]
/// Arguments for a minimal "grep" functionality.
///
/// These arguments namely represent:
/// - the pattern used to select matching lines;
/// - the input file whose lines will be evaluated;
/// - case sensitivity;
/// - ...
pub struct GrepArgs {
    query: String,
    file_path: String,
    ignore_case: bool,
}

/// Prints lines described by the [GrepArgs] `args`.
pub fn run(args: &GrepArgs) -> Result<(), String> {
    let content = fs::read_to_string(&args.file_path).map_err(|err| err.to_string())?;
    // println!(
    //     "{} first line: {:?}",
    //     args.file_path,
    //     content.lines().next().unwrap_or_default()
    // );

    if args.ignore_case {
        search::print_matching_lines_case_insensitive(&args.query, &content);
    } else {
        search::print_matching_lines(&args.query, &content);
    }

    Ok(())
}

impl GrepArgs {
    /// Creates a new `GrepArgs`.
    pub fn new(query: String, file_path: String, ignore_case: bool) -> Self {
        Self {
            query,
            file_path,
            ignore_case,
        }
    }

    /// Creates a new `GrepArgs`.
    pub fn new_from_str(query: &str, file_path: &str, ignore_case: bool) -> Self {
        Self::new(String::from(query), String::from(file_path), ignore_case)
    }

    /// Builds `GrepArgs` from arguments returned by `env::args()`, expecting first the pattern then a file path.
    ///
    /// Case can be ignored by setting environment variable `IGNORE_CASE`.
    /// # Examples
    /// Suppose you run the following command:
    /// ```text
    /// cargo run -- abcd filename
    /// ```
    /// Then the following assertion holds:
    /// ```
    /// # use minigrep::GrepArgs;
    /// # use std::env;
    /// let args = env::args();
    /// # let args = vec![String::from("target/debug/minigrep"), String::from("abcd"), String::from("filename")].into_iter();
    /// assert_eq!(GrepArgs::build(args), Ok(GrepArgs::new_from_str("abcd", "filename", false)));
    /// ```
    /// Here is the explicit content of the `args` variable in the example above:
    /// ```
    /// # use minigrep::GrepArgs;
    /// let args = vec![String::from("target/debug/minigrep"), String::from("abcd"), String::from("filename")].into_iter();
    /// # assert_eq!(GrepArgs::build(args), Ok(GrepArgs::new_from_str("abcd", "filename", false)));
    /// ```
    /// Note that you can ignore case by setting environment variable `IGNORE_CASE`.
    /// ```text
    /// IGNORE_CASE=0 cargo run -- abcd filename
    /// ```
    /// ```no_run
    /// # use minigrep::GrepArgs;
    /// # use std::env;
    /// let args = env::args();
    /// assert_eq!(GrepArgs::build(args), Ok(GrepArgs::new_from_str("abcd", "filename", true)));
    /// ```
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        args.next(); // ignoring the 1st argument (executable path)
        let query = get_next_arg(&mut args)?;
        let file_path = get_next_arg(&mut args)?;
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // println!("Searching '{query}' in '{file_path} (ignore case ? {ignore_case:?})'.");
        Ok(Self::new(query, file_path, ignore_case))
    }

    /// Returns the query used to select matching lines.
    /// # Examples
    /// ```
    /// # use minigrep::GrepArgs;
    /// let args = GrepArgs::new_from_str("abcd", "filename", false);
    /// assert_eq!(args.query(), "abcd");
    /// ```
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Returns the path of the input file.
    /// # Examples
    /// ```
    /// # use minigrep::GrepArgs;
    /// let args = GrepArgs::new_from_str("abcd", "filename", false);
    /// assert_eq!(args.file_path(), "filename");
    /// ```
    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    /// Returns whether the case is ignored.
    /// # Examples
    /// ```
    /// # use minigrep::GrepArgs;
    /// let args = GrepArgs::new_from_str("abcd", "filename", false);
    /// assert_eq!(args.ignore_case(), false);
    /// ```
    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}

fn get_next_arg(args: &mut impl Iterator<Item = String>) -> Result<String, String> {
    args.next()
        .ok_or_else(|| GrepArgs::missing_args_message(args))
}

impl GrepArgs {
    fn missing_args_message(args: &mut impl Iterator<Item = String>) -> String {
        format!(
            "Missing arguments (see `GrepArgs::build` documentation), got: {:?}.",
            args.collect::<Vec<_>>()
        )
    }
}
