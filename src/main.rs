use chrono::{DateTime, Local};
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    市町村コード: String,           // 市町村コード
    都道府県名及び市町村名: String, // 都道府県名及び市町村名
    市町村内の番号: String,         // 市町村内の番号
    #[serde(rename = "施設・場所名")]
    施設_場所名: String, // 施設・場所名
    住所: String,                   // 住所
    洪水: String,                   // 洪水
    #[serde(rename = "崖崩れ、土石流及び地滑り")]
    崖崩れ_土石流及び地滑り: String, // 崖崩れ、土石流及び地滑り
    高潮: String,                   // 高潮
    地震: String,                   // 地震
    津波: String,                   // 津波
    大規模火事: String,             // 大規模火事
    内水氾濫: String,               // 内水氾濫
    火山現象: String,               // 火山現象
    指定避難所との住所同一: String, // 指定避難所との住所同一
    緯度: String,                   // 緯度
    経度: String,                   // 経度
    備考: String,                   // 備考
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

    let mut records = vec![];

    // for result in reader.deserialize() {
    //     let record: Record = result?;
    //     records.push(record);
    // }

    for (i, result) in reader.deserialize().enumerate() {
        if i == 0 {
            // 1行目はヘッダーなのでスキップ
            continue;
        }
        let record: Record = result?;
        records.push(record);
    }

    let output_file = File::create(output_file_name)?;
    serde_json::to_writer_pretty(output_file, &records)?;

    Ok(())
}
