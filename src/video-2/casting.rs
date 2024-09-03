fn main(){
    let f = 1.23_f32;
    let i = f as i8;

    dbg!(i, f);

    let word: u16 = 128;
    let byte = word as i8;
    let ubyte = word as u8;
    dbg!(word, byte, ubyte);

    let too_big = 1000;
    let too_small = too_big as u8;
    dbg!(too_big, too_small);

    dbg!(unchecked(0x0fff_ffff, 0x0fff_ffff));
    dbg!(checked(0x0fff_ffff, 0x0fff_ffff));
}

/// C-like wrapped multiplication
pub fn unchecked(x: i32, y: i32) -> i32{
    x.wrapping_mul(y)
}

/// in debug builds this will panic :confused:
pub fn checked(x: i32, y: i32) -> i32{
    x*y
}