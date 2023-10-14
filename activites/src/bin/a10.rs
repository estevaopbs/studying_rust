fn main () {
    let x: i32 = 101;
    let _verify: &str = if x > 100 {
        "its big"
    } else {
        "its small"
    };
    println!("{}", _verify)
}