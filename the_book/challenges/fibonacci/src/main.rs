use std::io;

fn fibonacci(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    fibonacci(n - 2) + fibonacci(n - 1)
}

fn main() {
    println!("Fibonnaci number calculator!");
    let number: u64 = loop {
        println!("Insert the number you discover.");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read a valid number.");

        let _: f64 = match number.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Not a valid number! It was to be grater than 0.");
                continue;
            }
        };
    };
    println!(
        "The number {number} of the Fibonacci sequence is: {}.",
        fibonacci(number)
    );
}
