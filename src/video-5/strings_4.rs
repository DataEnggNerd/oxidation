/// module to demonstrate writing multi line strings

fn main(){
    let multi_line_string_1 = "this
    is an accepted 
multi line
        string in Rust
    ".to_string();
    println!("Multiline string 1 => {}", multi_line_string_1);

    let multi_line_string_2 = "this is \"\" also \n an accepted 
    \n multiline string";
    println!("{}", multi_line_string_2);

    // note single quote for single characters. 
    // Rust panicks on using single quote for more than one character
    let single_char = 'c';
    let multichars = "ca";
    println!("Single character => {}", single_char);
    println!("Multiple character => {}", multichars);
}