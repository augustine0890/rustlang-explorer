use std::mem;

#[allow(dead_code)]
fn core_types() {
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

fn operators() {
    // arithmetic
    let mut a = 2 + 3*4;
    println!("a = {}", a);
    a = a + 1;
    println!("a = {}", a);
    a += 2; // -= += *= /= %=
    println!("a = {}", a);

    println!("The remainder of {} / {} = {}", a, 4, (a%4));

    let a_cubed = i32::pow(a, 3);
    println!("a_cubed = {}", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("b_cubed = {}", b_cubed);
    println!("b_to_pi = {}", b_to_pi);

    // bitwise
    // | OR & AND ^ XOR ! NOR
    let c = 1 | 2; // 01 OR 10 = 11 == 3_10
    println!("c = {}", c);
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    println!("Does PI less than 4.0: {}", pi_less_4);
    // > <= >= ==
    let x = 5;
    let x_is_5 = x == 5; // true
    println!("Does x less than 5: {}", x_is_5);
}

fn main() {
    // Core Data Types
    // core_types();

    operators();
}
