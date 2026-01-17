use minigrep::GrepArgs;
use minigrep::run;
use rstest::rstest;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn nonexistant_file() {
    let result = run(&GrepArgs::default());

    assert!(result.is_err());
    assert!(result.is_err_and(|err| err.starts_with("No such file or directory")));
}

#[rstest]
#[case::to_empty("to", "empty.txt", false)]
#[case::to_poem("to", "poem.txt", false)]
#[case::to_poem_case_insensitive("to", "poem.txt", true)]
fn empty_file(#[case] query: &str, #[case] file_path: &str, #[case] ignore_case: bool) {
    let args = GrepArgs::new(query, file_path, ignore_case);
    let result = run(&args);

    assert!(result.is_ok());

    let stdout = get_actual_result(&args, true);
    let expected_out = get_expected_result(&args, true);

    assert_eq!(stdout, expected_out, "stdout did not match expected file");

    let stderr = get_actual_result(&args, false);
    let expected_err = get_expected_result(&args, false);

    assert_eq!(stderr, expected_err, "stderr did not match expected file");
}

fn get_expected_result(args: &GrepArgs, stdout_not_stderr: bool) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!(
        "tests/expected/{}/{}",
        args.query(),
        args.file_path(),
    ));
    if args.ignore_case() {
        path.push("insensitive");
    }
    path.push(format!(
        "{}.txt",
        if stdout_not_stderr {
            "stdout"
        } else {
            "stderr"
        }
    ));
    fs::read_to_string(path).expect("couldn't read expected file")
}

fn get_actual_result(args: &GrepArgs, stdout_not_stderr: bool) -> String {
    let mut command = Command::new("target/debug/minigrep");
    command.arg(args.query()).arg(args.file_path());
    if args.ignore_case() {
        command.env("IGNORE_CASE", "");
    }
    let output = command.output().expect("failed to run binary");

    assert!(output.status.success(), "program exited with non-zero");

    String::from_utf8(if stdout_not_stderr {
        output.stdout
    } else {
        output.stderr
    })
    .expect("stdout not utf-8")
}
