use csv::{ReaderBuilder, WriterBuilder};
use chrono::Local;
use std::env;
use std::error::Error;
use std::io::{self};

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments for optional header flag
    let args: Vec<String> = env::args().collect();
    let has_header = args.contains(&"--header".to_string());

    // Prompt the user for the input file name
    let mut input_file = String::new();
    println!("Please enter the input file name WITH the file extension. File must be in the SAME directory as this script.");
    io::stdin().read_line(&mut input_file)?;
    let input_file = input_file.trim();

    // Generate the output file name with the current date
    let date = Local::now().format("%m_%d_%Y").to_string();
    let output_file = format!("extractedNumbers_{}.csv", date);

    // Open the input CSV file
    let mut rdr = ReaderBuilder::new()
        .has_headers(has_header)
        .from_path(input_file)?;

    // Open the output CSV file
    let mut wtr = WriterBuilder::new().from_path(&output_file)?;
    wtr.write_record(&["phone_numbers"])?;

    // Identify the column with valid phone numbers
    let mut phone_number_index: Option<usize> = None;

    if let Some(headers) = rdr.headers().ok() {
        for (i, header) in headers.iter().enumerate() {
            if header.chars().all(|c| c.is_numeric()) && header.len() == 10 {
                phone_number_index = Some(i);
                break;
            }
        }
    }

    if phone_number_index.is_none() {
        println!("No valid phone number column found in the headers.");
        return Ok(());
    }

    // Process the rows to extract valid phone numbers
    for result in rdr.records() {
        let record = result?;
        if let Some(index) = phone_number_index {
            if let Some(phone_number) = record.get(index) {
                if is_valid_phone_number(phone_number) {
                    wtr.write_record(&[phone_number])?;
                }
            }
        }
    }

    println!("Phone numbers have been extracted to '{}'.", output_file);

    Ok(())
}

fn is_valid_phone_number(phone_number: &str) -> bool {
    // Check if the phone number is valid (10 digits or starts with '1' followed by 10 digits)
    phone_number.len() == 10 && phone_number.chars().all(|c| c.is_numeric())
        || (phone_number.len() == 11
            && phone_number.starts_with('1')
            && phone_number.chars().skip(1).all(|c| c.is_numeric()))
}