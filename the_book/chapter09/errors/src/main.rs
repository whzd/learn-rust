use core::panic;
use std::fs;
use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

#[allow(dead_code)]
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Less verbose alternatives
#[allow(dead_code)]
fn read_username_from_file2() -> Result<String, io::Error> {
    // Using the ? operand but maitaining the logic
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
//
#[allow(dead_code)]
fn read_username_from_file3() -> Result<String, io::Error> {
    // Chaining calls
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
//
#[allow(dead_code)]
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Using ? operand on a Option return
#[allow(dead_code)]
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

#[allow(unused_variables)]
fn main() {
    // File open with creation on error
    let greeting_file_result = File::open("hello.txt");
    let greeting_file_result = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // Less verbose options
    let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file2 =
        File::open("hello.txt").expect("hello.txt should be included in this project");
    /*
    // Panic examples
    //// Automatic
    let v = vec![1, 2, 3];
    v[99];
    //// Forced
    panic!("crash and burn");
    */
}
