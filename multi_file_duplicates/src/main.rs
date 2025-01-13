use csv::ReaderBuilder;
use csv::WriterBuilder;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

fn read_files_into_set(files: &[String]) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut numbers = HashSet::new();

    for file in files {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(file)?; // Open file with csv reader
        for result in rdr.records() {
            let record = result?; // Process each record
            for value in record.iter() {
                numbers.insert(value.to_string());
            }
        }
    }
    Ok(numbers)
}

fn get_file_names(prompt: &str) -> Vec<String> {
    let mut files = Vec::new();
    let mut input = String::new();

    println!("{}", prompt);
    print!("How many files are there? ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed
    io::stdin().read_line(&mut input).unwrap();
    let file_count: usize = input.trim().parse().expect("Please enter a valid number.");

    for i in 1..=file_count {
        input.clear();
        print!("Enter the name of file {}: ", i);
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed
        io::stdin().read_line(&mut input).unwrap();
        files.push(input.trim().to_string()); // Add the trimmed filename
    }

    files
}

fn main() -> Result<(), Box<dyn Error>> {
    let list1_files = get_file_names("For List 1:");
    let list2_files = get_file_names("For List 2:");
    let output_file = "output.csv";

    let list1_numbers = read_files_into_set(&list1_files)?;
    let list2_numbers = read_files_into_set(&list2_files)?;

    let mut common_numbers = vec![];
    for num in list2_numbers {
        if list1_numbers.contains(&num) {
            common_numbers.push(num);
        }
    }

    let mut wtr = WriterBuilder::new().from_path(output_file)?;
    for num in &common_numbers {
        wtr.write_record(&[num])?;
    }

    println!(
        "There are {} numbers from List 2 in List 1.",
        common_numbers.len()
    );
    println!("Common numbers have been written to '{}'.", output_file);

    Ok(())
}