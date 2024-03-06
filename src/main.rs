use core::fmt;
use std::{fmt::write, ops::Add};

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self){
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait Wizzard {
    fn fly(&self);
}

trait Pilot {
    fn fly(&self);
}

struct  Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizzard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human{
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
// main function
fn main() {
    assert_eq!(
        Point {x: 1, y: 0} + Point {x: 2, y: 3}, Point{ x: 3, y: 3}
    );

    let person: Human = Human;

    Pilot::fly(&person);
    Wizzard::fly(&person);
    person.fly();

    let point: Point = Point {x: 2, y: 8};
    point.outline_print();
}
