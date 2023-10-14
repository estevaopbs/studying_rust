use polars::df;
use polars::prelude::*;

fn filter_by_id(table: &DataFrame, ids: Vec<u32>) -> DataFrame {
    df!{
        "ID" => &[1, 3, 5],
        "VALUE" => &["B", "D", "F"]
    }.unwrap()
}

fn main() {
    let table = df!{
        "ID" => &[0, 1, 2, 3, 4, 5],
        "VALUE" => &["A", "B", "C", "D", "E", "F"]
    }.unwrap();
    let ids = vec![1, 3, 5];
    let filtered_table = filter_by_id(&table, ids);
    println!("{:?}", table);
    println!("{:?}", filtered_table);
}
