// 1
use csv::ReaderBuilder;
use csv::WriterBuilder;
use std::collections::HashSet;
use std::error::Error;

fn read_files_into_set(files: &[&str]) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut numbers = HashSet::new();

    for file in files {
        let mut rdr = ReaderBuilder::new().has_headers(false).from_path(file)?;
        for result in rdr.records() {
            let record = result?;
            for value in record.iter() {
                numbers.insert(value.to_string());
            }
        }
    }
    Ok(numbers)
}

fn main() -> Result<(), Box<dyn Error>> {
    let list1_files = vec!["file1.csv", "file2.csv"];
    let list2_files = vec!["file3.csv", "file4.csv"];
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
        "There are {} numbers from list 2 in list 1.",
        common_numbers.len()
    );
    println!("Common numbers have been written to '{}'.", output_file);

    Ok(())
}
