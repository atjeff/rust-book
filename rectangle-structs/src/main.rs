// This is a trait, namely the Debug trait. It allows us to use the println!("{:#?}", &rect1);
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Structs can have multiple implementations
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn has_width(&self) -> bool {
        self.width > 0
    }

    fn has_height(&self) -> bool {
        self.height > 0
    }

    // This is a method on rectangle that also takes an arg
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // This is a static method. Rectangle::square()
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Part 1
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1: {:?}", &rect1);
    println!("{:#?}", &rect1);

    if rect1.has_width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    if rect1.has_height() {
        println!("The rectangle has a nonzero height; it is {}", rect1.height);
    }

    // Part 2
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Part 3
    let square_rectangle = Rectangle::square(2);

    println!("Width and height: {:#?}", square_rectangle)
}
