/// module to demonstrate how to implement default values in `struct`
/// 1. implement a `trait` which will create a default value
/// 2. Use `std::default::Default` to implement for "all" attributes

// implementation of default using `std::default::Default` 
use std::default::Default;

#[derive(Debug)]
struct A{
    x: i32,
    y: i32,
    z: i32
}

impl Default for A{
    fn default() -> Self{
        // note that default has to be applied for all attributes
        // otherwise compiler will throw error of `missing fields`
        Self{x: 0, y: 1, z: 0}
    }
}

impl A{
    fn new(x: i32, y:i32, z:Option<i32>) -> Self{
        match z {
            Some(z) => Self{x, y, z},
            None => Self{x, y, z:0}
        }
    }
}

fn main() {
    let a = A {x:1, y:2, ..A::default()}; // using implemented `default`
    let b = A::new(5, 10, None);
    println!("{:?}", a);
    println!("{:?}", b);
}
