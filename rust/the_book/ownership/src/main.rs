fn main() {
    let s1 = String::from("hellow"); // This creates a ptr a len and a capacity on the stack
                                     // The content of the string be stored on the heap
    // let s2 = s1; // This would move 
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("test");

    takes_ownership(s);

    // println!("{}", s); // This would throw an error since the s was moved to the function

    let x = 5;

    makes_copy(x);


    let s3 = String::from("another_test");

    let (s4, len) = calculate_length(s3);

    println!("The length of '{}' is {}.", s4, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
