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
