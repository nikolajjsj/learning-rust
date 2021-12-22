// Example struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // Example instance of the User struct
    let mut _user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Setting a specific value from a struct
    _user1.email = String::from("anotheremail@example.com");

    // Creating a new user with values from another
    let _user2 = User {
        email: String::from("another@example.com"),
        .._user1
    };

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Structs
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

// Function that returns a new User
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
