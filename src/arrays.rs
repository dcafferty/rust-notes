#[allow(dead_code)]

use std::mem;

pub fn arrays() {
    // let mut a = [1, 2, 3, 4, 5];
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;

    println!("a has {} elements, first is {}", a.len(), a[0]);

    println!("{:?}", a);

    if a != [321, 2, 3, 4, 5] {
        println!("doesnt match")
    }
    if a != [2, 3, 4, 5, 321] {
        println!("doesnt match")
    }

    let b = [1; 10]; //b.len() == 10

    for i in 0..b.len() {
        println!("{}", b[i]);
    }
    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];

    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            // if i == j {
            println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            // }
        }
    }
}