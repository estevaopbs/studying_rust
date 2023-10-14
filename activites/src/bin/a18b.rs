#[derive(Debug)]
enum Role {
    MaintenanceCrew,
    MarketingDepartmentEmployee,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnicians
}

struct Employee {
    name: String,
    role: Role,
    employed: bool
}

impl Employee {
    fn may_access_building(self: &Employee) -> Result<(), String> {
        match self.employed {
            true => match self.role {
                Role::MaintenanceCrew => Ok(()),
                Role::MarketingDepartmentEmployee => Ok(()),
                Role::Manager => Ok(()),
                _ => Err(format!("Employees from role {:?} cannot access building", self.role))
            }
            false => Err(format!("{} is not employed right now.", self.name))
        }
    }
}

fn main() {
    let employees: Vec<Employee> = vec![
        Employee{
            name: String::from("Roberto"),
            role: Role::MaintenanceCrew,
            employed: true
        },
        Employee{
            name: String::from("Geraldo"),
            role: Role::KitchenStaff,
            employed: true
        },
        Employee{
            name: String::from("Marilda"),
            role: Role::MarketingDepartmentEmployee,
            employed: false
        }
    ];
    for employee in &employees {
        println!("{}\n{:?}\n", employee.name, employee.may_access_building())
    }
}