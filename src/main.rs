use rand::seq::IteratorRandom; // 0.7.3
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    println!("{}", get_quote());
    Ok(())
}

fn get_exec_name() -> String {
    std::env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned()
}

fn get_current_exe_path() -> String {
    let path = std::env::current_exe().unwrap();
    let dir: String = path.display().to_string();
    let exec_name: String = get_exec_name().to_owned();
    dir.to_owned()
        .to_string()
        .replace(&exec_name, "")
        .replace("//", "/random-quotes/")
    // TOOD: find a more dynamic way to do this ^
}

fn get_quote() -> String {
    let file_name = get_current_exe_path() + "quotes.csv";
    let f = File::open(&file_name)
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", &file_name, e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));
    let line = lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines");

    // TODO: Figure out how to replace only last comma in line
    // not every comma
    let parsed_string: String = line
        .chars()
        .map(|x| match x {
            ',' => '–',
            _ => x,
        })
        .collect();

    parsed_string
}
