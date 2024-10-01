use rand::seq::IteratorRandom; // 0.7.3
use std::fs::File;
use csv::ReaderBuilder;
use std::env;

fn main() {
    let exec_path = env::current_exe().expect("Failed to get executable path");
    let quotes_path = exec_path.parent().unwrap().join("quotes.csv");

    let _quotes = match std::fs::read_to_string(&quotes_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading {}: {}", quotes_path.display(), e);
            eprintln!("Make sure quotes.txt exists in the same directory as the executable.");
            return;
        }
    };

    println!("{}", get_quote("quotes.csv"));
}

fn get_quote(file_path: &str) -> String {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");
    let quotes_path = exe_dir.join("quotes.csv");

    let file_name = if file_path.is_empty() {
        quotes_path.to_str().unwrap().to_string()
    } else {
        file_path.to_string()
    };
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
