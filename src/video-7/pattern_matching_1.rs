struct Dog {
    name: String,
}

struct Cat {
    age: u8,
}

enum Animals {
    Dog(Dog),
    Cat(Cat),
}

fn classify(animal: Animals){
    match animal {
        Animals::Dog(d) => println!("A dog named {}", d.name),
        Animals::Cat(c) => println!("A cat aged {} years", c.age),
    }
}

fn numbers(x: i32){
    match x{
        1 => println!("Uno!"),
        2 | 3 => println!("Dos or Tres!"),
        4..=i32::MAX => println!("Más de cuatro"),
        _ => println!("Cualquier cosa"),
    }
}

fn main(){
    classify(
        Animals::Dog(Dog{name: "Mr.Hulk".to_string(),})
    );
    classify(Animals::Cat(Cat{age: 3,}));
    for x in 0..10 { numbers(x) };
    numbers(-1);
}