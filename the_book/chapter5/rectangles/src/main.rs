#[derive(Clone, Copy, Debug)]
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
        Self {
            width: size,
            height: size,
        }
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn better_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn even_better_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    // Not using data structures
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Using tuples
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        better_area(rect1)
    );

    // Using struct
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rectangle is {:#?}", rectangle);

    println!(
        "The area of the rectangle is {} square pixels.",
        even_better_area(&rectangle)
    );

    // debug example
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    // Using methods
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );

    if rectangle.width() {
        println!(
            "The rectangle has a nonzero width; it is {}",
            rectangle.width
        );
    }

    // Testing can_hold method
    let rec2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rec3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rectangle hold rec2? {}", rectangle.can_hold(&rec2));
    println!("Can rectangle hold rec3? {}", rectangle.can_hold(&rec3));

    // Associated Functions
    let _sq = Rectangle::square(3);

    let r = Rectangle {
        width: 1,
        height: 2,
    };
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);

    // Allow copy, clone
    let mut rec = Rectangle {
        width: 0,
        height: 1,
    };
    let other_rec = Rectangle {
        width: 1,
        height: 0,
    };
    rec.set_to_max(other_rec);
}
