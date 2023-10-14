#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue
}

impl Color {
    fn to_string_uiui (self: Color) -> String {
        format!("{:?}", self)
    }
}

struct ShippingBox {
    dimensions: (u32, u32, u32),
    weight: u32,
    color: Color
}

impl ShippingBox {
    fn new (dimensions: (u32, u32, u32), weight: u32, color: Color) -> ShippingBox {
        ShippingBox {
            dimensions: dimensions,
            weight: weight,
            color: color
        }
    }

    fn description(&self) -> String {
        format!("Dimensions: {} {} {}\nWeight: {}\nColor: {:?}", self.dimensions.0, self.dimensions.1, self.dimensions.2, self.weight, self.color)
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn main() {
    let box1 = ShippingBox::new((1, 2, 3), 4, Color::Red);
    println!("{}", box1.description());
    print!("{}", Color::Red.to_string_uiui())
}