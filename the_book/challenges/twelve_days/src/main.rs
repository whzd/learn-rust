fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let gifts = ["A partridge in a pear tree", "Two turtle doves, and", "Three french hens", "Four calling birds",  "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    for i in 0..12 {
        println!("On the {} day of Christmas,", days[i]);
        println!("My true love sent to me");
        for i in (0..i+1).rev(){
            println!("{}", gifts[i]);
        }
        println!()
    }
}
