use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<String, u32> = HashMap::new();
    stock.insert("Chair".to_string(), 5);
    stock.insert("Bed".to_string(), 3);
    stock.insert("Table".to_string(), 2);
    stock.insert("Couch".to_string(), 0);
    for (item, amount) in stock.iter() {
        match amount {
            0 => println!("{} is out of stock.", item),
            _ => println!("{} unities of {} in stock", amount, item)
        }
    }
}