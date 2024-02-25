#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    _Penny,
    _Nickel,
    _Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::_Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
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

fn add_fancy_hat() {
    println!("Fancy hat added!")
}

fn remove_fancy_hat() {
    println!("Fancy hat removed!")
}

fn main() {
    let _coin_value = value_in_cents(Coin::Quarter(UsState::Alaska));
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    //Catch all example
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    //match ownership
    let opt: Option<String> = Some(String::from("Hello world"));
    // opt has to be a reference so that the first arm can use the value
    // and still be used later
    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };
    println!("{:?}", opt);

    // Match approach
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // If let approach
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // If let with else
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("{count}");
}
