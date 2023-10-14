struct GroceryItem {
    id: u32,
    quantity: u32
}


fn display_qtty(item: &GroceryItem) {
    println!("{}", item.quantity);
}


fn display_id(item: &GroceryItem) {
    println!("{}", item.id);
}


fn main () {
    let item1 = GroceryItem { id: 1, quantity: 2 };
    display_id(&item1);
    display_qtty(&item1);
}