/// module to give examples of closures
/// similar to lambda function in Python

fn main(){
    let example_closure = |x| x; // similar to lambda x: x

    let s: String = example_closure(String::from("Hello"));
    /* 
    note `.to_string()`;
    Otherwise compiler throws error for lack of 
    type in the declaration.
    */
    let number: String = example_closure(5.to_string());

    let x = vec![1, 3, 5];

    println!("assetion vector => {:?}", &x);
    // move is used to move the memory of `x` into the closure
    // which means `x` can not be accessed anymore outside the closur
    // the closure has taken up the `x` ownership
    let equal_to_x = move |z| z == x;

    let y = vec![1, 3, 5];
    assert!(equal_to_x(y));

    let items: Vec<i32> = vec![1, 2, 3, 4, 5];
    let plus_one: Vec<_> = items.iter().map(|x| x +1).collect();
    let sum_all: i32 = items.iter().map(|x| x+1).sum();
    println!("{:?} {}", plus_one, sum_all);

    // closure takes two arguments and gives their product as result
    let two_args = |x:i32, y:i32| x-y;
    println!("{}", two_args(3, 2));
}