/// module for introduction of struct

// to print a struct, #[derive(Debug)] is necessary. Otherwise we can implement a `trait` for displaying
#[derive(Debug)]
struct SampleStruct{
    attribute_1: i32,
    attribute_2: String
}

fn main() {
    let a = SampleStruct{attribute_1: 2, attribute_2:"name".to_string()};
    println!("SampleStruct attribute 1: {}", a.attribute_1);
    println!("SampleStruct attribute 2: {}", a.attribute_2);

    // demonstration on updating a new struct with declared struct
    let b = SampleStruct{ attribute_2: "new instance".to_string(), ..a}; // not `..a`, means rest of the declaration will be considered from `a`
    println!("Printing entire struct: {:?}", b);
}