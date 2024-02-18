fn main() {
    let cardinal_numbers = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for i in 0..12 {
        println!(
            "On the {} day of Christmas,\nmy true love game to me",
            cardinal_numbers[i]
        );
        for j in (0..i + 1).rev() {
            if j == 0 {
                if i == 11 {
                    println!("And {}!", gifts[j].to_lowercase());
                } else if i > 0 {
                    println!("And {}.", gifts[j].to_lowercase());
                } else {
                    println!("{}.", gifts[j]);
                }
            } else {
                println!("{},", gifts[j]);
            }
        }
        println!()
    }
}
