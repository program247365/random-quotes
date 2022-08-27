# Random Quotes

> Rust app that displays a new quote every time you open a terminal

![Random Quotes in Action!](/screenshot.gif?raw=true "Randome Quotes in Action!")

## Setup

1. `cargo build --release`
2. Move `random-quotes` binary to location on your path (e.g. $HOME/bin/random-quotes)
3. Put `quotes.csv` formatted in `quote goes here,author of quote` on each line
4. Put binary in bottom of ~/.zshrc or ~/.bashrc (e.g. $HOME/bin/random-quotes) and save
5. Reload terminal, and voilà, you have a random quote whenever you open the terminal!

## Future Things?

* Fix TODOs in program, once get better at Rust ;)
* Allow you to specify filename on cli
* Make it so you can put quotes in Google Spreadsheet, and will show those
