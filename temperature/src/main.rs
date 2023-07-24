//Temperature in degrees Fahrenheit (°F) = (Temperature in degrees Celsius (°C) * 9/5) + 32
//Temperature in degrees Celsius (°C) = (Temperature in degrees Fahrenheit (°F) - 32) * 5/9

use colored::*;
use std::io;

fn celtofh(c: f64) -> f64 {
    (c) * (9.0 / 5.0) + 32.0
}

fn fhtocel(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

fn main() {
    println!("{}", "Do you want to convert Celsius or Fahreheit?".green());
    println!("{}", "Input celsius or fahrenheit for conversion.".green());

    let mut celorfah = String::new();
    io::stdin()
        .read_line(&mut celorfah)
        .expect("Failed to read line");

    let celorfah = match celorfah.trim().parse() {
        Ok(celorfah) => celorfah,
        Err(_) => "Please Input the correct type!".to_string(),
    };

    let selected = String::from(celorfah);
    if selected == "celsius".to_string() {
        println!("{}", "You want to convert Fahrenheit to Celsius.".yellow());
    } else if selected == "fahrenheit".to_string() {
        println!("{}", "You want to convert Celsius to Fahrenheit.".yellow());
    } else {
        println!("{}", "Please input the correct type!".red());
    }

    println!("{}", "Input your temperature here:".green());
    let mut temps = String::new();
    io::stdin()
        .read_line(&mut temps)
        .expect("Failed to read line at temps");

    let temps = match temps.trim().parse() {
        Ok(temps) => temps,
        Err(_) => -1.0,
    };

    match selected.as_str() {
        "celsius" => println!("{}° Celsius", fhtocel(temps)),
        "fahrenheit" => println!("{}° Fahrenheit", celtofh(temps)),
        _ => println!("Temperature {:?}", selected),
    };
}
