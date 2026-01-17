use minigrep::GrepArgs;
use rstest::rstest;

#[rstest]
#[case::no_arguments(&[])]
#[case::one_argument(&[String::new()])]
#[case::two_arguments(&[String::new(), String::new()])]
fn build_doesnt_panic_when_not_given_enough_arguments(#[case] args: &[String]) {
    let result = GrepArgs::build(args);

    assert!(result.is_err());
    assert!(result.is_err_and(|err| err.starts_with("Expected at least three arguments")));
}

#[rstest]
#[case::three_empty_arguments(String::new(), String::new())]
#[case::three_arguments("rust", "file.txt")]
fn build_success_when_given_enough_arguments(#[case] query: String, #[case] file_path: String) {
    let result = GrepArgs::build(&[String::new(), query.clone(), file_path.clone()]);

    assert!(result.is_ok());

    let result = result.unwrap();
    assert_eq!(result.file_path(), file_path);
    assert_eq!(result.query(), query);
    assert!(!result.ignore_case());
}
