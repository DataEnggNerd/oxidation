
fn main() {
    //integers
    let _x = -1000i32;
    let x: i32 = -1000;
    let _y: u64 = 100;
    let y = 100u64;
    let large_number = 100__00;
    println!("{}, {}, {}", x, y, large_number);

    // floats
    let xf: f32 = -1.2345;
    let xf = -1.2345_f32;
    println!("{}", xf);

    // complex numbers
    // commenting out as there is a extrenal crate dependancy
    // let complex_integer = num::complex::Complex::new(10, 20);
    // let complex_float = num::complex::Complex::new(10.1, 21.1);
    // println!("{}, {}", complex_integer, complex_float);

    // boolean
    let yes = true;
    let no = false;
    println!("{}, {}", yes, no);

    // binary literals
    let x: u8 = 0b1010_1100;
    let y: u8 = 0b0101_0101;
    println!("{}, {}", x, y);
}