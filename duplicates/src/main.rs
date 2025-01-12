use csv::ReaderBuilder;
use csv::WriterBuilder;
use std::collections::HashMap;
use std::error::Error;

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
    // Input and output file names
    let input_file1 = "21234.csv";
    let input_file2 = "21235.csv";
    let output_file = "21234_21235.csv";

    let mut count: HashMap<String, usize> = HashMap::new();

    // Process both input files
    process_file(input_file1, &mut count)?;
    process_file(input_file2, &mut count)?;

    // Write duplicates to the output file
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