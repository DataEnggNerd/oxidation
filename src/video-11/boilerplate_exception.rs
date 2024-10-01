use std::error;
use std::fmt;


// creating a type alias to handle errors
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item in double")
    }
}

impl error::Error for EmptyVec {}

fn double_first_element(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result{
        Ok(n) => println!("The first doubled integer in {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main(){
    let numbers = vec!["1", "4", "3"];
    let empty_vector = vec![];
    let strings = vec!["one", "4", "3"];

    print(double_first_element(numbers));
    print(double_first_element(empty_vector));
    print(double_first_element(strings));
}