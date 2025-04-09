use std::fmt::{Display, Formatter};

#[derive(Debug)] // allows to print the structure in debug mode (ie. to use {:?})
struct Rectangle {
    x : f32,
    y : f32
}

trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;

    fn describe(&self) {
        println!("I'm a general shape");
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.x * self.y
    }
 
    fn perimeter(&self) -> f32 {
        2f32 * (self.x + self.y)
    }

    fn describe(&self) {
        println!("I'm a rectangle");
    }
}

impl Rectangle {
    fn new_square(x: f32) -> Rectangle {
        Rectangle{x, y: x}
    }

    fn area(&self) -> f32 {
        self.x * self.y
    }

    fn scale(&mut self, factor:f32) {
        self.x = self.x * factor;
        self.y = self.y * factor;
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle[x={},y={}]", self.x, self.y)?;
        Ok(())
    }
}


fn main() {
    let r1 = Rectangle{x: 1.0, y: 2.0}; 
    println!("{:?}", r1);    
    println!("x: {}, y: {}", r1.x, r1.y);

    let r2 = Rectangle{x : 5.0, ..r1};
    println!("{:?}", r2);
    println!("x: {}, y: {}", r2.x, r2.y);

    let r3 = Rectangle{x : 5.0, y : 9.0};
//    r3.x = 6.0; BŁĄD BO NIE mutowalne
    println!("[{}, {}]", r3.x, r3.y);

    let mut r4 = Rectangle{x : 5.0, y : 9.0};
    println!("[{}, {}]", r4.x, r4.y);
    r4.x = 6.0;
    r4.y = 7.0;
    println!("[{}, {}]", r4.x, r4.y);

    println!("Area of {:?} is {}", r4, r4.area());

    r4.scale(2.0);
    println!("Area of {:?} is {}", r4, r4.area());

    let square = Rectangle::new_square(5.0);
    println!("Square: {:?}", square);
    println!("Area of square {:?} is {}", square, square.area());

    square.describe();
    println!("{}", square);
}