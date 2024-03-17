#[allow(unused_variables)]
fn main() {
    // Initializing an empty vector
    let v0: Vec<i32> = Vec::new();
    // Initilizing a vector with value (type infer)
    let v1 = vec![1, 2, 3];

    // Add elements to a vector and infers the type
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading values from a vector
    // Using the index
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    // Using the 'get' method
    let fifth: Option<&i32> = v.get(4);
    match fifth {
        Some(fifth) => println!("The fifth element is {fifth}"),
        None => println!("There is no fifth element."),
    }

    // Iterating over the values of a vector
    let new_vec = vec![100, 32, 57];
    for n_ref in &new_vec {
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }
    // Iterating over a mutable references
    let mut vecz = vec![100, 32, 57];
    for n_ref in &mut vecz {
        *n_ref += 50;
        println!("{}", *n_ref);
    }

    // Vector of enums example
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
