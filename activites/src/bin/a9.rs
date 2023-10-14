fn coordinate() -> (i32, i32) {
    (2, 8)
}

fn main() {
    let (x, y) = coordinate();
    if y > 5 {
        println!("y = {} > 5", x);
    } else if y < 5{
        println!("y = {} < 5", y);
    }
    else {
        println!("y = {} = 5", y)
    }
}