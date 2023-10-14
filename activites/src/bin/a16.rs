struct Student{
    name: String,
    locker: Option<u32>,
}

fn main(){
    let mut students: Vec<Student> = Vec::new();
    students.push(Student{name: String::from("John"), locker: Some(1)});
    students.push(Student{name: String::from("Jane"), locker: Some(2)});
    students.push(Student{name: String::from("Jack"), locker: None});
    for student in students.iter() {
        match student.locker {
            Some(locker) => println!("{} has a locker at {}", student.name, locker),
            None => println!("{} does not have a locker", student.name),
        }
    }
}