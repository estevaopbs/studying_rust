fn compare_with_5(x: i32) {
    if x > 5{
        println!(">5")
    } else if x < 5{
        println!("<5")
    }
    else {
        println!("=5")
    }
}

fn main() {
    let x: i32 = 5;
    compare_with_5(x)
}