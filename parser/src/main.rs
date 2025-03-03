use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde_json::Value;
use std::error::Error;

mod consts;
mod utils;
use utils::{read_excel, write_excel, clean_phone_number, uppercase_state, truncate_zip_code, populate_zip_code, populate_state_from_zip, populate_state_zip_from_area_code};
use crate::consts::{STATE_ZIP};

fn main() -> Result<(), Box<dyn Error>> {
    // Read the config file
    let config_path = Path::new("config.json");
    let mut file = File::open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse JSON
    let config: Value = serde_json::from_str(&contents)?;
    let input_file = config["input_file"].as_str().ok_or("Invalid config file")?;
    let output_file = config["output_file"].as_str().ok_or("Invalid config file")?;

    let mut rows = read_excel(input_file, None)?;

    let headers = rows[0].clone();
    let mut row_number = 0;

    for row in &mut rows[1..] {
        row_number += 1;

        clean_phone_number(row);
        uppercase_state(row);
        truncate_zip_code(row);

        if row[7].is_empty() && !row[6].is_empty() {
            populate_zip_code(row);
        }

        if row[6].is_empty() || !STATE_ZIP.contains_key(&row[6]) {
            populate_state_from_zip(row);
        }

        if (row[6].len() != 2 || row[6].is_empty()) && row[7].is_empty() {
            populate_state_zip_from_area_code(row);
        }
    }

    let col_max = rows.iter().map(|row| row.len()).max().unwrap_or(0);
    write_excel(output_file, rows, col_max)?;
    println!("{} rows were processed.", row_number);
    Ok(())
}


