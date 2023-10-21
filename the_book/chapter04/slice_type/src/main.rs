fn main() {
    let mut s = String::from("hello world");

    let _word = first_word_try(&s);
    s.clear();
    // println!("{}", word);    // This will fail

    let s1 = String::from("hi there");
    println!("{}", &s1[0..2]);
    println!("{}", &s1[..2]);
    let len = s1.len();
    println!("{}", &s1[3..len]);
    println!("{}", &s1[3..]);
    println!("{}", &s1[0..len]);
    println!("{}", &s1[..]);

    let s1 = String::from("howdy partner!");

    println!("The first word is: {}", first_word(&s1));

    let test = "yesss sir!";

    println!("The first word is: {}", first_word(test));

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word_try(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes= s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
