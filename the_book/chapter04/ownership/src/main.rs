fn main() {
    //Referencing and dereferencing
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);
    let x_abs2 = x.abs();
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);
    let r_abs2 = r.abs();
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s);
    let s_len2 = s.len();
    assert_eq!(s_len1, s_len2);

    let mut v: Vec<i32> = vec![1, 2, 3];
    //v.push(4);
    let num: &mut i32 = &mut v[2];
    let num2: &i32 = &*num;
    println!("{} {}", *num, *num2);

    //copying vs. moving out of a collection
    // using an immutable reference
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");
    // using clone to take ownership
    let v2: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v2[0].clone();
    s.push('!');
    println!("{s}");
    // use a method to move the string of of the vector
    let mut v3: Vec<String> = vec![String::from("Hello world")];
    let mut s2: String = v3.remove(0);
    s2.push('!');
    println!("{s2}");
    assert!(v3.len() == 0);
}
