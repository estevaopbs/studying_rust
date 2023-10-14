struct Customer {
    age: u8,
}

fn verify_age(customer: &Customer) -> Result<(), String> {
    match customer.age {
        0..=20 => Err("Customer is too young".to_string()),
        21.. => Ok(()),
    }
}

fn main() {
    let customers: Vec<Customer> = vec![
        Customer {
            age: 20
        },
        Customer {
            age: 30
        }
    ];
    for customer in customers {
        println!("{:?}", verify_age(&customer))
        }
}