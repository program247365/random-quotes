use rand::seq::IteratorRandom; // 0.7.3
use std::fs::File;
use csv::ReaderBuilder;

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
    let file = File::open(&file_name)
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", &file_name, e));

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)  // Allow flexible number of fields
        .from_reader(file);

    let records: Vec<csv::StringRecord> = rdr.records().filter_map(Result::ok).collect();

    if records.is_empty() {
        return "No quotes available.".to_string();
    }

    let record = records.iter()
        .choose(&mut rand::thread_rng())
        .expect("Failed to choose a random quote");

    let quote = record.get(0).unwrap_or("").trim();
    let author = record.get(1).unwrap_or("").trim();

    format!("\"{}\" - {}", quote, author)
}
