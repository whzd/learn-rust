use core::panic;
use std::{collections::HashMap, io};

use itertools::Itertools;

fn main() {
    println!("Company management!");

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("\nSelect a option number:");
        println!("0. Exit");
        println!("1. Add employees to department.");
        println!("2. Get department employees");
        println!("3. Get company employees.");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read a valid temperature.");

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only numbers allowed!\n");
                continue;
            }
        };

        match option {
            0 => break,
            1 => {
                let (person, department) = get_person_department();
                company
                    .entry(department.to_lowercase())
                    .or_insert(Vec::new())
                    .push(person.to_lowercase());
            }
            2 => {
                let department = get_department(&company);
                print_department(&company, department);
            }
            3 => print_company(&company),
            _ => {
                println!("Not a valid option!\n");
                continue;
            }
        }
    }
}

fn get_person_department() -> (String, String) {
    let entry: String = loop {
        println!("\nType the entry to add.");

        let mut entry = String::new();
        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read a valid entry.");

        let words = entry.trim().to_lowercase();
        let tokens = words_count::count(&words);
        if tokens.words == 4
            && words.to_lowercase().contains("add")
            && words.to_lowercase().contains("to")
        {
            break words;
        } else {
            println!("\nInvalid entry. Entry should match the following format:");
            println!("Add <person> to <department>");
        }
    };
    let &[_, person, _, department] = entry.split_whitespace().collect::<Vec<_>>().as_slice()
    else {
        panic!("Something not expected happened!")
    };
    (person.to_string(), department.to_string())
}

fn get_department(company: &HashMap<String, Vec<String>>) -> String {
    let department: String = loop {
        println!("\nType and existing department name.");

        let mut entry = String::new();
        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read a valid entry.");
        let department = entry.trim().to_lowercase();
        if company.contains_key(&department) {
            break department;
        } else {
            println!("Not an existing department.\n")
        }
    };
    department
}

fn print_department(company: &HashMap<String, Vec<String>>, department: String) {
    let mut employees = company.get(&department).unwrap().clone();
    employees.sort();
    println!("\nDepartment: {department}");
    for employee in employees {
        println!("{employee}")
    }
}

fn print_company(company: &HashMap<String, Vec<String>>) {
    println!("\nCompany Roster");
    for key in company.keys().sorted() {
        println!("\nDepartment: {key}");
        let mut employees = company.get(key).unwrap().clone();
        employees.sort();
        for employee in employees {
            println!("{employee}")
        }
    }
}
