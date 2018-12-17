#[allow(dead_code)]

mod sh;
mod controls;
mod enums;
mod optionmod;
mod arrays;
mod vectors;

use std::mem;

const MEANING_OF_CAFF: u8 = 64; // no fixed address

static MEANING_VAR: u8 = 54;

fn simple_assignment() {
    let mut a: u16 = 123;
    println!("a = {}", a);

    a = 456;

    println!("a = {}", a);

    let mut c = 123456789;

    println!("c = {}", c);
    println!("c = {}, size = {}", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);
}
fn fundamental_data_types() {
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);

    println!(
        "z= {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d: char = 'x';
    let size_of_d = mem::size_of_val(&d);
    println!("d= {}, takes up {} bytes", d, size_of_d);

    let e = 2.5;
    println!("e= {}, takes up {} bytes", e, mem::size_of_val(&e));

    let g = false;
    println!("e= {}, takes up {} bytes", g, mem::size_of_val(&g));

    let f = 4 > 0;
    println!("f= {}, takes up {} bytes", f, mem::size_of_val(&f));
}

fn operators() {
    let mut a = 2 + 3 * 4;
    println!("a = {}", a);
    a = a - 1;
    a -= 2;

    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);
}

fn bitoperators() {
    let c = 4 | 1;
    println!("4 | 1 = {}", c);
    let d = 7 | 1;
    println!("7 | 1 = {}", d);
    let f = 4 & 1;
    println!("4 & 1 = {}", f);
    let g = 7 & 1;
    println!("7 & 1 = {}", g);
    let h = 4 ^ 1;
    println!("4 ^ 1 = {}", h);
    let i = 7 ^ 1;
    println!("7 ^ 1 = {}", i);

    let two_to_4 = 1 << 3;
    println!("2 << 3 = {}", two_to_4);
}

fn logicaloperators() {
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("pi_less_4 = {}", pi_less_4);

    let t = 4 == 5;
    println!("4 == 5 = {}", t);
}

fn scope_and_shadowing() {
    let a = 123;

    {
        let b = 456;
        println!("b = {}", b);

        let a = 789;
        println!("a inner = {}", a);
    }
    println!("a outer = {}", a);

}

fn main() {
    // println!("Hello, world!");
    // println!("MEANING_OF_CAFF = {}", MEANING_OF_CAFF);
    // println!("MEANING_VAR = {}", MEANING_VAR);
    // simple_assignment();
    // fundamental_data_types();
    // operators();
    // bitoperators();
    // logicaloperators();
    // scope_and_shadowing();
    // sh::stack_and_heap();
    // controls::if_statement();
    // controls::while_and_loop();
    // controls::for_loop();
    // controls::match_statement();
    // sh::structures();
    // enums::enums();
    // optionmod::option();
    // arrays::arrays();
    vectors::vectors();
}