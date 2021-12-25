fn main() {
    // Using panic to do a unrecoverable crash
    panic!("crash and burn");

    // Another example of a panic
    let v = vec![1, 2, 3];
    v[99];
}
