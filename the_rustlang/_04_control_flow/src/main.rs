#[allow(dead_code)]
fn if_statement() {
    let mut temp = 25;
    if temp > 30 {
        println!("really hot outside!");
    } else if temp < 10 {
        println!("really cold!");
    } else {
        println!("temperature is OK");
    }

    let day = if temp > 20 {"sunny"} else {"cloundy"};
    println!("today if {}", day);

    println!("it is {}",
        if temp > 30 {"hot"} else if temp < 10 {"cold"} else {"OK"}
    );

    temp = 37;
    println!("it is {}",
        if temp > 30 {
            if temp > 35 {"very hot"} else {"hot"}
        } else if temp < 10 {"cold"} else {"OK"}
    );
}

#[allow(dead_code)]
fn while_and_loop() {
    let mut x = 1;
    while x < 100 {
        x *= 2;
        if x == 64 { continue; }
        println!("x = {}", x);
    }

    let mut y = 1;
    loop { // while true
        y *= 2;
        println!("y = {}", y);
        
        if y == 1<<10 { break; }
    }
}


fn main() {
    // If statement
    // if_statement();

    // While and loop
    while_and_loop();
}
