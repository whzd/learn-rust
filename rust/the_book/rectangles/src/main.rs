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

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    let rect1 = (30, 50);

    println!("The area of the rectangle1 is {} square pixels.", area1(rect1));

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rect2 is {:#?}", rect2);
    dbg!(&rect2);

    println!("The area of the rectangle2 is {} square pixels.", area2(&rect2));

    println!("The area of the rectangle2 is {} square pixels.", rect2.area());
    println!("The rectangle2 has a nonzero width; It is {}, the width is {}.", rect2.width(), rect2.width);

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect4 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect5 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));
    println!("Can rect3 hold rect5? {}", rect3.can_hold(&rect5));

    let sq = Rectangle::square(3);
    dbg!(&sq);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
