#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let coin = Coin::Quarter;

    println!(
        "The value of '{:?}' is {} in cents.",
        coin,
        value_in_cents(&coin)
    )
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn other_match() {
    let dice_roll = 9;
    match dice_roll {
        3 => println!("Is a 3"),
        7 => println!("Is a 7"),
        _ => println!("Not a 3 or 7..."),
    }
}
