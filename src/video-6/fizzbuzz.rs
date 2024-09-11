/// module to show multiple statement capability of rust closures

fn main(){
    let fizz_buzz_closure = |x: i32|{
        if x % 3 == 0{
            println!("Fizz => {}", x);
        } else if x % 5 == 0{
            println!("Buzz => {}", x);
        } else if x % 15 == 0 {
            println!("Fizzbuzz => {}", x);
        } else {
            println!("{}", x);
        }
    };

    for i in 0..16{
        fizz_buzz_closure(i);
    }
}