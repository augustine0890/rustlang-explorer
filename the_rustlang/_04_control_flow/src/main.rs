mod cl;

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

#[allow(dead_code)]
fn for_loop() {
    for x in 1..11 {
        if x == 3 {continue;}
        if x == 8 {break;}
        println!("x = {}", x);
    }

    for (pos,y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

#[allow(dead_code)]
fn match_statement() {
    let country_code = 44;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid"
    };
    println!("the country with code {} is {}", country_code, country);

    let x = false;
    let y = match x {
        true => "yes",
        _ => "no"
    };
    println!("the answer is {}", y);
}

fn main() {
    // If statement
    // if_statement();

    // While and loop
    // while_and_loop();

    // For loops
    // for_loop();

    // Match Statement
    // match_statement();

    // Combination Look
    cl::combination_lock();

}
