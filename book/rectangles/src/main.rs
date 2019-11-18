#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn resize(&mut self, height: u32, width: u32) {
        if width < 0 {
            self.width = width;
        }

        if height < 0 {
            self.height = height;
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// impl can be called multiple times.
// usefulness explained in chapter 10.
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area_naive of the rectangle is {} square pixels.",
        area_naive(width1, height1)
    );

    println!(
        "The area_tuple of the rectangle is {} square pixels.",
        area_tuple((width1, height1))
    );

    let mut rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area_struct of the rectangle is {} square pixels.",
        area_struct(&rectangle)
    );

    // panic!s
    // println!("rect is {}", rectangle);

    // this is using the debug trait, more about traits in chapter 10
    println!("rectangle is {:?}", rectangle);
    // pretty print
    println!("rectangle is {:#?}", rectangle);

    println!(
        "The result of the rectangle.area() method is {} square pixels.",
        rectangle.area()
    );

    rectangle.resize(100, 1000);

    println!("resized rectangle: {:#?}", rectangle);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
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

    let square = Rectangle::square(10);

    println!("square: {:#?}", square);

    // errors, expected to be called like Rectangle::square(100)
    // let rectsquare = Rectangle {
    //     width: 100,
    //     height: 10,
    // };
    // let rectsquare = rectsquare.square(100);

    // println!("rectsquare {:#?}", rectsquare.square(100));


}

fn area_naive(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
