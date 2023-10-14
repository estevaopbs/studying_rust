use std::io;


enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate
}


impl PowerState {
    fn new(input: String) -> Option<PowerState> {
        match input.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None
        }
    }
}


fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_lowercase())
}

fn execute_power_state_command(input: PowerState) {
    use PowerState::*;
    match input {
        Off => println!("Turning off"),
        Sleep => println!("Sleeping"),
        Reboot => println!("Restarting"),
        Shutdown => println!("Shutting down"),
        Hibernate => println!("Hibernating"),
    }
}

fn main() {
    let new_power_state = PowerState::new(get_input().unwrap());
    match new_power_state {
        Some(any) => execute_power_state_command(any),
        None => println!("Input was not recognized")
    }
}
