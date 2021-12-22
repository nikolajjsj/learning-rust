fn main() {
    // References and Borrowing
    // Use references to borrow a variables value
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Here a reference (&) is used to borrow the value of s1
    println!("The length of '{}' is {}.", s1, len);

    // Because we pass the value as a reference, -
    // it is not possible to modify it
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
