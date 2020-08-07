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

fn main() {
    if_statement();
}
