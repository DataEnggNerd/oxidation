//! Module docstring - Structure of functions

/// function docstring
fn sum(x: i64, y: i64) -> i64{
    /* Multi line comment
    showing no return statement
    needed */
    x+y
}

fn main(){
    let z = sum(4, 3);
    println!("Result ==> {}", z);
    let result_text = format!("Result text with format ==> {w}", w=z);
    println!("{}", result_text)
}