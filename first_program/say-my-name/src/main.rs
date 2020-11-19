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
}

fn say_name(first: String, last: String) {
    println!("{} {}", first, last);
}
