#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// implementation blocks can be seperated or like this:
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.height > rectangle.height && self.width > rectangle.width
    }

    // Associated function that does not need the 'self' instance
    // This function can be called, like: Rectangle::square(3)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    let square1 = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
