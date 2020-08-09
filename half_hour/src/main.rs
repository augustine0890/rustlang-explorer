#[allow(dead_code)]
fn var_binding() {
    let x = 42; // assign 42 to x
    println!("x = {}", x);
    
    let y:i32 = 42; // `i32` is a signed 32-bit integer
    println!("y = {}", y);

    let _z = 42; // get rid of a compiler warning for now

    let t = 15;
    let t = t + 5; // the first `t` no longer exists -> 20
    println!("t = {}", t);

}

#[allow(dead_code)]
fn tuples() {
    let pair = ('a', 17);
    println!("pair.0 = {}, pair.1 = {}", pair.0, pair.1);

    let pair2: (char, i32) = ('a', 17); // annotate the type of pair
    println!("pair.0 = {}, pair.1 = {}", pair2.0, pair2.1);

    let (some_char, some_int) = ('a', 17);
    println!("some_char = {}, some_int = {}", some_char, some_int);

    let slice = [1, 2, 3, 4, 5];
    let (left, right) = slice.split_at(2);
    println!("left = {:?}, right = {:?}", left, right);

}
#[allow(dead_code)]
fn fair_dice_roll() -> i32 { // `fn` declares a function
    let feeling_lucky= true;
    // if feeling_lucky {
    //     6
    // } else {
    //     4
    // }
    //return 6;
    match feeling_lucky {
        true => 6,
        false => 4,
    }
}

#[allow(dead_code)]
fn scope() {
    let _x = "out";
    {
        // this is a different `x`
        let x = "in";
        println!("x = {}", x);
    }
    print!("x = {}", _x);
}

fn main() {
    println!("===========================");
    println!("Happy Birthday, Augustine!");
    println!("===========================");
    
    // Let's print a cake!
    println!("   ,,,,,   ");
    println!("   |||||   ");
    println!(" =========");
    println!("@@@@@@@@@@@");
    println!(" ~@~@~@~@~");
    println!("@@@@@@@@@@@");
    
    // var_binding();
    // tuples();
    
    let x = fair_dice_roll();
    println!("dice = {}", x);
    
    scope();
}
