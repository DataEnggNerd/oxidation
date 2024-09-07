// function to find type of the element/variable
fn type_of<T>(_: &T){
    println!("{}", std::any::type_name::<T>());
}

/// vector data type implementation
fn main() {
    let mut xs = vec![1, 3, 5, 7_u32];
    println!("First vector => {:?}", xs);

    // access first element of the vector
    println!("First element of the first vector => {}", xs[0]);

    // print current length of the vector
    println!("Length of the vector => {}", xs.len());

    // add one more element to the last index
    xs.push(9_u32);

    // print current length of the vector
    println!("Length of the vector after adding one element => {}", xs.len());

    // Pop last element of the vector
    println!("Element popped out of the vector => {:?}", xs.pop());

    // print current length of the vector
    println!("Length of the vector after popping last element => {}", xs.len());

    // iterating the vector
    for x in xs.iter(){
        println!(">> {}", x);
    }

    // iterating and enumerating the vector
    for (i, x) in xs.iter().enumerate(){
        println!("Value {} is available in {} position", x, i);
        type_of(&x);
    }

    // creating vector from a range of numbers
    let collected_vector: Vec<u32> = (0..10).collect();
    println!("Collected vector => {:?}", collected_vector);

    // creating vector by mofiying another mutable vector
    let xss : Vec<u32> = xs.iter().map(|x| x*3).collect();
    println!("Modified xs vector => {:?}", xss);
}