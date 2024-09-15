/// module for implementation of HashMaps, `dict` in python
/// Note: There is no native implementation of hashmap in Rust - sab jughad! 

use std::collections::HashMap;

fn main() {
    let first_map: HashMap<&str, i32> = vec![("one", 1), ("two", 2)].into_iter().collect();
    println!("{:?}", first_map);

    let mut second_map = HashMap::new(); // notice `mut` here; otherwise next steps will fail
    second_map.insert("key_1", 1);
    second_map.insert("key_2", 2);
    println!("Accessing the values from HashMap, {:?}", second_map.get("key_1")); //returns Some(1)
    second_map.remove("key_1"); // can delete an item by calling the key
    println!("Accessing the key which is not there: {:?}", second_map.get("key_1")); // No more `KeyError`
    for (key, value) in second_map{
        println!("Key: {} => Value: {}", key, value);
    }
}