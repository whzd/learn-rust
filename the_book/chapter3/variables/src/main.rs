const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // The mut keyword is required when reassing a value to a variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("In 3 hours there are {THREE_HOURS_IN_SECONDS} seconds.");

    let y = 5;

    // By using let again instead of reassining a new value it creates a new
    // variable y that will shadow the previous one.
    let y = y + 1;

    // This indicates an inner scope
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    //Shadowing allow to change the type of value that is stored on a variable.
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The were {spaces} spaces on the \"spaces\" string.");
}
