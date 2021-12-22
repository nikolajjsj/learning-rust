fn main() {
    // Ownership Rules
    //  - Each value in Rust has a variable that's called its "owner"
    //  - There can only be one owner at a time
    //  - When the owner goes out of scope, the value will be dropped

    {
        let _s = "hello";
        // Do stuff with s
    } // Now that the scope is over the value for s will be dropped

    let _s1 = String::from("hello");
    let _s2 = _s1; // _s1 value borrowed

    // Cant use the _s1 variable now, because it is borrowed
    println!("{}, world!", _s2);
}

fn _main2() {
    let s = String::from("hello"); // s comes into scope

    _takes_ownership(s); // s's value moves into the function...
                         // ... and so is no longer valid here

    let x = 5; // x comes into scope

    _makes_copy(x); // x would move into the function,
                    // but i32 is Copy, so it's okay to still
                    // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn _takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn _makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
