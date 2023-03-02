mod date;
mod person;

use std::io;
//use std::collections::HashMap;

use date::date::Date;
use person::Person;
fn main() {
    let p = create_person();
    let p_str = p.repr();
    println!("{p_str}");
    greeting(p.get_name())
}

fn greeting(name: &String) {
    println!("Hi, {}", name);
}

fn read_terminal(msg: &str) -> String {
    println!("{}", msg);
    let mut line = String::new();
    let _ioresult = io::stdin().read_line(&mut line);
    line.trim().to_owned()
}

fn create_date() -> Date {
    let mut input_fail = true;
    while input_fail {
        let birthday = read_terminal("Give birthday (dd/mm/yyyy): ");
        let splitdate: Vec<&str> = birthday.split('/').collect();

        let y = match splitdate[2].parse::<i16>() {
            Ok(x) => x,
            Err(_) => {
                println!("Error parsing year! Try again!");
                input_fail = true;
                continue;
            }
        };

        let m = match splitdate[1].parse::<u8>() {
            Ok(x) => x,
            Err(_) => {
                println!("Failed parsing month! Try again!");
                input_fail = true;
                continue;
            }
        };

        let d = match splitdate[0].parse::<u8>() {
            Ok(x) => x,
            Err(_) => {
                println!("Failed parsing day! Try again!");
                input_fail = true;
                continue;
            }
        };

        match Date::new(y, m, d) {
            Ok(x) => return x,
            Err(y) => {
                println!("{y}");
                input_fail = true;
                continue;
            }
        };
    }
    return Date::default();
}

fn input_height() -> f32 {
    loop {
        match read_terminal("How tall r u? [m]: ").parse() {
            Ok(x) => return x,
            Err(y) => println!("{y}"),
        }
    }
}

fn create_person() -> Person {
    let name = read_terminal("Give name");
    let bday = create_date();
    let bd_str = bday.repr();
    let nrod = bday.get_month().get_nr_of_days().to_string();
    println!("{bd_str}");
    println!("{nrod}");

    loop {
        let height = input_height();
        match Person::new(&name, &bday, height) {
            Ok(x) => return x,
            Err(y) => println!("{y}"),
        }
    }
}
