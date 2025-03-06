use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json;
use csv::{ReaderBuilder, WriterBuilder, StringRecord};

#[derive(Serialize, Deserialize)]
struct Config {
    max_line: usize,
    input_file: String,
    output_folder: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the config file
    let config_file = File::open("config.json")?;
    let config: Config = serde_json::from_reader(config_file)?;

    // Create the output folder if it doesn't exist
    fs::create_dir_all(&config.output_folder)?;

    // Open the input CSV file
    let file = File::open(&config.input_file)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Get the header from the input CSV
    let header = reader.headers()?.clone();

    // Initialize variables for chunking
    let mut chunk_number = 1;
    let mut line_count = 0;
    let mut writer = create_writer(&config.output_folder, &config.input_file, chunk_number, &header)?;

    // Iterate over the CSV records
    for result in reader.records() {
        let record = result?;

        // Write the record to the current chunk file
        writer.write_record(&record)?;
        line_count += 1;

        // If the line count reaches the max_line value, start a new chunk
        if line_count == config.max_line {
            writer.flush()?;
            chunk_number += 1;
            line_count = 0;
            writer = create_writer(&config.output_folder, &config.input_file, chunk_number, &header)?;
        }
    }

    // Flush the last chunk file
    writer.flush()?;

    println!(
        "CSV file '{}' split into chunks of {} lines. Output saved to '{}'.",
        config.input_file, config.max_line, config.output_folder
    );

    Ok(())
}

// Helper function to create a new CSV writer for a chunk
fn create_writer(
    output_folder: &str,
    input_file: &str,
    chunk_number: usize,
    header: &StringRecord,
) -> Result<csv::Writer<File>, Box<dyn Error>> {
    // Get the input file name without extension
    let input_file_name = Path::new(input_file)
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid input file name")?;

    // Create the output file path
    let output_file = format!(
        "{}/{}_{}.csv",
        output_folder, input_file_name, chunk_number
    );

    // Create the CSV writer
    let mut writer = WriterBuilder::new().from_path(output_file)?;

    // Write the header to the output file
    writer.write_record(header)?;

    Ok(writer)
}