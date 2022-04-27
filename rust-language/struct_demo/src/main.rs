fn main() {
    let rect1 = Rectangle {
        width: 1,
        height: 50,
    };

    println!("Area: {}", rect1.area());
    if rect1.width() && rect1.height() {
        println!("Width: {}, Height: {}", rect1.width, rect1.height);
    }

    let rect2 = Rectangle::square(65);
    println!(
        "Rect2.width: {}, Rect2.height: {}",
        rect2.width, rect2.height
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn height(&self) -> bool {
        self.height > 0
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
