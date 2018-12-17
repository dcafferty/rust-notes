#[allow(dead_code)]

use std::mem;
use std::fmt;

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

impl fmt::Display for Line {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "start {}, end {}", self.start, self.end)
    }
}

impl fmt::Display for Point {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}:{}", self.x, self.y)
    }
}

pub fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 10.0, y: 11.0 };
    println!("point p2 is at ({}, {})", p2.x, p2.y);

    let myline = Line { start: p, end: p2 };
    println!("myline is at ({})", myline);
}

pub fn stack_and_heap() {
    let a = Box::new(10);
    println!("a box = {}", a);
    println!("*a box = {}", *a);

    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("p3.x = {}", p3.x);
}