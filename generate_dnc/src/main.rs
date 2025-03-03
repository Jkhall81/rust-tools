use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    total_file: String,
    clean_file: String,
    fdnc_file: String,
    output_file: String,
}

fn main() -> io::Result<()> {
    // Read configuration file
    let config: Config = toml::from_str(&fs::read_to_string("config.toml")?)
        .expect("Failed to parse configuration file");

    // Read data from files into HashSets
    let total_numbers = read_file_to_set(&config.total_file)?;
    let clean_numbers = read_file_to_set(&config.clean_file)?;
    let fdnc_numbers = read_file_to_set(&config.fdnc_file)?;

    // Filter numbers
    let mut filtered_numbers: HashSet<_> = total_numbers.difference(&clean_numbers).cloned().collect();
    filtered_numbers = filtered_numbers.difference(&fdnc_numbers).cloned().collect();

    // Write filtered numbers to output file
    write_set_to_file(&filtered_numbers, &config.output_file)?;
    println!("Filtered list written to {}", config.output_file);
    Ok(())
}

fn read_file_to_set(file_path: &str) -> io::Result<HashSet<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut set = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        set.insert(line);
    }
    Ok(set)
}

fn write_set_to_file(set: &HashSet<String>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    let mut sorted_values: Vec<_> = set.iter().collect();
    sorted_values.sort();

    for value in sorted_values {
        writeln!(file, "{}", value)?;
    }
    Ok(())
}
