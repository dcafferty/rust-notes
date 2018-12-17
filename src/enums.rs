#[allow(dead_code)]

use std::mem;
use std::fmt;

enum Color {
    Red,
    Green,
    Blue,
    Pink,
    RgbColor(u8, u8, u8), //tuple
    Cmyk {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // struct
}

pub fn enums() {
    // let c: Color = Color::Red;
    // let c: Color = Color::RgbColor(0, 0, 0);
    let c: Color = Color::Cmyk {
        cyan: 78,
        magenta: 45,
        yellow: 123,
        black: 255,
    };

    match c {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::Cmyk {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("black other"),
        _ => println!("some other color"),
    }
}

pub fn option() {
    // Option<T>
    let x = 3.0;
    let y = 2.0;

    let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };

    println!("{:?}", result);
}