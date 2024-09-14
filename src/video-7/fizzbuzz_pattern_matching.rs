/// rewriting fizzbuzz from previous module with match case

fn main(){
    // with if..else
    fn fizzbuzz_method(x: i32){
        if x % 3 == 0{
            println!("Fizz")
        } else if x % 5 == 0 {
            println!("Buzz")
        } else if x % 15 == 0 {
            println!("FizzBuzz")
        } else {
            println!(":thumbs down:!")
        }
    }

    let fizzbuzz_closure = |x: i32| match (x % 3, x % 5){
        (0, 0) => println!("FizzBuzz => {}", x), // if this is not on top compiler will emit `warning: unreachable pattern` as 15 matches for next two patterns as well
        (0, _) => println!("Fizz => {}", x),
        (_, 0) => println!("Buzz => {}", x),
        (_, _) => println!(":thumbs down:!"),
    };

    for i in 0..24 { fizzbuzz_closure(i) };
    for i in 0..10 { fizzbuzz_method(i)};
}