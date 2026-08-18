# Random Quotes

> Rust app that displays a new quote every time you open a terminal

![Random Quotes in Action!](/screenshot.gif?raw=true "Random Quotes in Action!")

## Setup

1. `cargo build --release`
2. Move the `random-quotes` binary somewhere on your `$PATH` (e.g. `$HOME/bin/random-quotes`)
3. Put a `quotes.csv` next to the binary, or pass a path to one (see Usage)
4. Add the binary call to the bottom of `~/.zshrc` or `~/.bashrc` and save
5. Reload your terminal, and voilà, you have a random quote whenever you open the terminal!

## Usage

```zsh
random-quotes                    # reads quotes.csv sitting next to the binary
random-quotes ~/my-quotes.csv    # reads the file you name
```

Exits `1` with a message on stderr if the file is missing or has no quotes, so a
broken setup never derails your shell startup.

## Quotes file format

RFC 4180 CSV, one quote per row, UTF-8:

```csv
"quote","author"
"Slow is smooth, smooth is fast.","Navy Seals"
"He said ""hello, world"" and left.","Anonymous"
```

- A `quote,author` header row is optional — it is skipped if present.
- Quoting every field is the safe default: commas, semicolons, colons, dashes,
  ellipses, and any Unicode all pass through untouched.
- A literal `"` inside a quote is escaped by doubling it (`""`), per RFC 4180.
  On output it is printed as a single quote, so the outer pair always marks
  where the quote begins and ends:

  ```
  "He said ""hello, world"" and left.","O'Brien, Jr."
    prints as:  "He said 'hello, world' and left." - O'Brien, Jr.
  ```
- Fields are trimmed, blank rows are skipped, and a trailing empty third column
  (from older exports) is ignored.

## Development

```zsh
cargo test          # unit + CLI integration tests
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

## Future Things?

* ~~Fix TODOs in program, once I get better at Rust ;)~~
* ~~Allow specifying the filename via command-line options~~
* Integrate with Google Spreadsheets to fetch quotes
