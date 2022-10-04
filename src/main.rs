use core::str;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let query = &args[1];
    let filename = &args[2];
    // println!("Searching for {}", query);
    // println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents: str::Split<&str> = contents.split("\n");
    for (i, line) in contents.enumerate() {
        if let Some(n) = line.find(query) {
            println!("Text found on line {}, character {}", i + 1, n + 1);
            return ();
        };
    }
    println!("Text not found.");
}
