fn another_function(value: i32, unit_label: &str) {
    println!("Another function.");
    println!("The size is: {value}{unit_label}.");
}

fn five() -> i32 {
    // The last expression of a function is the returned value
    // If there is ";" at the end it would became a statement
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world!");

    another_function(17, "cm");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let z = five();
    println!("The value of z is: {z}");

    let z = plus_one(z);
    println!("The new value of z is: {z}");
}
