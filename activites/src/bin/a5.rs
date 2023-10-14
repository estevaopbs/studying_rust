fn main(){
    let mut x = 1;
    loop {
        println!("{} ", x);
        if x == 4 {
            break;
        }
        x += 1;
    }
}