
/// main method to declare all variables
fn main(){
    let x = 1;
    println!("x: {}", x);
    // shadowing the value above
    let x = "i";
    println!("x: {}", x);

    // initalize, declare
    let something;
    let x = 5;
    something = x*5;
    println!("x, something, {}, {}", x, something);

    // mutatbility
    let mut y = 2;
    y = y*2+x;
    dbg!(y);

    // constants
    const ANYTHING: i64 = 5;
    y*=ANYTHING;
    dbg!(y);
}