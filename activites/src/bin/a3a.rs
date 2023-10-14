fn main (){
    let x: bool = true;
    display(x);
}

fn display (x: bool) {
    if x {
        println!("hello");
    } else {
        println!("goodbye");
    }
}