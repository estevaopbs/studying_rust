struct Person {
    name: String,
    age: u8,
    fav_color: String,
}

impl Person {
    fn print(&self) {
        println!("{} is {} years old and their favorite color is {}", self.name, self.age, self.fav_color);
    }
}

fn main(){
    let mut people: Vec<Person> = Vec::new();
    people.push(Person{name: String::from("John"), age: 8, fav_color: String::from("blue")});
    people.push(Person{name: String::from("Jane"), age: 15, fav_color: String::from("red")});
    people.push(Person{name: String::from("Jack"), age: 6, fav_color: String::from("green")});
    for person in people.iter() {
        if person.age <= 10 {
            person.print();
        }
    }
}