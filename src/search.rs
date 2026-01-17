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
    use rstest::rstest;

    const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.";

    #[rstest]
    #[case::one_result_duct("duct", vec!["safe, fast, productive."])]
    #[case::one_result_three("three", vec!["Pick three."])]
    #[case::two_results_u("u", vec!["Rust:", "safe, fast, productive."])]
    #[case::no_result_rust("rust", vec![])]
    fn search_is_correct(#[case] query: &str, #[case] result: Vec<&str>) {
        assert_eq!(result, search(query, CONTENTS));
    }

    #[rstest]
    #[case::one_result_rust("rust", vec!["Rust:"])]
    #[case::one_result_rust("pick", vec!["Pick three."])]
    fn search_case_insensitive_is_correct(#[case] query: &str, #[case] result: Vec<&str>) {
        assert_eq!(result, search_case_insensitive(query, CONTENTS));
    }
}
