use polars::{
    df,
    prelude::{DataFrame, DataType, NamedFrom, Series, TimeUnit},
};
use std::collections::HashMap;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

pub fn main_table() -> HashMap<String, DataType> {
    map! {
        "ID".to_string() => DataType::UInt32,
        "NAME".to_string() => DataType::UInt32,
        "AMOUNT".to_string() => DataType::UInt32,
        "VALUE".to_string() => DataType::Float64,
        "DATETIME".to_string() => DataType::Datetime(TimeUnit::Milliseconds, None),
        "CURRENCY".to_string() => DataType::UInt32,
        "RECIPIENT".to_string() => DataType::UInt32,
        "SITUATION".to_string() => DataType::UInt32,
        "ACTIVE".to_string() => DataType::UInt8
    }
}

pub fn history_table() -> HashMap<String, DataType> {
    map! {
        "ID".to_string() => DataType::UInt32,
        "FKEY".to_string() => DataType::UInt32,
        "ACTION".to_string() => DataType::UInt8,
        "UNDOREDO".to_string() => DataType::UInt8
    }
}

pub fn name_table() -> HashMap<String, DataType> {
    map! {
        "ID".to_string() => DataType::UInt32,
        "NAME".to_string() => DataType::Utf8
    }
}

pub fn situation_table() -> HashMap<String, DataType> {
    map! {
        "ID".to_string() => DataType::UInt32,
        "SITUATION".to_string() => DataType::Utf8
    }
}

pub fn recipient_table() -> HashMap<String, DataType> {
    map! {
        "ID".to_string() => DataType::UInt32,
        "RECIPIENT".to_string() => DataType::Utf8
    }
}

pub fn currency_table() -> HashMap<String, DataType> {
    map! {
        "ID".to_string() => DataType::UInt32,
        "CURRENCY".to_string() => DataType::Utf8
    }
}

pub fn insert_table() -> HashMap<String, DataType> {
    map! {
        "ID".to_string() => DataType::UInt32,
        "FKEY".to_string() => DataType::UInt32
    }
}

pub fn delete_table() -> HashMap<String, DataType> {
    map! {
        "ID".to_string() => DataType::UInt32,
        "FKEY".to_string() => DataType::UInt32
    }
}

pub fn restore_table() -> HashMap<String, DataType> {
    map! {
        "ID".to_string() => DataType::UInt32,
        "FKEY".to_string() => DataType::UInt32
    }
}

pub fn update_table() -> HashMap<String, DataType> {
    map! {
        "ID".to_string() => DataType::UInt32,
        "FKEY".to_string() => DataType::UInt32,
        "NEW NAME".to_string() => DataType::UInt32,
        "NEW AMOUNT".to_string() => DataType::UInt32,
        "NEW VALUE".to_string() => DataType::Float64,
        "NEW DATETIME".to_string() => DataType::Datetime(TimeUnit::Milliseconds, None),
        "NEW CURRENCY".to_string() => DataType::UInt32,
        "NEW RECIPIENT".to_string() => DataType::UInt32,
        "NEW SITUATION".to_string() => DataType::UInt32,
        "NEW ACTIVE".to_string() => DataType::UInt8
    }
}

pub fn action_table() -> HashMap<String, DataType> {
    map! {
        "ID".to_string() => DataType::UInt8,
        "ACTION".to_string() => DataType::UInt8
    }
}

pub fn copy_table() -> HashMap<String, DataType> {
    map! {
        "ID".to_string() => DataType::UInt32,
        "FKEY_C".to_string() => DataType::UInt32,
        "FKEY_P".to_string() => DataType::UInt32
    }
}

pub fn sequence_table() -> HashMap<String, DataType> {
    map! {
        "TABLE".to_string() => DataType::UInt8,
        "NEXTVALUE".to_string() => DataType::UInt8
    }
}

pub fn all_tables() -> HashMap<String, HashMap<String, DataType>> {
    map! {
        "MAIN".to_string() => main_table(),
        "HISTORY".to_string() => history_table(),
        "NAME".to_string() => name_table(),
        "SITUATION".to_string() => situation_table(),
        "RECIPIENT".to_string() => recipient_table(),
        "CURRENCY".to_string() => currency_table(),
        "INSERT".to_string() => insert_table(),
        "DELETE".to_string() => delete_table(),
        "RESTORE".to_string() => restore_table(),
        "UPDATE".to_string() => update_table(),
        "COPY".to_string() => copy_table(),
        "SEQUENCE".to_string() => sequence_table(),
        "ACTION".to_string() => action_table()
    }
}

pub fn create_table(columns: HashMap<String, DataType>) -> DataFrame {
    let mut columns_vec: Vec<Series> = vec![];
    for (name, dtype) in &columns {
        columns_vec.push(Series::new_empty(name, dtype))
    }
    DataFrame::new(columns_vec).unwrap()
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum AllTables {
    MAIN,
    HISTORY,
    NAME,
    SITUATION,
    RECIPIENT,
    CURRENCY,
    INSERT,
    DELETE,
    RESTORE,
    UPDATE,
    COPY,
    SEQUENCE,
    ACTION,
    DO,
}

pub fn actions_table() -> DataFrame {
    let id = Series::new_empty("ID", &DataType::UInt8);
    let action = Series::new_empty("ACTION", &DataType::Utf8);
    let actions_dataframe = DataFrame::new(vec![id, action]).unwrap();
    actions_dataframe
        .vstack(
            &df! [
                "ID" => [0, 1, 2, 3, 4],
                "ACTION" => ["INSERT", "DELETE", "UPDATE", "RESTORE", "COPY"]
            ]
            .unwrap(),
        )
        .unwrap()
}

pub fn do_table() -> DataFrame {
    let id = Series::new_empty("ID", &DataType::UInt8);
    let do_status = Series::new_empty("DO", &DataType::Utf8);
    let do_dataframe = DataFrame::new(vec![id, do_status]).unwrap();
    do_dataframe
        .vstack(
            &df! [
                "ID" => [0, 1],
                "ACTION" => ["DONE", "UNDONE"]
            ]
            .unwrap(),
        )
        .unwrap()
}
