use std::io;

fn main() {
    println!("Temperature Converter!");

    println!("Will you insert Celsius(°C) or Fahrenheit(°F)?");
    let mut temp_format = String::new();
    loop{
        io::stdin()
            .read_line(&mut temp_format)
            .expect("Failed to read temperature format");
        match temp_format.trim().to_lowercase().as_str(){
            "celsius" | "c" => {
                temp_format = "celsius".to_string();
                break;
            },
            "fahrenheit" | "f" => {
                temp_format = "fahrenheit".to_string();
                break;
            },
            _ => {
                println!("Invalid type!");
                println!("Will you insert Celsius(°C) or Fahrenheit(°F)?");
            }
        };
    }

    println!("What is the temperature?");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read temperature");

    let temperature: i32 = temperature.trim().parse().expect("Please type a number!");

    if temp_format.eq("celsius") {
        println!("{}°C is equal to {:.1}°F", temperature, 
                 celsius_to_fahrenheit(temperature) );
    }else {
        println!("{}°F is equal to {:.1}°C", temperature, fahrenheit_to_celsius(temperature));

    }
}

fn celsius_to_fahrenheit(celsius: i32) -> f32{
    (9.0/5.0)*celsius as f32+32.0
}

fn fahrenheit_to_celsius(fahrenheit: i32) -> f32{
    (5.0/9.0)*(fahrenheit as f32-32.0)
}
