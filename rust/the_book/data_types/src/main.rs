fn main() {
    let x = 2.0; // f64
    println!("The value of x is: {x}");
    let y: f32 = 3.0; // f32
    println!("The value of y is: {y}");

    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");
    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");
    // difvision
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");
    let truncated = -5 / 3;
    println!("The value of truncated is: {truncated}");
    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");

    let t = true;
    println!("The value of t is: {t}");
    let f: bool = false;
    println!("The value of f is: {f}");

    let c = 'z';
    println!("The value of c is: {c}");
    let z: char = 'ℤ';
    println!("The value of z is: {z}");
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);
    let tup_two = (500, 6.4, 1);
    let (q,w,e) = tup_two;
    println!("The value of q is: {q}");
    println!("The value of w is: {w}");
    println!("The value of e is: {e}");
    let five_hundred = tup.0;
    println!("The value of five_hundred is: {five_hundred}");
    let six_point_four = tup.1;
    println!("The value of six_point_four is: {six_point_four}");
    let one = tup.2;
    println!("The value of one is: {one}");

    let array = [1, 2, 3, 4, 5];
    println!("The value of array is: {:?}", array);
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("The value of months is: {:?}", months);
    let array_two: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of array_two is: {:?}", array_two);
    let array_three = [3; 5];
    println!("The value of array_three is: {:?}", array_three);
    let first = array[0];
    println!("The value of first is: {:?}", first);
    let second = array[1];
    println!("The value of second is: {:?}", second);

}
