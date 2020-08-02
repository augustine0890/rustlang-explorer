use std::mem;

fn main() {
    // unsigned 0 + (255)
    let a:u8 = 123; // 8bits
    println!("a = {}", a);

    let mut b:i8 = 0; // mutable
    println!("b = {}", b);
    b = 45;
    println!("b = {}", b);

    let mut c = 123456789; // 32-bit signed i32 (not unsigned)
    println!("c = {}, size = {} bytes.", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    // i8 u8 i16 u16 i32 u32 i64 u64
    let z:isize = 123; // isize//usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS.",
                z, size_of_z, size_of_z * 8);
    
    let d:char = 'x';
    println!("d = {}, size = {} bytes.", d, mem::size_of_val(&d));

    let e:f64 = 2.5; // double-precision, 8 bytes or 64 bits, f64
    println!("e = {}, size = {} bytes.", e, mem::size_of_val(&e));

    // true false
    let g = false;
    println!("g = {}, size = {} bytes.", g, mem::size_of_val(&g));
    let f = 7 > 0;
    println!("f = {}, size = {} bytes.", f, mem::size_of_val(&f));
}
