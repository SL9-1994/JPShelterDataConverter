use chrono::{DateTime, Local};
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    code: String,
    name: String,
    no: String,
    facility_name: String,
    address: String,
    flood: String,
    landslide: String,
    high_tide: String,
    earthquake: String,
    tsunami: String,
    large_fire: String,
    inland_flood: String,
    volcanic_phenomena: String,
    same_address_as_evacuation_site: String,
    latitude: String,
    longitude: String,
    remarks: String,
}

fn read_file_path() -> Result<String, Box<dyn Error>> {
    let mut path = String::new();
    io::stdin().read_line(&mut path)?;
    Ok(path.trim().to_string())
}

fn generate_output_file_name(input_file_path: &str) -> Result<String, Box<dyn Error>> {
    let now: DateTime<Local> = Local::now();
    let file_stem = Path::new(input_file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Failed to read file name")?;
    Ok(format!("{}_{}.json", file_stem, now.format("%Y%m%d%H%M%S")))
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter a CSV file path");
    let path = read_file_path()?;
    let output_file_name = generate_output_file_name(&path)?;

    let file = File::open(&path)?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: Record = result?;
        records.push(record);
    }

    let output_file = File::create(output_file_name)?;
    serde_json::to_writer_pretty(output_file, &records)?;

    Ok(())
}
