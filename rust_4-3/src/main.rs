fn main() {
    let test_string = String::from("This is a string");
    let word_length = first_word(&test_string);
    println!(
        "The length of the first word in '{}' is {}",
        test_string, word_length
    );
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Slice version of first_word(s: &String)
fn slice_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
