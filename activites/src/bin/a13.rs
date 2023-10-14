fn main() {
    let vec_a = vec![10, 20, 30, 40];
    for i in &vec_a {
        match i {
            30 => println!("Thirty"),
            _ => println!("{}", i)
        }
    };
    println!("len = {}", vec_a.len())
}