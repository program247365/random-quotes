use csv::ReaderBuilder;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::{env, process};

const HELP: &str = concat!(
    "random-quotes ",
    env!("CARGO_PKG_VERSION"),
    r#"
Print one random quote from a CSV file.

USAGE
  random-quotes [FILE]      Print a random quote
  random-quotes --help      Show this help
  random-quotes --version   Show the version

ARGUMENTS
  FILE   Path to a quotes CSV. Defaults to quotes.csv beside the binary.

FILE FORMAT
  RFC 4180 CSV, UTF-8, one quote per row. Header row optional.

    "quote","author"
    "Slow is smooth, smooth is fast.","Navy Seals"

  Escape a literal " by doubling it (""). It prints as a single quote
  so the outer pair always marks where the quote begins and ends.

EXIT STATUS
  0  quote printed      1  file missing or empty      2  bad usage

SHELL STARTUP
  Add `random-quotes` to ~/.zshrc.local or ~/.bashrc."#
);

#[derive(Debug, PartialEq)]
enum Command {
    Quote(Option<String>),
    Help,
    Version,
    Usage(String),
}

fn parse_args(args: &[String]) -> Command {
    match args {
        [] => Command::Quote(None),
        [one] => match one.as_str() {
            "--help" | "-h" | "help" => Command::Help,
            "--version" | "-V" => Command::Version,
            flag if flag.starts_with('-') => Command::Usage(format!("unknown option '{flag}'")),
            path => Command::Quote(Some(path.to_string())),
        },
        _ => Command::Usage("expected at most one FILE argument".to_string()),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match parse_args(&args) {
        Command::Quote(file) => print_quote(file),
        Command::Help => println!("{HELP}"),
        Command::Version => println!("random-quotes {}", env!("CARGO_PKG_VERSION")),
        Command::Usage(message) => {
            eprintln!("random-quotes: {message}\n\n{HELP}");
            process::exit(2);
        }
    }
}

fn print_quote(file: Option<String>) {
    match pick_quote(&quotes_path(file)) {
        Ok(line) => println!("{line}"),
        Err(message) => {
            eprintln!("random-quotes: {message}");
            process::exit(1);
        }
    }
}

/// CLI argument wins; otherwise `quotes.csv` beside the executable.
fn quotes_path(arg: Option<String>) -> PathBuf {
    match arg {
        Some(path) => PathBuf::from(path),
        None => env::current_exe()
            .expect("cannot locate the executable")
            .with_file_name("quotes.csv"),
    }
}

fn pick_quote(path: &Path) -> Result<String, String> {
    let file = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    parse_quotes(file)
        .choose(&mut rand::thread_rng())
        .map(|(quote, author)| format_quote(quote, author))
        .ok_or_else(|| format!("{}: no quotes found", path.display()))
}

/// Reads `quote,author` rows. Tolerates the legacy format: unquoted rows,
/// a trailing empty third field, and no header line.
fn parse_quotes<R: io::Read>(reader: R) -> Vec<(String, String)> {
    ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(reader)
        .records()
        .flatten()
        .map(|row| {
            let field = |i| row.get(i).unwrap_or("").trim().to_string();
            (field(0), field(1))
        })
        .filter(|(quote, author)| {
            let is_header_row = quote == "quote" && author == "author";
            !quote.is_empty() && !is_header_row
        })
        .collect()
}

/// Nested quotation takes single quotes, so the outer pair always marks
/// where the quote begins and ends.
fn format_quote(quote: &str, author: &str) -> String {
    let quote = quote.replace('"', "'");
    if author.is_empty() {
        format!("\"{quote}\"")
    } else {
        format!("\"{quote}\" - {author}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;

    const LEGACY: &str = include_str!("../tests/fixtures/legacy-quotes.csv");
    const CURRENT: &str = include_str!("../quotes.csv");

    fn parse(s: &str) -> Vec<(String, String)> {
        parse_quotes(s.as_bytes())
    }

    #[test]
    fn keeps_commas_inside_a_quoted_field() {
        let rows = parse("\"Slow is smooth, smooth is fast.\",Navy Seals\n");
        assert_eq!(
            rows,
            vec![(
                "Slow is smooth, smooth is fast.".to_string(),
                "Navy Seals".to_string()
            )]
        );
    }

    #[test]
    fn reads_unquoted_legacy_rows() {
        let rows = parse("How you do anything is how you will do everything.,John Wooden\n");
        assert_eq!(rows[0].1, "John Wooden");
    }

    #[test]
    fn ignores_the_legacy_trailing_empty_field() {
        let rows = parse("\"Whenever you find yourself here, pause.\",Mark Twain,\n");
        assert_eq!(
            rows,
            vec![(
                "Whenever you find yourself here, pause.".to_string(),
                "Mark Twain".to_string()
            )]
        );
    }

    #[test]
    fn skips_the_header_row() {
        let rows = parse("quote,author\n\"A quote, here.\",Someone\n");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].1, "Someone");
    }

    #[test]
    fn keeps_escaped_double_quotes_inside_a_quote() {
        let rows = parse("\"He said \"\"hello, world\"\" loudly.\",Someone\n");
        assert_eq!(rows[0].0, "He said \"hello, world\" loudly.");
    }

    #[test]
    fn keeps_a_newline_inside_a_quoted_field() {
        let rows = parse("\"Line one,\nline two.\",Someone\n");
        assert_eq!(rows[0].0, "Line one,\nline two.");
    }

    #[test]
    fn keeps_unicode_punctuation_intact() {
        let rows = parse("\"Don\u{2019}t stop \u{2014} ever, jamás.\",José\n");
        assert_eq!(rows[0].0, "Don\u{2019}t stop \u{2014} ever, jamás.");
        assert_eq!(rows[0].1, "José");
    }

    #[test]
    fn skips_blank_rows() {
        let rows = parse("\"Real quote.\",Someone\n\n,\n");
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn reads_every_legacy_quote_without_field_bleed() {
        let rows = parse(LEGACY);
        assert_eq!(rows.len(), 79);
        assert!(rows.iter().all(|(q, a)| !q.is_empty() && !a.is_empty()));
        assert!(rows
            .iter()
            .any(|(q, a)| q == "Slow is smooth, smooth is fast." && a == "Navy Seals"));
        assert!(rows.iter().any(|(_, a)| a == "Johann Wolfgang von Goethe"));
    }

    #[test]
    fn reads_every_current_quote_without_field_bleed() {
        let rows = parse(CURRENT);
        assert_eq!(rows.len(), 60);
        assert!(rows.iter().all(|(q, a)| !q.is_empty() && !a.is_empty()));
        assert!(rows.iter().any(|(q, a)| q
            == "Everything can be taken from a man but one thing, the last of the human freedoms -- to choose one's attitude in any given set of circumstances, to choose one's own way."
            && a == "Viktor Frankl"));
    }

    /// Removed after a source audit: each of these is either documented as
    /// false or has no primary source in the named person's work.
    #[test]
    fn quotes_that_failed_source_verification_stay_out() {
        const UNSOURCED: &[&str] = &[
            "Miyamoto Musashi",
            "Archilochus",
            "Aristotle",
            "Abraham Lincoln",
            "Sun Tzu",
            "Carl Gustav Jung",
            "Warren Buffett",
            "Mae West",
            "Giacomo Casanova",
            "Salvador Dal\u{ed}",
            "Winston Churchill",
            "John Wooden",
            "James Watson",
        ];
        let rows = parse(CURRENT);
        for author in UNSOURCED {
            assert!(
                !rows.iter().any(|(_, a)| a == author),
                "unsourced quote is back in the file: {author}"
            );
        }
        // Covey keeps his own sourced quote but not the 19th-century proverb.
        assert!(!rows.iter().any(|(q, _)| q.starts_with("Sow a thought")));
        // Epictetus keeps Discourses/Enchiridion but not Lebell's paraphrase.
        assert!(!rows.iter().any(|(q, _)| q.starts_with("Keep company only")));
        // Einstein keeps the sourced quotes but not the birthday one.
        assert!(!rows.iter().any(|(q, _)| q.starts_with("Birthdays cause")));
    }

    #[test]
    fn every_current_quote_survives_a_csv_round_trip() {
        let rows = parse(CURRENT);
        let mut buf = Vec::new();
        {
            let mut w = csv::Writer::from_writer(&mut buf);
            w.write_record(["quote", "author"]).unwrap();
            for (q, a) in &rows {
                w.write_record([q, a]).unwrap();
            }
            w.flush().unwrap();
        }
        assert_eq!(parse(std::str::from_utf8(&buf).unwrap()), rows);
    }

    #[test]
    fn formats_a_quote_with_its_author() {
        assert_eq!(
            format_quote("Slow is smooth, smooth is fast.", "Navy Seals"),
            "\"Slow is smooth, smooth is fast.\" - Navy Seals"
        );
    }

    #[test]
    fn formats_a_quote_with_no_author() {
        assert_eq!(
            format_quote("Anonymous wisdom.", ""),
            "\"Anonymous wisdom.\""
        );
    }

    #[test]
    fn formatted_output_is_a_single_terminal_line() {
        for (q, a) in parse(CURRENT) {
            let line = format_quote(&q, &a);
            assert!(!line.contains('\n'), "multi-line output for {a}");
            assert!(!line.contains('\r'), "carriage return in output for {a}");
            assert!(
                !line.contains('\u{1b}'),
                "escape sequence in output for {a}"
            );
        }
    }

    #[test]
    fn a_cli_argument_overrides_the_default_path() {
        assert_eq!(
            quotes_path(Some("/tmp/mine.csv".to_string())),
            PathBuf::from("/tmp/mine.csv")
        );
    }

    #[test]
    fn without_an_argument_the_path_sits_beside_the_executable() {
        let path = quotes_path(None);
        assert_eq!(path.file_name().unwrap(), "quotes.csv");
        assert_eq!(
            path.parent().unwrap(),
            std::env::current_exe().unwrap().parent().unwrap()
        );
    }

    #[test]
    fn a_random_pick_is_always_one_of_the_file_s_quotes() {
        let rows = parse(CURRENT);
        for _ in 0..200 {
            let pick = rows.choose(&mut rand::thread_rng()).unwrap();
            assert!(rows.contains(pick));
        }
    }

    fn write_temp(name: &str, contents: &str) -> PathBuf {
        let path = std::env::temp_dir().join(name);
        std::fs::write(&path, contents).unwrap();
        path
    }

    #[test]
    fn a_missing_file_is_an_error_not_a_panic() {
        let err = pick_quote(Path::new("/nonexistent/quotes.csv")).unwrap_err();
        assert!(err.contains("/nonexistent/quotes.csv"), "{err}");
    }

    #[test]
    fn a_file_with_no_quotes_is_an_error_not_a_panic() {
        let path = write_temp("random-quotes-empty.csv", "quote,author\n");
        assert!(pick_quote(&path).unwrap_err().contains("no quotes found"));
    }

    #[test]
    fn a_quote_is_read_from_a_file_on_disk() {
        let path = write_temp(
            "random-quotes-one.csv",
            "quote,author\n\"Slow is smooth, smooth is fast.\",Navy Seals\n",
        );
        assert_eq!(
            pick_quote(&path).unwrap(),
            "\"Slow is smooth, smooth is fast.\" - Navy Seals"
        );
    }

    #[test]
    fn the_quotes_file_is_canonically_formatted() {
        let mut buf = Vec::new();
        {
            let mut writer = csv::WriterBuilder::new()
                .quote_style(csv::QuoteStyle::Always)
                .from_writer(&mut buf);
            writer.write_record(["quote", "author"]).unwrap();
            for (quote, author) in parse(CURRENT) {
                writer.write_record([&quote, &author]).unwrap();
            }
            writer.flush().unwrap();
        }
        assert_eq!(
            CURRENT,
            std::str::from_utf8(&buf).unwrap(),
            "quotes.csv is not canonical: expected LF endings, a trailing newline, \
             a quoted header, and every field quoted"
        );
    }

    /// Wordings and attributions corrected against primary sources. Each was
    /// wrong in an earlier revision; this keeps a careless edit from undoing them.
    #[test]
    fn source_verified_quotes_stay_corrected() {
        const VERIFIED: &[(&str, &str)] = &[
            // comma lost somewhere in the "Update quotes.csv" commits
            ("If [more] information was the answer, then we'd all be billionaires with perfect abs.", "Derek Sivers"),
            // Let Us Have Faith (1940); "a" is not optional and "at all" is a later accretion
            ("Life is either a daring adventure or nothing.", "Helen Keller"),
            // Cargo Cult Science, Caltech commencement, 1974
            ("The first principle is that you must not fool yourself -- and you are the easiest person to fool.", "Richard Feynman"),
            // Oglethorpe University commencement, 22 May 1932
            ("But above all, try something.", "Franklin D. Roosevelt"),
            // to the press aboard the Belgenland, December 1930
            ("I never think of the future. It comes soon enough.", "Albert Einstein"),
            ("If I'd observed all the rules I'd never have got anywhere.", "Marilyn Monroe"),
            // "A Cult of Ignorance", Newsweek, 21 January 1980
            ("The strain of anti-intellectualism has been a constant thread winding its way through our political and cultural life, nurtured by the false notion that democracy means that 'my ignorance is just as good as your knowledge'.", "Isaac Asimov"),
            // Brandolini's law, coined January 2013 -- routinely misattributed to Paul Kedrosky
            ("The amount of energy necessary to refute bullshit is an order of magnitude bigger than to produce it.", "Alberto Brandolini"),
            // Chicago Tribune column, June 1997 -- Schmich has publicly denied Roosevelt said it
            ("Do one thing every day that scares you.", "Mary Schmich"),
            // Peak Performers (1987) -- not Gandhi
            ("Action expresses priorities.", "Charles Garfield"),
            // Business @ the Speed of Thought (1999) -- what Gates actually wrote
            ("We always overestimate the change that will occur in the next two years and underestimate the change that will occur in the next ten.", "Bill Gates"),
        ];
        let rows = parse(CURRENT);
        for (quote, author) in VERIFIED {
            assert!(
                rows.iter().any(|(q, a)| q == quote && a == author),
                "missing or altered: {author} -- {quote}"
            );
        }
    }

    #[test]
    fn author_names_are_spelled_correctly() {
        let rows = parse(CURRENT);
        assert!(rows.iter().any(|(_, a)| a == "David McCullough"));
        assert!(!rows.iter().any(|(_, a)| a == "David McCoullough"));
    }

    #[test]
    fn nested_double_quotes_become_single_quotes() {
        assert_eq!(
            format_quote("He said \"hello, world\" and left.", "Someone"),
            "\"He said 'hello, world' and left.\" - Someone"
        );
    }

    #[test]
    fn a_quote_that_is_entirely_a_quotation_still_reads_unambiguously() {
        assert_eq!(format_quote("\"Why?\"", "Someone"), "\"'Why?'\" - Someone");
    }

    #[test]
    fn apostrophes_are_left_alone() {
        assert_eq!(
            format_quote("It's Kevin's, isn't it?", "Someone"),
            "\"It's Kevin's, isn't it?\" - Someone"
        );
    }

    #[test]
    fn formatted_output_carries_exactly_one_pair_of_double_quotes() {
        for (quote, author) in parse(CURRENT) {
            let line = format_quote(&quote, &author);
            assert_eq!(
                line.matches('"').count(),
                2,
                "ambiguous delimiters in output for {author}"
            );
        }
        assert_eq!(
            format_quote("A \"nested\" one \"here\" too.", "X")
                .matches('"')
                .count(),
            2
        );
    }

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn no_arguments_asks_for_a_quote_from_the_default_file() {
        assert_eq!(parse_args(&args(&[])), Command::Quote(None));
    }

    #[test]
    fn a_lone_path_asks_for_a_quote_from_that_file() {
        assert_eq!(
            parse_args(&args(&["/tmp/mine.csv"])),
            Command::Quote(Some("/tmp/mine.csv".to_string()))
        );
    }

    #[test]
    fn every_spelling_of_help_asks_for_help() {
        for flag in ["--help", "-h", "help"] {
            assert_eq!(parse_args(&args(&[flag])), Command::Help, "{flag}");
        }
    }

    #[test]
    fn every_spelling_of_version_asks_for_the_version() {
        for flag in ["--version", "-V"] {
            assert_eq!(parse_args(&args(&[flag])), Command::Version, "{flag}");
        }
    }

    #[test]
    fn an_unknown_option_is_a_usage_error_naming_the_option() {
        let Command::Usage(message) = parse_args(&args(&["--bogus"])) else {
            panic!("expected a usage error");
        };
        assert!(message.contains("--bogus"), "{message}");
    }

    #[test]
    fn extra_arguments_are_a_usage_error() {
        assert!(matches!(
            parse_args(&args(&["a.csv", "b.csv"])),
            Command::Usage(_)
        ));
    }

    #[test]
    fn a_filename_that_looks_like_a_flag_is_still_rejected() {
        assert!(matches!(parse_args(&args(&["-x"])), Command::Usage(_)));
    }

    #[test]
    fn the_help_text_documents_what_an_agent_needs() {
        for section in ["USAGE", "ARGUMENTS", "FILE FORMAT", "EXIT STATUS"] {
            assert!(HELP.contains(section), "help is missing {section}");
        }
        assert!(HELP.contains(env!("CARGO_PKG_VERSION")));
    }

    /// `make help` builds its listing from the `## ` comments, so a target
    /// without one is invisible to anyone -- or anything -- reading the output.
    #[test]
    fn every_makefile_target_documents_itself() {
        const MAKEFILE: &str = include_str!("../Makefile");
        let phony = MAKEFILE
            .lines()
            .find_map(|line| line.strip_prefix(".PHONY:"))
            .expect("Makefile has no .PHONY line");
        let targets: Vec<&str> = phony.split_whitespace().collect();
        assert!(targets.len() > 5, "suspiciously few targets: {targets:?}");

        for target in targets {
            let rule = MAKEFILE
                .lines()
                .find(|line| line.starts_with(&format!("{target}:")))
                .unwrap_or_else(|| panic!("`{target}` is in .PHONY but has no rule"));
            assert!(
                rule.contains("## "),
                "`{target}` has no `## ` description, so it is missing from make help"
            );
        }
    }
}
