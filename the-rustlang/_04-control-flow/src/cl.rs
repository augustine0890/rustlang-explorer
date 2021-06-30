#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

//use rand::Rng;
use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked
}

pub fn combination_lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end())
                        // 123
                    }
                    Err(_) => continue
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry) {
                    // "1234".starts_with(125)
                    state = State::Failed;
                }
            }

            State::Failed => {
                println!("FAILED");
                entry.clear(); // ""
                state = State::Locked;
                continue;
            }

            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }

        }
    }
}