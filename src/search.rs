pub(super) fn print_matching_lines<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    filter_lines_and_print(content, |line| line.contains(query))
}

pub(super) fn print_matching_lines_case_insensitive<'a>(
    query: &str,
    content: &'a str,
) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();
    filter_lines_and_print(content, |line| {
        line.to_lowercase().contains(&lowercase_query)
    })
}

fn filter_lines_and_print<F>(content: &str, f: F) -> Vec<&str>
where
    F: Fn(&&str) -> bool,
{
    content
        .lines()
        .filter(f)
        .inspect(|elem| println!("{elem}"))
        .collect()
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
    fn print_matching_lines_is_correct(#[case] query: &str, #[case] result: Vec<&str>) {
        assert_eq!(result, print_matching_lines(query, CONTENT));
    }

    #[rstest]
    #[case::one_result_rust("rust", vec!["Rust:"])]
    #[case::one_result_rust("pick", vec!["Pick three."])]
    fn print_matching_lines_case_insensitive_is_correct(
        #[case] query: &str,
        #[case] result: Vec<&str>,
    ) {
        assert_eq!(
            result,
            print_matching_lines_case_insensitive(query, CONTENT)
        );
    }
}
