pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matched_lines = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matched_lines.push(line);
        }
    }
    matched_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn another_one_result() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Rust:"], search(query, contents));
    }

    #[test]
    fn two_results() {
        let query = "u";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Rust:", "safe, fast, productive."],
            search(query, contents)
        );
    }
}
