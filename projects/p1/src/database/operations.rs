use super::tables::*;
use crate::user_interface::cli::Active;
use polars::lazy::dsl::Expr;
use polars::sql::SQLContext;
use polars::{prelude::*, sql};
use std::collections::HashMap;

fn insert(row: DataFrame, table: DataFrame) -> DataFrame {
    table.vstack(&row).unwrap()
}

fn delete(ids: Series, table: DataFrame) -> (DataFrame, DataFrame) {
    let deleted_filter = col("ID").is_in(lit(ids.clone()));
    let deleted_rows = table
        .clone()
        .lazy()
        .filter(deleted_filter)
        .collect()
        .unwrap();
    let remaining_filter = col("ID").is_in(lit(ids)).not();
    let remaining_rows = table.lazy().filter(remaining_filter).collect().unwrap();
    (deleted_rows, remaining_rows)
}

fn select(ids: Series, table: DataFrame) {
    let filter_expr = col("ID").is_in(lit(ids));
    table.lazy().filter(filter_expr).collect().unwrap();
}

fn filter(mut query: String, tables: HashMap<AllTables, DataFrame>, active: Active) -> DataFrame {
    let main_df = &tables[&AllTables::MAIN];
    let situation_df = &tables[&AllTables::SITUATION];
    let recipient_df = &tables[&AllTables::RECIPIENT];
    let currency_df = &tables[&AllTables::CURRENCY];
    let name_df = &tables[&AllTables::NAME];
    let mut bills_df = match active {
        Active::Active => main_df
            .clone()
            .lazy()
            .filter(col("ACTIVE").eq(1))
            .collect()
            .unwrap()
            .drop("ACTIVE")
            .unwrap(),
        Active::Inactive => main_df
            .clone()
            .lazy()
            .filter(col("ACTIVE").eq(0))
            .collect()
            .unwrap()
            .drop("ACTIVE")
            .unwrap(),
        Active::All => main_df.clone(),
    };
    bills_df = bills_df
        .left_join(&name_df, ["NAME"], ["ID"])
        .unwrap()
        .left_join(&currency_df, ["CURRENCY"], ["ID"])
        .unwrap()
        .left_join(&recipient_df, ["RECIPIENT"], ["ID"])
        .unwrap()
        .left_join(&situation_df, ["SITUATION"], ["ID"])
        .unwrap();
    let mut context = SQLContext::try_new().unwrap();
    context.register("BILLS", bills_df.clone().lazy());
    context.execute(&query).unwrap().collect().unwrap()
}

fn sql_view_op(tables: HashMap<AllTables, DataFrame>, query: String) -> DataFrame {
    let main_df = &tables[&AllTables::MAIN];
    let history_df = &tables[&AllTables::HISTORY];
    let action_df = &tables[&AllTables::ACTION];
    let situation_df = &tables[&AllTables::SITUATION];
    let recipient_df = &tables[&AllTables::RECIPIENT];
    let currency_df = &tables[&AllTables::CURRENCY];
    let insert_df = &tables[&AllTables::INSERT];
    let delete_df = &tables[&AllTables::DELETE];
    let restore_df = &tables[&AllTables::RESTORE];
    let update_df = &tables[&AllTables::UPDATE];
    let copy_df = &tables[&AllTables::COPY];
    let do_df = &tables[&AllTables::DO];
    let name_df = &tables[&AllTables::NAME];
    let sequence_df = &tables[&AllTables::SEQUENCE];
    let bills_df = main_df
        .left_join(&name_df, ["NAME"], ["ID"])
        .unwrap()
        .left_join(&currency_df, ["CURRENCY"], ["ID"])
        .unwrap()
        .left_join(&recipient_df, ["RECIPIENT"], ["ID"])
        .unwrap()
        .left_join(&situation_df, ["SITUATION"], ["ID"])
        .unwrap();
    let joined_history = history_df
        .left_join(&action_df, ["ACTION"], ["ID"])
        .unwrap()
        .left_join(&do_df, ["DO"], ["ID"])
        .unwrap();
    let joined_insert = &insert_df.left_join(&bills_df, ["FKEY"], ["ID"]).unwrap();
    let joined_delete = &delete_df.left_join(&bills_df, ["FKEY"], ["ID"]).unwrap();
    let joined_restore = &delete_df.left_join(&bills_df, ["FKEY"], ["ID"]).unwrap();
    let joined_copy = &copy_df.left_join(&bills_df, ["FKEY"], ["ID"]).unwrap();
    let joined_update = &update_df.left_join(&bills_df, ["FKEY"], ["ID"]).unwrap();
    let mut context = SQLContext::try_new().unwrap();
    context.register("BILLS", bills_df.clone().lazy());
    context.register("HISTORY", joined_history.clone().lazy());
    context.register("INSERT_DF", joined_insert.clone().lazy());
    context.register("DELETE_DF", joined_delete.clone().lazy());
    context.register("RESTORE_DF", joined_restore.clone().lazy());
    context.register("COPY_DF", joined_copy.clone().lazy());
    context.register("UPDATE_DF", joined_update.clone().lazy());
    context.execute(&query).unwrap().collect().unwrap()
}
