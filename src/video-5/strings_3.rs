/// memory allocation and consumption of ascii and unicode strings in rust

fn main(){
    let ascii = String::from("Hello");
    println!(
        "{}, Length -> {}, Characters -> {}, Memory size -> {}",
        ascii,
        ascii.len(),
        ascii.chars().count(),
        std::mem::size_of::<String>() + ascii.len()
    );
    println!("{:?}", ascii.as_bytes());
    println!("After first l: {:?}", &ascii[3..]);

    let unicode_word = String::from("Héllö");
    println!(
        "{}, Length -> {}, Characters -> {}, Memory size -> {}",
        unicode_word,
        unicode_word.len(),
        unicode_word.chars().count(),
        std::mem::size_of::<String>() + unicode_word.len()
    );
    println!("{:?}", unicode_word.as_bytes());
    println!("After first l: {:?}", &unicode_word[2..])
}