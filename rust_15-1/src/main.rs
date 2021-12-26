// A pointer is a general concept for a variable that contains an address in memory.
fn main() {
    // Using a Box<T>
    let b = Box::new(5);
    println!("b = {}", b);
}
