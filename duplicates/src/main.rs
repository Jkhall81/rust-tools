use csv::ReaderBuilder;
use csv::WriterBuilder;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

fn process_file(file: &str, count: &mut HashMap<String, usize>) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(file)?;
    
    for result in rdr.records() {
        let record = result?;
        for value in record.iter() {
            *count.entry(value.to_string()).or_insert(0) += 1;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file1 = String::new();
    let mut input_file2 = String::new();
    let output_file = "output.csv";

    let mut stdout = io::stdout();
    print!("Enter the name of the first input file: ");
    stdout.flush()?;
    io::stdin().read_line(&mut input_file1)?;
    input_file1 = input_file1.trim().to_string();

    print!("Enter the name of the second input file: ");
    stdout.flush()?;
    io::stdin().read_line(&mut input_file2)?;
    input_file2 = input_file2.trim().to_string();

    let mut count: HashMap<String, usize> = HashMap::new();

    process_file(&input_file1, &mut count)?;
    process_file(&input_file2, &mut count)?;

    let mut wtr = WriterBuilder::new().from_path(output_file)?;
    let mut has_duplicates = false;

    for (num, freq) in &count {
        if *freq > 1 {
            wtr.write_record(&[num])?;
            has_duplicates = true;
        }
    }

    if has_duplicates {
        println!("Duplicates have been written to '{}'.", output_file);
    } else {
        println!("No duplicates found.");
    }

    Ok(())
}

