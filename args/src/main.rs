use std::env;

fn main() {
    println!("Hello, world!");
    let mut index = 0;
    for arg in env::args() {
        println!("Arg:{}\t = {}", index, arg);
        index += 1;
    }
}
