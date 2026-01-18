pub(super) fn matching_lines<'a>(
    query: &str,
    content: &'a str,
) -> impl Iterator<Item = &'a str> {
    filter_lines(content, move |line| line.contains(query))
}

pub(super) fn matching_lines_case_insensitive<'a>(
    query: &str,
    content: &'a str,
) -> impl Iterator<Item = &'a str> {
    let lowercase_query = query.to_lowercase();
    filter_lines(content, move |line| {
        line.to_lowercase().contains(&lowercase_query)
    })
}

fn filter_lines<F>(content: &str, f: F) -> impl Iterator<Item = &str>
where
    F: Fn(&&str) -> bool,
{
    content.lines().filter(f)
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
    fn matching_lines_is_correct(#[case] query: &str, #[case] result: Vec<&str>) {
        assert_eq!(
            result,
            matching_lines(query, CONTENT).collect::<Vec<_>>()
        );
    }

    #[rstest]
    #[case::one_result_rust("rust", vec!["Rust:"])]
    #[case::one_result_rust("pick", vec!["Pick three."])]
    fn matching_lines_case_insensitive_is_correct(
        #[case] query: &str,
        #[case] result: Vec<&str>,
    ) {
        assert_eq!(
            result,
            matching_lines_case_insensitive(query, CONTENT).collect::<Vec<_>>()
        );
    }
}
