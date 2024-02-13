use std::io;

fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * 1.8 + 32.0
}

fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("Temperature Converter!");

    let scale: char = loop {
        println!("Will you Celsius(°C) or Fahrenheit(°F) ?");

        let mut scale = String::new();
        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read a valid scale.");

        match scale.trim().to_lowercase().as_str() {
            "celsius" | "c" => {
                break 'c';
            }
            "fahrenheit" | "f" => {
                break 'f';
            }
            _ => {
                println!("Invalid type!");
            }
        }
    };

    let temperature: f64 = loop {
        println!("What is the temperature?");

        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read a valid temperature.");

        let _: f64 = match temperature.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Not a valid temperature.");
                continue;
            }
        };
    };

    if scale == 'c' {
        println!(
            "{temperature} °C is equal to {:.1} °F.",
            celsius_to_fahrenheit(temperature)
        );
    } else {
        println!(
            "{temperature} °F is equal to {:.1} °C.",
            fahrenheit_to_celsius(temperature)
        );
    }
}
