use std::io::{self, Read};

fn main() {
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        if c == 'q' {
            println!("Program is exiting...");
            break;
        }
        println!("{}", c);
    }
}
