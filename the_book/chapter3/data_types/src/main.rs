fn main() {
    // Scalar Types

    // Integer
    let _a: i8 = -1;
    let _au: u8 = 1;
    let _b: i16 = -1;
    let _bu: u16 = 1;
    let _c: i32 = -1;
    let _cu: u32 = 1;
    let _d: i64 = -1;
    let _du: u64 = 1;
    let _e: i128 = -1;
    let _eu: u128 = 1;
    let _f: isize = -1;
    let _fu: usize = 1;

    // Integer literals
    let _dec = 98_222;
    let _hex = 0xff;
    let _oct = 0o77;
    let _bin = 0b1111_0000;
    let _byt = b'A';

    // Floating-point
    let _g: f64 = 2.0; // double precision float
    let _h: f32 = 3.0; // single precision float

    // Numeric Operations
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _wquotient = 56.7 / 32.2;
    let _truncated = -5 / 3;
    let _remainder = 43 % 5;

    // Boolean
    let _t: bool = true;
    let _f: bool = false;

    // Char
    // In Rust a char type is four bytes
    // and represents a Unicode Scalar Value
    let _z: char = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Compound Types

    // Tuple
    let tup: (i32, f64, u8) = (-500, 6.4, 1);
    // Tuple destructing
    let (_, y, _) = tup;
    println!("The value of x is: {}", tup.0);
    println!("The value of y is: {y}");
    println!("The value of z is: {}", tup.2);

    // Array
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("The second month of the year is: {}", months[1]);
    let arr: [i32; 5] = [3; 5];
    println!("The content of arr is: {:?}", arr);
}
