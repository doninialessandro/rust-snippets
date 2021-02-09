#![allow(dead_code)]
use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn function() {
    println!("--- STACK AND HEAP ---");
    let p1 = origin(); // (STACK)
    let p2 = Box::new(origin()); // point to some location (HEAP)

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));
    let p3 = *p2; // star lets you follow where the boxed value actually lives and assign it to variables.
    println!(
        "p3 takes up {} bytes, and its value is x: {}, y: {}",
        mem::size_of_val(&p3),
        p3.x,
        p3.y
    );
}
