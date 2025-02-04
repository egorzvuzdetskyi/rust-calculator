use std::io::{stdin, stdout, Write};

pub fn read_input(msg: String) -> String {
    println!("{msg}");
    let mut s = String::new();
    // explanation why: https://users.rust-lang.org/t/why-do-we-need-to-use-flush-when-we-use-print/14701
    stdout().flush().expect("Something failed with terminal");
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    return s.trim().to_string();
}

pub fn read_input_number(msg: String) -> f32 {
    return read_input(msg).parse::<f32>().expect("msg");
}
