#[allow(unused_variables)]
fn main() {
    // Initializing an empty string
    let s = String::new();

    // Adding data tot the string
    let data = "initial content";
    let s = data.to_string();

    let s = "initial content".to_string();

    let s = String::from("initial content");

    // Updating the value of a string
    let mut s1 = String::from("foo");
    s1.push_str("bar");
    s1.push('!');

    // String concatenation
    let s2 = String::from("Hello, ");
    let s3 = String::from("world!");
    let s4 = s1 + &s2;
    // Concatenation of multiple strings
    let st1 = String::from("tic");
    let st2 = String::from("tac");
    let st3 = String::from("toe");
    // The '+' approach
    let st4 = st1 + "-" + &st2 + "-" + &st3;
    // The 'format!' approach
    let st1 = String::from("tic");
    let st4 = format!("{st1}-{st2}-{st3}");

    // Iterating over strings
    for c in "abcd".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}")
    }
}
