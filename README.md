# Random Quotes

> Rust app that displays a new quote every time you open a terminal

![Random Quotes in Action!](/screenshot.gif?raw=true "Random Quotes in Action!")

## Setup

1. `cargo build --release`
2. Move `random-quotes` binary to a location on your path (e.g. $HOME/bin/random-quotes)
3. Create a `quotes.csv` file formatted with `quote goes here,author of quote` on each line
4. Place the `quotes.csv` file next to the `random-quotes` binary, or use a full path (see Usage below)
5. Add the binary call to the bottom of ~/.zshrc or ~/.bashrc (e.g. $HOME/bin/random-quotes) and save
6. Reload your terminal, and voilà, you have a random quote whenever you open the terminal!

## Usage

You can run the program in two ways:

1. Without arguments: It will look for `quotes.csv` in the same directory as the executable

   ```zsh
   random-quotes
   ```

2. With a full path to the quotes file:

   ```zsh
   random-quotes /Users/kevin/.kevin/code/random-quotes/quotes.csv
   ```

## Future Things?

* Fix TODOs in program, once I get better at Rust ;)
* Allow specifying the filename via command-line options
* Integrate with Google Spreadsheets to fetch quotes
