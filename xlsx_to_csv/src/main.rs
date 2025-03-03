use calamine::{open_workbook, Reader, Xlsx, Data};
use csv::Writer;
use std::error::Error;
use std::io::{self, Write};
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    println!("Current directory: {:?}", current_dir);

    let mut input_path = String::new();
    let mut output_path = String::new();

    print!("Enter the path to the XLSX file: ");
    io::stdout().flush()?; 
    io::stdin().read_line(&mut input_path)?;
    let input_path = input_path.trim(); 

    print!("Enter the desired path for the output CSV file: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut output_path)?;
    let output_path = output_path.trim();

    let mut workbook: Xlsx<_> = open_workbook(input_path).expect("Cannot open file");

    println!("Available sheets:");
    for sheet_name in workbook.sheet_names() {
        println!("- {}", sheet_name);
    }

    if let Some(first_sheet_name) = workbook.sheet_names().get(0) {
        println!("Trying to read the first sheet: {}", first_sheet_name);
        if let Ok(range) = workbook.worksheet_range(first_sheet_name) {
            let mut wtr = Writer::from_path(output_path)?;

            for row in range.rows() {
                let row_data: Vec<String> = row.iter().map(|cell| match cell {
                    Data::Empty => String::new(),
                    Data::String(s) => s.clone(),
                    Data::Float(f) => f.to_string(),
                    Data::Int(i) => i.to_string(),
                    Data::Bool(b) => b.to_string(),
                    Data::Error(e) => e.to_string(),
                    Data::DateTime(_) => "DateTime".to_string(),
                    Data::DateTimeIso(_) => "DateTimeIso".to_string(),
                    Data::DurationIso(_) => "DurationIso".to_string(),
                }).collect();
                wtr.write_record(row_data)?;
            }
            wtr.flush()?;
            println!("Data written to {}", output_path);
        } else {
            eprintln!("Could not read the sheet: {}", first_sheet_name);
        }
    } else {
        eprintln!("No sheets found in the workbook.");
    }
    Ok(())
}
