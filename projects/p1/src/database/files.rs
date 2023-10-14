use home::home_dir;
use polars::prelude::{CsvReader, CsvWriter, DataFrame, SerReader, SerWriter};
use std::fs::File;

pub fn create_csv(table_name: String, mut table: DataFrame) {
    let mut csv_file = File::create(
        home_dir()
            .unwrap()
            .join(format!("./bills/{}.csv", table_name)),
    )
    .unwrap();
    CsvWriter::new(&mut csv_file)
        .has_header(true)
        .finish(&mut table)
        .unwrap()
}

pub fn read_csv(table_name: String) -> DataFrame {
    CsvReader::from_path(
        home_dir()
            .unwrap()
            .join(format!("./bills/{}.csv", table_name)),
    )
    .unwrap()
    .has_header(true)
    .finish()
    .unwrap()
}
