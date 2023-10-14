use home::home_dir;
use std::{path::{Path, PathBuf}, fs, process::exit};
use polars::{prelude::*, io::csv};
use std::fs::File;
use clap::{ValueEnum};
use super::{UserInput, AddCommand};
use super::tables;

fn save_df_as_csv(mut df: DataFrame, filename: &str) {
    let mut df_file = File::create(home_dir().unwrap().join(format!(".bills/{}.csv", filename))).unwrap();
    CsvWriter::new(&mut df_file)
        .has_header(true)
        .finish(&mut df)
        .unwrap();
}

fn main_dir_exists() -> bool {
    home_dir().unwrap().join(".bills").is_dir()
}

fn csv_file_exists(filename: &str) -> bool {
    home_dir().unwrap().join(format!(".bills/{}.csv", filename)).is_file()
}

fn create_main_dir() {
    match fs::create_dir(home_dir().unwrap().join(".bills")){
        Ok(()) => (),
        Err(any) => exit_with_error(any.to_string(), None) 
    }
}

fn exit_with_error(error: String, exit_code: Option<i32>) {
    println!("{:?}", error);
    match exit_code {
        None => exit(0),
        Some(any) => exit(any)
    }
}

fn create_csv_files(create_main: bool, create_hist: bool) {
    match create_main {
        true => create_main_csv(),
        false => ()
    };
    match create_hist {
        true => create_hist_csv(),
        false => ()
    }
}

fn create_main_csv() {
    let id = Series::new_empty("ID", &DataType::UInt32);
    let name = Series::new_empty("NAME", &DataType::UInt32);
    let amount = Series::new_empty("AMOUNT", &DataType::UInt32);
    let value = Series::new_empty("VALUE", &DataType::Float64);
    let datetime = Series::new_empty("DATETIME", &DataType::Datetime(TimeUnit::Milliseconds, None));
    let currency = Series::new_empty("CURRENCY", &DataType::UInt32);
    let recipient = Series::new_empty("RECIPIENT", &DataType::UInt32);
    let situation = Series::new_empty("SITUATION", &DataType::UInt32);
    let active = Series::new_empty("MODE", &DataType::UInt8);
    let mut main_df = DataFrame::new(vec![id, name, amount, value, datetime, currency, recipient, situation, active]).unwrap();
    let mut main_file = File::create(home_dir().unwrap().join(".bills/main.csv")).unwrap();
    CsvWriter::new(&mut main_file)
        .has_header(true)
        .finish(&mut main_df)
        .unwrap();
}

fn create_hist_csv() {
    let id = Series::new_empty("ID", &DataType::UInt32);
    let action = Series::new_empty("ACTION", &DataType::UInt8);
    let mut  hist_df  = DataFrame::new(vec![id, action]).unwrap();
    let mut hist_file = File::create(home_dir().unwrap().join(".bills/main.csv")).unwrap();
    CsvWriter::new(&mut hist_file)
        .has_header(true)
        .finish(&mut hist_df)
        .unwrap();
}

fn populate () {
    create_main_dir()

}

fn startup () {
    if !main_dir_exists() {
        create_main_dir()
    }
}

fn add (command: AddCommand) {

}