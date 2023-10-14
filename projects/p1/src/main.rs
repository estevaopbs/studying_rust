pub mod user_interface;
use polars::sql;
use user_interface::{FilterSubcommand, UserInput};
mod database;
use clap::Parser;
use database::*;
use home::home_dir;
use mimalloc::MiMalloc;
use polars::prelude::DataType;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// Read the file '~/.bills/tz' and converts the content to i32. If the file does not ex

fn main() {
    let user_input: UserInput = UserInput::parse();
    print!("{:#?}", user_input);
}
