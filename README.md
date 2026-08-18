# Random Quotes

> Rust app that displays a new quote every time you open a terminal

![Random Quotes in Action!](/screenshot.gif?raw=true "Random Quotes in Action!")

## Setup

```zsh
git clone git@github.com:program247365/random-quotes.git
cd random-quotes
make install
```

`make install` builds an optimized binary and installs it, together with
`quotes.csv`, into `$PREFIX` (default `~/.kevin/bin`). It then runs the
installed binary to prove the install works, and warns if `$PREFIX` is not on
your `PATH`.

```zsh
make install PREFIX=~/bin   # install somewhere else
make uninstall              # remove both files from $PREFIX
```

To print a quote whenever you open a terminal, add the binary to your shell
startup file:

```zsh
echo 'random-quotes' >> ~/.zshrc.local
```

### Updating

Re-run `make install`. It overwrites both the binary and `quotes.csv` in place,
so there is nothing else to clean up:

```zsh
git pull && make install
```

## Usage

```zsh
random-quotes              # print a random quote
random-quotes ~/mine.csv   # print one from a specific file
random-quotes --help       # full help (also -h, help)
random-quotes --version    # print the version (also -V)
```

With no `FILE`, it reads `quotes.csv` from the directory the binary lives in,
so it works from any working directory.

| Exit | Meaning |
|------|---------|
| `0`  | a quote was printed |
| `1`  | the quotes file is missing or has no quotes |
| `2`  | bad usage — the error and full help go to stderr |

Because a missing file exits non-zero with a message on stderr rather than
crashing, a broken install never derails your shell startup.

### Who calls this

Running the bare command prints a quote, and that behaviour is load-bearing —
a regression test in `tests/cli.rs` pins it. Known callers:

- `~/.zshrc.local` — prints a quote on every new shell
- `~/.dotfiles/config/lua/plugins/dashboard.lua` — pipes the quote into the
  Neovim `snacks.nvim` dashboard header

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

`make` on its own prints every target it can run:

```zsh
make            # same as make help
make check      # format check + clippy + tests, i.e. everything CI runs
make test       # tests only
make fmt        # format in place
make lint       # clippy, warnings treated as errors
make run ARGS=--help
make release
make clean
```

Targets document themselves: `make help` builds its listing from the `##`
comments in the `Makefile`, and a test fails if any target lacks one.

Tests live in two places — unit tests in `src/main.rs` and CLI tests in
`tests/cli.rs` that drive the compiled binary. `tests/fixtures/legacy-quotes.csv`
is the original 2019 quotes file, kept so the old format stays readable.

## Future Things?

* ~~Fix TODOs in program, once I get better at Rust ;)~~
* ~~Allow specifying the filename via command-line options~~
* Integrate with Google Spreadsheets to fetch quotes
