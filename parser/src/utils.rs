use calamine::{open_workbook_auto, DataType, Reader};
use std::error::Error;
use std::path::Path;
use xlsxwriter::{Workbook};
use crate::consts::{ZIP_CODE_RANGES, STATE_ZIP, STATE_AREA_CODES};


// this will read an excel file, each line of the file will become an array
pub fn read_excel(
    file: &str,
    sheet: Option<usize>,
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let path = Path::new(file);
    
    // Open workbook (Sheets<BufReader<File>>)
    let mut workbook = open_workbook_auto(path)?;

    let sheet_index = sheet.unwrap_or(0);
    let sheet = workbook
        .worksheet_range_at(sheet_index)
        .ok_or("Sheet not found")??;

    let mut rows = Vec::new();

    for row in sheet.rows() {
        let mut cells = Vec::new();
        for cell in row {
            match cell {
                DataType::String(s) => cells.push(s.to_string()),
                DataType::Float(f) => cells.push(f.to_string()),
                DataType::Int(i) => cells.push(i.to_string()),
                _ => cells.push(String::new()),
            }
        }
        rows.push(cells);
    }
    Ok(rows)
}

pub fn write_excel(file: &str, rows: Vec<Vec<String>>, col_max: usize) -> Result<(), Box<dyn std::error::Error>>{
    let workbook = Workbook::new(file)?;
    let mut worksheet = workbook.add_worksheet(None)?;

    for (row_num, row) in rows.iter().enumerate() {
        for col_num in 0..=col_max {
            if let Some(cell_value) = row.get(col_num) {
                worksheet.write_string(row_num as u32, col_num as u16, cell_value, None)?;
            }
        }
    }
    workbook.close()?;

    println!("{} lines written", rows.len());
    Ok(())
}

pub fn clean_phone_number(row: &mut Vec<String>) {
    if let Some(phone) = row.get_mut(8) {
        *phone = phone.chars().filter(|c| c.is_digit(10)).collect();
        if phone.len() == 11 && phone.starts_with('1') {
            *phone = phone[1..].to_string();
        }
    }
}

pub fn uppercase_state(row: &mut Vec<String>) {
    if let Some(state) = row.get_mut(6) {
        *state = state.to_uppercase();
    }
}

// Fills in missing state from zipcode info
pub fn populate_state_from_zip(row: &mut Vec<String>) {
    if let Some(zip_str) = row.get(7) {
        if let Ok(zip) = zip_str.parse::<usize>() {
            for (state, ranges) in ZIP_CODE_RANGES.entries() {
                let mut iter = ranges.iter();
                while let (Some(&min), Some(&max)) = (iter.next(), iter.next()) {
                    if zip >= min && zip <= max {
                        row[6] = state.to_string();
                        return;
                    }
                }
            }
        }
    }
}


// if the area code value is longer than 5 chars, cuts it down to 5
pub fn truncate_zip_code(row: &mut Vec<String>) {
    if let Some(zip_str) = row.get_mut(7) {    
            if zip_str.len() > 5 {
                *zip_str = zip_str.chars().take(5).collect();
                println!("Truncated ZIP code to 5 characters for row: {:?}", row.get(0));
            }
    }
}

// Adds zipcode if zipcode is missing and state is present
pub fn populate_zip_code(row: &mut Vec<String>) {
    if !row[6].is_empty() && row[7].is_empty() {
        let state = &row[6];
        if let Some(zip_code) = STATE_ZIP.get(state) {
            row[7] = zip_code.to_string();
        }
    }
}


// Fills in state and zip from area code
pub fn populate_state_zip_from_area_code(row: &mut Vec<String>) {
    if let Some(area_code_str) = row.get(8) {
        let area_code = &area_code_str[..3.min(area_code_str.len())];
        
        for (state, codes) in STATE_AREA_CODES.entries() {
            if codes.contains(&area_code.parse::<usize>().unwrap_or(0)) {
                row[6] = state.to_string();
                
                if let Some(&zip) = STATE_ZIP.get(state) {
                    row[7] = zip.to_string();
                }
                break;
            }
        }
    }
}
