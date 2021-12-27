// Temperature conversion between Celsius and Fahrenheit and vice versa.

use std::io;

fn main() {
    // let units = ["C", "F"];
    loop {
        println!("Enter a unit to convert: ");
        let unit = read_input().trim().to_uppercase();

        println!("Enter a temperature: ");
        let temperature: f64 = match read_input().trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if unit == "C" {
            println!("{}{} is {}F", temperature, unit, c_to_f(temperature));
            break;
        } else if unit == "F" {
            println!("{}{} is {}C", temperature, unit, f_to_c(temperature));
            break;
        } else {
            continue;
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input
}
fn c_to_f(value: f64) -> i64 {
    (value * (9.0 / 5.0) + 32.0) as i64
}

fn f_to_c(value: f64) -> i64 {
    ((value - 32.0) * 5.0 / 9.0) as i64
}
