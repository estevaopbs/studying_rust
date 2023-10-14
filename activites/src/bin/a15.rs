#[derive(Debug)]
enum Ticket {
    Backstage(u32, String),
    Standard(u32),
    Vip(u32, String),
}

fn main() {
    let mut tickets: Vec<Ticket> = Vec::new();
    tickets.push(Ticket::Backstage(100, String::from("John")));
    tickets.push(Ticket::Standard(25));
    tickets.push(Ticket::Vip(50, String::from("Jane")));
    for ticket in tickets.iter() {
        match ticket {
            Ticket::Backstage(price, name) => println!("{} is a backstage ticket for ${}", name, price),
            Ticket::Standard(price) => println!("Standard ticket for ${}", price),
            Ticket::Vip(price, name) => println!("{} is a VIP ticket for ${}", name, price),
        }
    }
    println!("{:#?}", tickets)
}