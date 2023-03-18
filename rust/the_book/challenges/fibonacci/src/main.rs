use std::io;

fn main() {
    println!("Fibonacci sequence number.");

    println!("Insert nth number of the fibonacci sequence you want to know.");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read number");

    let number: u32 = number.trim().parse().expect("Number not valid");

    println!("The value in postion {} of the fibonacci sequence is {}.", number, fibonacci(number));
}

// Max value allowed as number is 186
fn fibonacci(number: u32) -> u128{
    let mut a = 0;
    let mut b = 1;
    let mut c;
    if number == 0{
        return a;
    }else if number == 1{
        return b;
    }else{
        for _ in 2..number+1 {
            c = a + b;
            a = b;
            b = c;
        }
        return b;
    }
}
