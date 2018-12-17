#[allow(dead_code)]

use std::mem;
use std::fmt;

pub fn option() {
    //Option<T>
    let x = 3.0;
    let y = 2.0;

    // Some(z) None
    let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };

    println!("result: {:?}", result);
    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        Node => println!("cannot divide {}/{}", x, y),
    }

    if let Some(z) = result {
        println!("z = {}", z);
    }
}