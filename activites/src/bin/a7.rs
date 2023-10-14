enum Colors {
    Red,
    Green,
    Blue
}

fn main() {
    let c = Colors::Blue;
    match c {
        Colors::Red => println!("red"),
        Colors::Green => println!("green"),
        Colors::Blue => println!("blue"),
    }
}