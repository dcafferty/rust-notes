#[allow(dead_code)]

use std::mem;

pub fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(44);

    println!("a = {:?}", a);

    println!("a[0] = {}", a[0]);

    // //wont work due to i not u, 32 not always true 64-bit machine
    // let index:i32 = 0;
    // println!("a[index] = {}", a[index]);

    let index:usize = 0;
    println!("a[index] = {}", a[index]);

    // GET elements from vertex
    // match a.get(2) {
    match a.get(6) {
      Some(x) => println!("a[6] = {}", x),
      None => println!("error, no such element")
    }
    
    for x in &a { println!("{}", x);}

    a.push(77);
    println!("{:?}", a);

    let last_elem = a.pop();
    println!("last elem: {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop() {
      println!("{}", x);
    }
    
}