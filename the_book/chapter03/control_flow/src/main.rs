fn main() {
    // if Expressions
    let number = 3;
    // if number {} will not work since rust will
    // not automatically convert non-boolean to boolean.
    // If there are more than 1 else if block the code should
    // be refactored to use match
    if number < 5 {
        println!("The number was lower than 5.")
    } else if number > 5 {
        println!("The number was greater than 5.")
    } else {
        println!("The number was 5.")
    }

    let condition = true;
    let _new_number = if condition { 5 } else { 6 };
    // Values from each arm of the if must be the same type
    // so that the compiler can verify its validity at runtime
    // the following example would not work
    // let number = if condition { 5 } else { "six" };

    // loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while loop
    let mut countdown = 3;
    while countdown != 0 {
        println!("{countdown}");
        countdown -= 1;
    }
    println!("LIFTOFF!!!");

    // for loop
    let a = [10, 20, 30, 40, 50];
    // The safety and conciseness of for loops make them the best choice.
    for element in a {
        println!("The value is: {element}");
    }
}
