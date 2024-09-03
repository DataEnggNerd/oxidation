fn main(){
    
    // scope
    // this binding can be accessed within main function
    let long_lived_binding = 1;

    // introducing block with a defined scope of memory
    // By end of the block - "}", rust clears memory
    {
        // this binding is accessible only inside this block
        let short_lived_binding = 2;

        println!("inner short binding: {}", short_lived_binding);

        // this **shadows** the variable outside this block
        let long_lived_binding = 5;

        println!("Shadowed long binding: {}", long_lived_binding);
    }
    // end of the block - Rust compiler cleared memory allocations
    // Trying to access `short_lived_binding` will cause error
    // println!("inner short binding: {}", short_lived_binding);
    /* error[E0425]: cannot find value `short_lived_binding` in this scope
    --> variable_scopes.rs:22:41
    |
    22 |     println!("inner short binding: {}", short_lived_binding);
    |                                         ^^^^^^^^^^^^^^^^^^^ help: a local variable with a similar name exists: `long_lived_binding`
     */

    println!("Long binding, original: {}", long_lived_binding);
    // shodowing withing same block
    let long_lived_binding = "a";

    println!("Long binding, shadowed: {}", long_lived_binding);

}