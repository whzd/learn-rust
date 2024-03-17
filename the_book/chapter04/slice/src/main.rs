fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn better_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    // first word without slices
    let s = String::from("Hello world");
    let word = first_word(&s);
    let hello: &str = &s[0..word];
    println!("{hello}");

    // slices
    let s2 = String::from("hello");
    let start = &s2[0..2];
    let start2 = &s2[..2];
    assert_eq!(start, start2);

    let len = s2.len();

    let end = &s2[3..len];
    let end2 = &s2[3..];
    assert_eq!(end, end2);
    let whole = &s2[0..len];
    let whole2 = &s2[..];
    assert_eq!(whole, whole2);

    // first word with slices
    let s3 = String::from("Hello world");
    let word2 = better_first_word(&s3);
    println!("{word2}");

    // other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
