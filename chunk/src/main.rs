// main.rs
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use chrono::Local;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    input_file: String,
    numbers_per_file: usize,
}

fn main() -> io::Result<()> {
    // Parse config.toml
    let config: Config = parse_config("config.toml")
        .expect("Failed to read or parse config.toml");

    let input_file = &config.input_file;
    let numbers_per_file = config.numbers_per_file;
    let output_dir = format!("output_{}", Local::now().format("%m_%d_%Y"));

    // Create output directory if it doesn't exist
    if !Path::new(&output_dir).exists() {
        fs::create_dir(&output_dir)?;
    }

    let file = File::open(input_file)
        .expect("Could not open input file");
    let reader = BufReader::new(file);

    let mut numbers = Vec::new();
    let mut file_count = 0;
    let mut line_count = 0;

    for line in reader.lines() {
        let line = line?;
        numbers.push(line);
        line_count += 1;

        if line_count == numbers_per_file {
            file_count += 1;
            write_to_file(&output_dir, file_count, &numbers)?;
            numbers.clear(); // Clear the vector
            line_count = 0;
        }
    }

    // Write remaining numbers
    if !numbers.is_empty() {
        file_count += 1;
        write_to_file(&output_dir, file_count, &numbers)?;
    }

    println!("Files created in directory '{}'", output_dir);

    Ok(())
}

fn parse_config(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

fn write_to_file(dir: &str, count: usize, numbers: &[String]) -> io::Result<()> {
    let date = Local::now().format("%m_%d_%Y");
    let file_name = format!("{}/output_{}_{}.txt", dir, date, count);
    let mut file = File::create(&file_name)?;

    for number in numbers {
        writeln!(file, "{}", number)?;
    }

    println!("Created file: {}", file_name);
    Ok(())
}
