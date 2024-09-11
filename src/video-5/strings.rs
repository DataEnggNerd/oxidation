/// module to understand strings and string based operations

fn main(){
    let string_1 = String::from("I like dogs!");
    let string_2: String = string_1.replace("dog", "cat");

    println!("Truth about me: {}", string_1);
    println!("Lie about me: {}", string_2);

    // &' denotes the lifetime annotation 
    // this static variable gets added as a read only variable
    let static_string: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Static string: {}", static_string);

    println!("Lets print words in the above sentence reversed");
    for word in static_string.split_whitespace().rev(){
        println!(">> {}", word);
    }

    // converting the sentence into vector of unique characters in the string
    let mut chars: Vec<char> = static_string.chars().collect();
    chars.sort();
    chars.dedup();

    // add the unique characters to a new string comma seperated
    let mut string = String::new();
    for c in chars{
        string.push(c);
        string.push_str(", ");
    }

    // trimming the `, ` added in previous looping
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_string: &str = string.trim_matches(chars_to_trim);
    println!("Before trimming: {}", string);
    println!("After trimming: {}", trimmed_string);
}