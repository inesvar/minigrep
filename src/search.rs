pub(super) fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    filter_lines(content, |line| line.contains(query))
}

pub(super) fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    filter_lines(content, |line| {
        line.to_lowercase().contains(&query.to_lowercase())
    })
}

fn filter_lines<F>(content: &str, f: F) -> Vec<&str>
where
    F: Fn(&&str) -> bool,
{
    content.lines().filter(f).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const CONTENT: &str = "\
Rust:
safe, fast, productive.
Pick three.";

    #[rstest]
    #[case::one_result_productive("productive", vec!["safe, fast, productive."])]
    #[case::one_result_three("three", vec!["Pick three."])]
    #[case::two_results_u("u", vec!["Rust:", "safe, fast, productive."])]
    #[case::no_result_rust("rust", vec![])]
    fn search_is_correct(#[case] query: &str, #[case] result: Vec<&str>) {
        assert_eq!(result, search(query, CONTENT));
    }

    #[rstest]
    #[case::one_result_rust("rust", vec!["Rust:"])]
    #[case::one_result_rust("pick", vec!["Pick three."])]
    fn search_case_insensitive_is_correct(#[case] query: &str, #[case] result: Vec<&str>) {
        assert_eq!(result, search_case_insensitive(query, CONTENT));
    }
}
