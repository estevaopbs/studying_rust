// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn get_perimeter(&self) -> u32;
}

#[derive(Debug)]
struct Triangle {
    sides: (u32, u32, u32),
}

impl Perimeter for Triangle {
    fn get_perimeter(&self) -> u32 {
        self.sides.0 + self.sides.1 + self.sides.2
    }
}

#[derive(Debug)]
struct Square {
    sides: (u32, u32, u32, u32),
}

impl Perimeter for Square {
    fn get_perimeter(&self) -> u32 {
        self.sides.0 + self.sides.1 + self.sides.2 + self.sides.3
    }
}

fn print_parameter(object: impl Perimeter) {
    println!("Perimeter: {}", object.get_perimeter())
}

fn main() {
    let triangle = Triangle { sides: (3, 4, 5) };
    println!("{:?}", triangle);
    print_parameter(triangle);
    let square = Square {
        sides: (3, 5, 3, 5),
    };
    println!("{:?}", square,);
    print_parameter(square);
}
