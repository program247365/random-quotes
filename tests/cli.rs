use std::path::PathBuf;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_random-quotes");

fn run(args: &[&str], cwd: &str) -> (String, String, bool) {
    let out = Command::new(BIN)
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("failed to run random-quotes");
    (
        String::from_utf8(out.stdout).unwrap(),
        String::from_utf8(out.stderr).unwrap(),
        out.status.success(),
    )
}

#[test]
fn prints_a_quote_from_the_path_given_on_the_command_line() {
    let csv = std::env::temp_dir().join("random-quotes-cli.csv");
    std::fs::write(
        &csv,
        "quote,author\n\"Slow is smooth, smooth is fast.\",Navy Seals\n",
    )
    .unwrap();

    let (stdout, stderr, ok) = run(&[csv.to_str().unwrap()], "/");

    assert!(ok, "exited non-zero: {stderr}");
    assert_eq!(stdout, "\"Slow is smooth, smooth is fast.\" - Navy Seals\n");
}

#[test]
fn finds_quotes_csv_beside_the_binary_when_run_from_elsewhere() {
    let beside = PathBuf::from(BIN).with_file_name("quotes.csv");
    std::fs::copy(concat!(env!("CARGO_MANIFEST_DIR"), "/quotes.csv"), &beside).unwrap();

    let (stdout, stderr, ok) = run(&[], "/");

    assert!(ok, "exited non-zero: {stderr}");
    assert!(stdout.starts_with('"'), "unexpected output: {stdout:?}");
    assert!(stdout.contains(" - "), "missing author: {stdout:?}");
}

#[test]
fn a_missing_quotes_file_exits_nonzero_with_a_readable_message() {
    let (stdout, stderr, ok) = run(&["/nonexistent/quotes.csv"], "/");

    assert!(!ok, "should have failed");
    assert!(stdout.is_empty(), "wrote to stdout: {stdout:?}");
    assert!(stderr.contains("/nonexistent/quotes.csv"), "{stderr}");
    assert!(
        !stderr.contains("panicked"),
        "panicked instead of failing: {stderr}"
    );
}

#[test]
fn a_quote_full_of_punctuation_survives_the_round_trip_to_the_terminal() {
    let quote = "He said \"\"hello, world\"\" \u{2014} then left; nobody laughed. Jos\u{e9}?!";
    let csv = std::env::temp_dir().join("random-quotes-punctuation.csv");
    std::fs::write(
        &csv,
        format!("quote,author\n\"{quote}\",\"O'Brien, Jr.\"\n"),
    )
    .unwrap();

    let (stdout, stderr, ok) = run(&[csv.to_str().unwrap()], "/");

    assert!(ok, "exited non-zero: {stderr}");
    assert_eq!(
        stdout,
        "\"He said 'hello, world' \u{2014} then left; nobody laughed. Jos\u{e9}?!\" - O'Brien, Jr.\n"
    );
    assert_eq!(
        stdout.matches('"').count(),
        2,
        "delimiters are ambiguous: {stdout}"
    );
}

#[test]
fn help_goes_to_stdout_and_exits_zero() {
    for flag in ["--help", "-h", "help"] {
        let (stdout, stderr, ok) = run(&[flag], "/");
        assert!(ok, "{flag} exited non-zero: {stderr}");
        assert!(stderr.is_empty(), "{flag} wrote to stderr: {stderr}");
        for section in ["USAGE", "ARGUMENTS", "FILE FORMAT", "EXIT STATUS"] {
            assert!(stdout.contains(section), "{flag} help is missing {section}");
        }
    }
}

#[test]
fn an_unknown_option_prints_help_on_stderr_and_exits_two() {
    let out = Command::new(BIN)
        .arg("--bogus")
        .current_dir("/")
        .output()
        .expect("failed to run random-quotes");
    let stdout = String::from_utf8(out.stdout).unwrap();
    let stderr = String::from_utf8(out.stderr).unwrap();

    assert_eq!(out.status.code(), Some(2), "wrong exit code");
    assert!(stdout.is_empty(), "help leaked to stdout: {stdout}");
    assert!(
        stderr.contains("--bogus"),
        "error does not name the option: {stderr}"
    );
    assert!(stderr.contains("USAGE"), "no help on stderr: {stderr}");
}

#[test]
fn version_reports_the_crate_version() {
    let (stdout, _, ok) = run(&["--version"], "/");
    assert!(ok);
    assert_eq!(
        stdout.trim(),
        format!("random-quotes {}", env!("CARGO_PKG_VERSION"))
    );
}

/// ~/.zshrc.local and the Neovim snacks dashboard both invoke the binary with
/// no arguments and expect a quote. Adding --help must never change that.
#[test]
fn bare_invocation_prints_a_quote_and_never_the_help_text() {
    let beside = PathBuf::from(BIN).with_file_name("quotes.csv");
    std::fs::copy(concat!(env!("CARGO_MANIFEST_DIR"), "/quotes.csv"), &beside).unwrap();

    let (stdout, stderr, ok) = run(&[], "/");

    assert!(ok, "exited non-zero: {stderr}");
    assert!(stdout.starts_with('"'), "not a quote: {stdout:?}");
    assert!(!stdout.contains("USAGE"), "printed help instead of a quote");
    assert_eq!(stdout.lines().count(), 1, "expected one line: {stdout:?}");
}
