#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area_method(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn new_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn area_par(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let scale: u32 = 3;

    let rect_struct = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect_tup = (30, 50);

    println!(
        "With two arguments: The area of the rectangle is {} square pixels.",
        area_par(width1, height1)
    );
    println!(
        "With a tuple: The area of the rectangle is {} square pixels.",
        area_tup(rect_tup)
    );
    println!(
        "With a struct: The area of the rectangle is {} square pixels.",
        area_struct(&rect_struct)
    );
    println!("Let's try to print this motherfucker: {:#?}", rect_struct);
    dbg!(&rect_struct);

    println!(
        "
        With a method: {}",
        rect_struct.area_method()
    );
    let small_rectangle = Rectangle {
        width: 1,
        height: 1,
    };
    println!(
        "Small rectangle can fit inside the other: {}",
        rect_struct.can_hold(&small_rectangle)
    );
    println!(
        "The other way around: {}",
        small_rectangle.can_hold(&rect_struct)
    );

    let square = Rectangle::new_square(5);
    println!("My magnificent square: {:#?}", square);
}
