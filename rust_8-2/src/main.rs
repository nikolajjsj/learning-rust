fn main() {
    let mut s = String::new();

    let data = "Initial contents";

    let s = data.to_string();
    // also works directly
    let s = "Initial contents".to_string();

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // Indexing into Strings
    let s1 = String::from("hello");
    let h = s1[0];
}
