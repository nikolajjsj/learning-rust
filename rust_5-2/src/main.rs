// Program that calculates the area of a rectangle
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    println!("rect1 is {:?}", rect1);
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
