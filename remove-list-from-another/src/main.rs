use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, Write};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Config {
    input_file_a: String,
    input_file_dnc: String,
    output_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let json_file = File::open("input.json")?;
    let config: Config = serde_json::from_reader(&json_file)?;

    let hashmap_a = process_file(&config.input_file_a)?;
    let hashmap_dnc = process_file(&config.input_file_dnc)?;

    let mut output_file = File::create(&config.output_file)?;

    for (number, _) in &hashmap_a {
        if !hashmap_dnc.contains_key(number) {
            writeln!(output_file, "{}", number)?;
        }
    }
    println!("Processing complete. Results written to {}", config.output_file);
    Ok(())
}

fn process_file(file: &str) -> Result<HashMap<String, usize>, Box<dyn Error>> {
    let mut count = HashMap::new();

    let file = File::open(file)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let number = line.trim().to_string();
        *count.entry(number).or_insert(0) += 1;
    }
    Ok(count)
}
