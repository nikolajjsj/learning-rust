fn main() {
    // Data Types
    let _guess: u32 = "42".parse().expect("Not a number!");

    // Scalar Types
    // Integer Types
    let _eight_bit: u8 = 8; // 8-bit
    let _sixteen_bit: u16 = 16; // 16-bit
    let _thirtytwo_bit: u32 = 32; // 32-bit
    let _sixtyfour_bit: u64 = 64; // 64-bit
    let _onehundredtwentyeight_bit: u128 = 16; // 128-bit
    let _arch: usize = 16; // arch

    let _decimal = 98_222;
    let _hex = 0xff;
    let _octal = 0o77;
    let _binary = 0b1111_0000;
    let _byte_u8_only = b'A';

    // Floating-Point Types
    let _x = 2.0;
    let _y: f32 = 3.0;

    // Numeric Operations
    let _sum = 5 + 10; // addition

    let _difference = 95.5 - 4.3; // subtraction

    let _product = 4 * 30; // multiplication

    let _quotient = 56.7 / 32.2; // division
    let _floored = 2 / 3; // Results in 0

    let _remainder = 43 % 5; // remainder

    // The Boolean Type
    let _t = true;
    let _f: bool = false;

    // The Character Type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Compound Types
    // The Tuple Type
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let _tup = (500, 6.4, 1);
    let (_x, _y, _z) = _tup;
    println!("The value of _y if: {}", _y);

    let _five_hundred = _tup.0;
    let _six_point_four = _tup.1;
    let _one = _tup.2;

    // The Array Type
    let _a = [1, 2, 3, 4, 5];
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5]; // = [3, 3, 3, 3, 3];

    let _first_element = _a[0];
    let _second_element = _a[1];
}
