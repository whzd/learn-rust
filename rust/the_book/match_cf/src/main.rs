#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin1 = Coin::Penny;
    let coin2 = Coin::Dime;
    let coin3 = Coin::Nickel;
    let coin4 = Coin::Quarter;
    println!("{}", value_in_cents(coin1));
    println!("{}", value_in_cents(coin2));
    println!("{}", value_in_cents(coin3));
    println!("{}", value_in_cents(coin4(UsState::Alabama)));
    println!("{}", value_in_cents(coin4(UsState::Alaska)));

    let five = Some(5);
    println!("{:?}", five);
    let six = plus_one(five);
    println!("{:?}", six);
    let none = plus_one(None);
    println!("{:?}", none);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let config_max2 = Some(5u8);
    if let Some(max) = config_max2 {
        println!("The maximum2 is configured to be {}", max);
    }

    let mut count = 0;
    match coin4(UsState::Alaska) {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("{}", count);

    let mut count2 = 0;
    if let Coin::Quarter(state) = coin4(UsState::Alabama) {
        println!("State quarter from {:?}!", state);
    } else {
        count2 += 1;
    }
    println!("{}", count2);
}

