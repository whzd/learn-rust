fn main() {
    // let s = String::from("hello");   // Default variable can't be mutable
    let mut s = String::from("hello");
    println!("{}", s);

    // change(&s);  // References are imutable by default
    change(&mut s);
    println!("{}", s);

    let r1 = &mut s;
    println!("{}", r1);

    // You can only 1 mutable reference to a value
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    
    let mut s2 = String::from("hi there!");
    
    {
        let r3 = &mut s2;
        println!("{}", r3);
    }
    let r4 = &mut s2;
    println!("{}", r4);

    let mut s3 = String::from("howdy partner!");

    let r5 = &s3;
    let r6 = &s3;
    // A mutable reference to a value cannot exist where an immutable one is already in place
    // let r7 = &mut s3;
    // println!("{}, {}, and {}", r5, r6, r7);
    println!("{} and {}", r5, r6);

    let r7 = &mut s3;
    println!("{}", r7);

}

// fn change(some_string: &String) { // Since references are imutable you can't change them
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
