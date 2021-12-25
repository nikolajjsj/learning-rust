use std::net::IpAddr;

fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    // Creating custom types for validation
    loop {
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100");
            continue;
        }
    }
}
