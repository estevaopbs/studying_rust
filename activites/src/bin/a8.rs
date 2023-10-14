enum Flavors {
    Sparkling,
    Sweet,
    Fuity
}

struct Drink {
    flavor: Flavors,
    ounces: f64
}

fn display_drink(d: Drink) {
    match d.flavor {
        Flavors::Sparkling => println!("You chose sparkling"),
        Flavors::Sweet => println!("You chose sweet"),
        Flavors::Fuity => println!("You chose fuity")
    }
    println!("You chose {:?} ounces", d.ounces);
}

fn main() {
    let d = Drink {
        flavor: Flavors::Sparkling,
        ounces: 12.0
    };
    display_drink(d);
}