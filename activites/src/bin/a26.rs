// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono;
fn main() {
    println!(
        "{}",
        chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S")
    );
}
