use std::io;

fn main() {
    let name = "Augustine";
    let another_name = "Quy";
    println!("{} and {}", name, another_name);

    let mut nick_name = "Ty";
    println!("{}", nick_name);
    nick_name = "No name";
    println!("{}", nick_name);

    let first = "Augustine".to_string();
    let last = "Nguyen".to_string();
    say_name(first, last);

    let first_name = "Augustine".to_string();
    say_first_name(&first_name);
    say_first_name(&first_name);

    println!("Please enter your name: ");
    let mut your_name = String::new();

    io::stdin().read_line(&mut your_name);
    println!("Hello {}", your_name);
}

fn say_name(first: String, last: String) {
    println!("{} {}", first, last);
}

fn say_first_name(first_name: &String) {
    println!("{}", first_name);
}
