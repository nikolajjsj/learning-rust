fn main() {
    let v: Vec<i32> = Vec::new();
    // of with a macro:
    let mut v = vec![1, 2, 3];

    v.push(5);
    v.push(7);

    let third_value: &i32 = &v[2];
    println!("The third element is {}", third_value);
}
