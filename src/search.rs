pub(super) fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matched_lines = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matched_lines.push(line);
        }
    }
    matched_lines
}

pub(super) fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matched_lines = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            matched_lines.push(line);
        }
    }
    matched_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.";

    #[test]
    fn search_has_one_result() {
        let query = "duct";
        assert_eq!(vec!["safe, fast, productive."], search(query, CONTENTS));
    }

    #[test]
    fn search_has_another_result() {
        let query = "Rust";

        assert_eq!(vec!["Rust:"], search(query, CONTENTS));
    }

    #[test]
    fn search_has_two_results() {
        let query = "u";

        assert_eq!(
            vec!["Rust:", "safe, fast, productive."],
            search(query, CONTENTS)
        );
    }

    #[test]
    fn search_case_insensitive_has_one_result() {
        let query = "rust";

        assert_eq!(vec!["Rust:"], search_case_insensitive(query, CONTENTS));
    }

    #[test]
    fn search_case_sensitive_has_no_result() {
        let query = "rust";

        assert_eq!(Vec::<&str>::new(), search(query, CONTENTS));
    }
}
