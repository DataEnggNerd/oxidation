/// reversing the given tuple 
fn reverse(sample_tuple: (&str, bool)) -> (bool, &str){
    // unpacking the elements of the tuple
    let (str_element, boolean_element) = sample_tuple;

    // returning the reversed tuple
    (boolean_element, str_element)
}

#[derive(Debug)]
struct Matrix(f32, f32, i8, u8);

fn main(){
    let many_typed_tuple = (1u8, 2u16, "a", true);

    // instead of tuple[n] from python, it is tuple.n  
    println!("First value in the tuple ==> {}", many_typed_tuple.0);
    println!("Second value in the tuple ==> {}", many_typed_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 1u8), (4u64, -1i8));
    println!("Tuple of tuples ==> {:?}", tuple_of_tuples);

    let pair = ("string", true);
    println!("Pair ==> {:?}", pair);
    println!("Reversed tuple ==> {:?}", reverse(pair));

    // tuple unpacking
    let tuple = (1, 3, 5.5, 10);
    let(a, b, c, d) = tuple;
    println!("Unpacked tuple => {:?}, {:?}, {:?}, {:?}", a, b, c, d);

    // creating a struct similar to tuple
    let matrix = Matrix(3.14, 6.0, -1, 2);
    println!("Matrix struct => {}, {}, {}, {}", matrix.0, matrix.1, matrix.2, matrix.3);

}