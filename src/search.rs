pub(super) fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    filter_lines(query, contents, |line, query| line.contains(query))
}

pub(super) fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    filter_lines(&query.to_lowercase(), contents, |line, query| {
        line.to_lowercase().contains(query)
    })
}

fn filter_lines<'a, F>(query: &str, contents: &'a str, filter: F) -> Vec<&'a str>
where
    F: Fn(&str, &str) -> bool,
{
    contents
        .lines()
        .filter(|line| filter(line, query))
        .collect()
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
    #[case::one_result_productive("productive", vec!["safe, fast, productive."])]
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
