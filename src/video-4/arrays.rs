/// demo of arrays
fn main() {
    let xs = [1,2,3,4,5];

    // this follows a pattern of [type; size_of_array]
    let xl: [i32; 500] = [0; 500];
    // below line means, 0 of type u64 is being repeated 500 times
    let _xl = [0_u64, 500];

    println!("Length of the xs sized array => {}", xs.len());
    println!("Length of the xl sized array => {}", xl.len());

    println!("Accessing first element of the xs array => {}", xs[0]);
    println!("Accessing second element of the xs array => {}", xs[1]);

    // Note the `&` while referring the array as it is important to borrow the memory
    // Rust doesn't allow to directly access the elements while slicing
    println!("Borrowing a section of the array by slicing => {:?}", &xs[1..3]);
}